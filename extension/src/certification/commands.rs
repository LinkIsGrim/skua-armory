//! Arma-callable certification commands.
//!
//! Each command returns `Processing` synchronously and fires a callback on
//! `skua:certification / <function>` with a [`QueryOutcome`]:
//! - `[Done, payload]` on success — payload shape per command, JSON-encoded
//!   so SQF can `fromJSON` it.
//! - `[TransientFailure, error]` on failure.
//!
//! `grant` and `revoke` both ship a [`PlayerCertEvent`] payload (`player_id` +
//! `cert_id`). Same shape is reused by `push_player_certs` (cert hydration on
//! `player_connect`) and the live-load watchdog so SQF has one handler per
//! direction regardless of source.

use arma_rs::{Context, Group};
use serde::Serialize;
use tokio_postgres::Client;
use tracing::{error, instrument};

use super::types::Certification;
use crate::core::RUNTIME;
use crate::database::get_client;
use crate::domain::PlayerId;
use crate::error::{QueryError, QueryOutcome, QueryState, transient_query_error};

/// JSON payload for `grant` / `revoke` callbacks. Field names match what
/// `addons/certifications/functions/fnc_onGrantReturn.sqf` (and the parallel
/// revoke handler) read.
#[derive(Serialize)]
pub(super) struct PlayerCertEvent<'a> {
    pub(super) player_id: PlayerId,
    pub(super) cert_id: &'a str,
}

pub fn group() -> Group {
    Group::new()
        .command("list", list)
        .command("get_player", get_player)
        .command("grant", grant)
        .command("revoke", revoke)
}

// -- callback dispatch helpers --

/// Fires a `skua:certification/grant` callback with the `[player_id, cert_id]`
/// payload SQF expects. Reused by the Arma `grant` command, `push_player_certs`
/// (on `player_connect`), and the watchdog (on live grant detection).
pub(crate) fn dispatch_grant_event(ctx: &Context, player_id: PlayerId, cert_id: &str) {
    dispatch_cert_event(ctx, "grant", player_id, cert_id);
}

/// Fires a `skua:certification/revoke` callback with the `[player_id, cert_id]`
/// payload SQF expects. Reused by the Arma `revoke` command and the watchdog
/// (on live revoke detection).
pub(crate) fn dispatch_revoke_event(ctx: &Context, player_id: PlayerId, cert_id: &str) {
    dispatch_cert_event(ctx, "revoke", player_id, cert_id);
}

fn dispatch_cert_event(ctx: &Context, function: &'static str, player_id: PlayerId, cert_id: &str) {
    let payload = match serde_json::to_string(&PlayerCertEvent { player_id, cert_id }) {
        Ok(p) => p,
        Err(e) => {
            error!(error = ?e, function, %cert_id, "failed to serialize cert event");
            return;
        }
    };
    let outcome: QueryOutcome<String> = QueryOutcome::Done(payload);
    if let Err(e) = ctx.callback_data("skua:certification", function, outcome) {
        error!(error = ?e, function, %cert_id, "failed to dispatch cert event callback");
    }
}

fn dispatch_failure(ctx: &Context, function: &'static str, err: QueryError) {
    let outcome: QueryOutcome<String> = QueryOutcome::Failed(err);
    if let Err(e) = ctx.callback_data("skua:certification", function, outcome) {
        error!(error = ?e, function, "failed to dispatch failure callback");
    }
}

// -- list --

fn list(ctx: Context) -> QueryState {
    RUNTIME.spawn(async move {
        let client = match get_client().await {
            Ok(c) => c,
            Err(e) => {
                let outcome: QueryOutcome<String> =
                    QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
                if let Err(e) = ctx.callback_data("skua:certification", "list", outcome) {
                    error!(error = ?e, "failed to dispatch certification:list callback");
                }
                return;
            }
        };
        push_list(&ctx, &client).await;
    });
    QueryState::Processing
}

/// Queries the certification table and fires `skua:certification/list` with
/// the JSON-encoded list (or a transient failure on query/serialize error).
///
/// Reused by [`crate::sync::push_post_bootstrap`] so the same dispatch path
/// serves both ad-hoc SQF requests and post-bootstrap pushes.
pub(crate) async fn push_list(ctx: &Context, client: &Client) {
    let outcome: QueryOutcome<String> = match list_inner(client).await {
        Ok(rows) => match serde_json::to_string(&rows) {
            Ok(json) => QueryOutcome::Done(json),
            Err(e) => QueryOutcome::Failed(transient_query_error(
                "Failed to serialize certifications",
                e,
            )),
        },
        Err(err) => QueryOutcome::Failed(err),
    };
    if let Err(e) = ctx.callback_data("skua:certification", "list", outcome) {
        error!(error = ?e, "failed to dispatch certification:list callback");
    }
}

#[instrument(level = "debug", name = "certification_list", skip_all)]
pub(crate) async fn list_inner(client: &Client) -> Result<Vec<Certification>, QueryError> {
    let rows = client
        .query(
            "SELECT id, display_name, document, description, perk, pay_bonus,
                    grant_event, revoke_event, requires
             FROM skua_master.certifications
             ORDER BY id",
            &[],
        )
        .await
        .map_err(|e| transient_query_error("Failed to query certifications", e))?;

    Ok(rows
        .iter()
        .map(|row| Certification {
            id: row.get("id"),
            display_name: row.get("display_name"),
            document: row.get("document"),
            description: row.get("description"),
            perk: row.get("perk"),
            pay_bonus: row.get("pay_bonus"),
            grant_event: row.get("grant_event"),
            revoke_event: row.get("revoke_event"),
            requires: row.get("requires"),
        })
        .collect())
}

