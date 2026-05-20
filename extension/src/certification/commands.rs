//! Arma-callable certification commands.
//!
//! `list` and `get_player` fire a callback on `skua:certification/<function>`
//! with a [`QueryOutcome`]:
//! - `[Done, payload]` on success — payload shape per command, JSON-encoded
//!   so SQF can `fromJSON` it.
//! - `[TransientFailure, error]` on failure.
//!
//! `grant` and `revoke` emit state-change events via [`crate::event::emit`] on
//! success (`Event::CertificationGranted` / `Event::CertificationRevoked`).
//! The same emit path is used by `push_player_certs` (cert hydration on
//! `player_connect`) and the live-load watchdog, so SQF subscribers have a
//! single source for "this cert was granted/revoked" regardless of trigger.
//! On failure, `grant` / `revoke` still fire a `skua:certification/<function>`
//! callback so callers can distinguish a failed request from a no-op.

use arma_rs::{Context, Group};
use tokio_postgres::Client;
use tracing::{error, instrument};

use super::types::Certification;
use crate::core::RUNTIME;
use crate::database::get_client;
use crate::domain::PlayerId;
use crate::error::{QueryError, QueryOutcome, QueryState, transient_query_error};
use crate::event::{self, Event};

pub fn group() -> Group {
    Group::new()
        .command("list", list)
        .command("get_player", get_player)
        .command("grant", grant)
        .command("revoke", revoke)
}

// -- callback dispatch helpers --

/// Emits [`Event::CertificationGranted`] on the unified event channel. Reused
/// by the Arma `grant` command, `push_player_certs` (on `player_connect`), and
/// the watchdog (on live grant detection) — one canonical emit site means
/// command and watchdog paths cannot drift in the wire shape they produce.
pub(crate) fn dispatch_grant_event(ctx: &Context, player_id: PlayerId, cert_id: &str) {
    event::emit(
        ctx,
        &Event::CertificationGranted {
            player_id,
            cert_id: cert_id.to_owned(),
        },
    );
}

/// Emits [`Event::CertificationRevoked`] on the unified event channel. Reused
/// by the Arma `revoke` command and the watchdog (on live revoke detection).
pub(crate) fn dispatch_revoke_event(ctx: &Context, player_id: PlayerId, cert_id: &str) {
    event::emit(
        ctx,
        &Event::CertificationRevoked {
            player_id,
            cert_id: cert_id.to_owned(),
        },
    );
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
                error!(error = ?e, "certification:list failed to get database client");
                return;
            }
        };
        push_list(&ctx, &client).await;
    });
    QueryState::Processing
}

/// Queries the certification table and emits [`Event::CertificationListChanged`]
/// with the full list. Reused by [`crate::sync::push_post_bootstrap`] (one-shot
/// push after bootstrap) and [`crate::sync::watchdog`] (when the live-load
/// hash diff detects definition changes) — all three trigger paths emit the
/// same event so SQF subscribers don't have to distinguish.
///
/// Query failures log and drop. The watchdog will retry on its next tick;
/// post-bootstrap retries happen via the bootstrap pipeline.
pub(crate) async fn push_list(ctx: &Context, client: &Client) {
    match list_inner(client).await {
        Ok(rows) => event::emit(ctx, &Event::CertificationListChanged { certs: rows }),
        Err(err) => error!(error = %err, "failed to query certifications for push"),
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

fn grant(
    ctx: Context,
    player_id: PlayerId,
    cert_id: String,
    granter_steam_id: PlayerId,
) -> QueryState {
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
        match grant_inner(&mut client, player_id, &cert_id, granter_steam_id).await {
            Ok(()) => dispatch_grant_event(&ctx, player_id, &cert_id),
            Err(err) => dispatch_failure(&ctx, "grant", err),
        }
    });
    QueryState::Processing
}

#[instrument(level = "debug", name = "certification_grant", skip(client), fields(player_id = %player_id, cert_id = %cert_id, granted_by = %granted_by))]
pub(super) async fn grant_inner(
    client: &mut Client,
    player_id: PlayerId,
    cert_id: &str,
    granted_by: PlayerId,
) -> Result<(), QueryError> {
    let tx = client
        .transaction()
        .await
        .map_err(|e| transient_query_error("Failed to begin grant transaction", e))?;

    // Ensure both grantee and granter have player_info rows so the
    // player_certs (steam_id) and player_certs.granted_by FKs are satisfied.
    // The granter may not have connected yet — admins can grant from outside
    // an active session in principle.
    tx.execute(
        "INSERT INTO skua_master.player_info (steam_id, name)
         VALUES ($1, ''), ($2, '')
         ON CONFLICT (steam_id) DO NOTHING",
        &[&player_id, &granted_by],
    )
    .await
    .map_err(|e| transient_query_error("Failed to ensure player_info rows", e))?;

    tx.execute(
        "INSERT INTO skua_master.player_certs (steam_id, cert_id, granted_by)
         VALUES ($1, $2, $3)
         ON CONFLICT DO NOTHING",
        &[&player_id, &cert_id, &granted_by],
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
// `Event::CertificationGranted` events, one per cert. SQF's
// `fnc_onCertificationGranted` then re-runs each cert's CBA event for the
// player. No DB writes — this is "replay what's stored". Used by
// `database:player_connect` to hydrate a player's certs on join.
//
// Returns the cert id list so the caller can seed the watchdog state map
// (preventing the watchdog from re-firing these same grants on its next tick).
//
// Caller provides the `tokio_postgres::Client` so the same connection used for
// the surrounding upsert can be reused. On query failure, a single failure
// callback fires on the legacy `skua:certification/grant` channel (caller-ack
// for the rare error case); an empty list is returned.

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