// -- get_player --

fn get_player(ctx: Context, player_id: PlayerId) -> QueryState {
    RUNTIME.spawn(async move {
        let outcome = run_get_player(player_id).await;
        if let Err(e) = ctx.callback_data("skua:certification", "get_player", outcome) {
            error!(error = ?e, "failed to dispatch certification:get_player callback");
        }
    });
    QueryState::Processing
}

async fn run_get_player(player_id: PlayerId) -> QueryOutcome<Vec<String>> {
    let client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            return QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
        }
    };
    match get_player_inner(&client, player_id).await {
        Ok(rows) => QueryOutcome::Done(rows),
        Err(err) => QueryOutcome::Failed(err),
    }
}

#[instrument(level = "debug", name = "certification_get_player", skip(client), fields(player_id = %player_id))]
pub(crate) async fn get_player_inner(
    client: &Client,
    player_id: PlayerId,
) -> Result<Vec<String>, QueryError> {
    let rows = client
        .query(
            "SELECT cert_id FROM skua_master.player_certs
             WHERE steam_id = $1
             ORDER BY cert_id",
            &[&player_id],
        )
        .await
        .map_err(|e| transient_query_error("Failed to query player certifications", e))?;

    Ok(rows
        .iter()
        .map(|row| row.get::<_, String>("cert_id"))
        .collect())
}

// -- grant --

fn grant(ctx: Context, player_id: PlayerId, cert_id: String) -> QueryState {
    RUNTIME.spawn(async move {
        let mut client = match get_client().await {
            Ok(c) => c,
            Err(e) => {
                dispatch_failure(
                    &ctx,
                    "grant",
                    transient_query_error("Failed to get database client", e),
                );
                return;
            }
        };
        match grant_inner(&mut client, player_id, &cert_id).await {
            Ok(()) => dispatch_grant_event(&ctx, player_id, &cert_id),
            Err(err) => dispatch_failure(&ctx, "grant", err),
        }
    });
    QueryState::Processing
}

#[instrument(level = "debug", name = "certification_grant", skip(client), fields(player_id = %player_id, cert_id = %cert_id))]
pub(super) async fn grant_inner(
    client: &mut Client,
    player_id: PlayerId,
    cert_id: &str,
) -> Result<(), QueryError> {
    let tx = client
        .transaction()
        .await
        .map_err(|e| transient_query_error("Failed to begin grant transaction", e))?;

    tx.execute(
        "INSERT INTO skua_master.player_info (steam_id, name)
         VALUES ($1, '')
         ON CONFLICT (steam_id) DO NOTHING",
        &[&player_id],
    )
    .await
    .map_err(|e| transient_query_error("Failed to ensure player_info row", e))?;

    tx.execute(
        "INSERT INTO skua_master.player_certs (steam_id, cert_id)
         VALUES ($1, $2)
         ON CONFLICT DO NOTHING",
        &[&player_id, &cert_id],
    )
    .await
    .map_err(|e| transient_query_error("Failed to grant certification", e))?;

    tx.commit()
        .await
        .map_err(|e| transient_query_error("Failed to commit grant transaction", e))?;

    Ok(())
}

// -- revoke --

fn revoke(ctx: Context, player_id: PlayerId, cert_id: String) -> QueryState {
    RUNTIME.spawn(async move {
        let client = match get_client().await {
            Ok(c) => c,
            Err(e) => {
                dispatch_failure(
                    &ctx,
                    "revoke",
                    transient_query_error("Failed to get database client", e),
                );
                return;
            }
        };
        match revoke_inner(&client, player_id, &cert_id).await {
            Ok(()) => dispatch_revoke_event(&ctx, player_id, &cert_id),
            Err(err) => dispatch_failure(&ctx, "revoke", err),
        }
    });
    QueryState::Processing
}

#[instrument(level = "debug", name = "certification_revoke", skip(client), fields(player_id = %player_id, cert_id = %cert_id))]
pub(super) async fn revoke_inner(
    client: &Client,
    player_id: PlayerId,
    cert_id: &str,
) -> Result<(), QueryError> {
    client
        .execute(
            "DELETE FROM skua_master.player_certs
             WHERE steam_id = $1 AND cert_id = $2",
            &[&player_id, &cert_id],
        )
        .await
        .map_err(|e| transient_query_error("Failed to revoke certification", e))?;

    Ok(())
}

// -- push_player_certs --
//
// Re-emits the DB's stored grants for a player as a stream of
// `skua:certification / grant` callbacks, one per cert. SQF's
// `fnc_onGrantReturn` then re-runs each cert's CBA event for the player. No
// DB writes — this is "replay what's stored". Used by
// `database:player_connect` to hydrate a player's certs on join.
//
// Returns the cert id list so the caller can seed the watchdog state map
// (preventing the watchdog from re-firing these same grants on its next tick).
//
// Caller provides the `tokio_postgres::Client` so the same connection used for
// the surrounding upsert can be reused. On query failure, a single failure
// callback fires on the `grant` channel so the existing SQF error branch picks
// it up; an empty list is returned.

#[instrument(level = "debug", name = "certification_push_player_certs", skip(ctx, client), fields(player_id = %player_id))]
pub(crate) async fn push_player_certs(
    ctx: &Context,
    client: &Client,
    player_id: PlayerId,
) -> Vec<String> {
    let cert_ids = match get_player_inner(client, player_id).await {
        Ok(ids) => ids,
        Err(err) => {
            dispatch_failure(ctx, "grant", err);
            return Vec::new();
        }
    };

    for cert_id in &cert_ids {
        dispatch_grant_event(ctx, player_id, cert_id);
    }
    cert_ids
}
