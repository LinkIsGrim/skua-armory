//! Arma-callable certification commands.
//!
//! Each command returns `Processing` synchronously and fires a callback on
//! `skua:certification / <function>` with a [`QueryOutcome`]:
//! - `[Done, payload]` on success (`list` = `Vec<Certification>`,
//!   `get_player` = `Vec<String>`, `grant`/`revoke` = empty `[]`)
//! - `[TransientFailure, error]` on failure.
//!
//! Each command has a thin Arma entry (`list`/`get_player`/...) and a testable
//! inner function (`*_inner`) that takes a raw `tokio_postgres::Client` so the
//! SQL paths can be exercised in integration tests without touching the global
//! `OnceCell`-backed pool.

use arma_rs::{Context, Group};
use tokio_postgres::Client;
use tracing::{error, instrument};

use super::types::Certification;
use crate::core::RUNTIME;
use crate::database::get_client;
use crate::domain::PlayerId;
use crate::error::{QueryError, QueryOutcome, QueryState, transient_query_error};

pub fn group() -> Group {
    Group::new()
        .command("list", list)
        .command("get_player", get_player)
        .command("grant", grant)
        .command("revoke", revoke)
}

// -- list --

fn list(ctx: Context) -> QueryState {
    RUNTIME.spawn(async move {
        let outcome = run_list().await;
        if let Err(e) = ctx.callback_data("skua:certification", "list", outcome) {
            error!(error = ?e, "failed to dispatch certification:list callback");
        }
    });
    QueryState::Processing
}

async fn run_list() -> QueryOutcome<Vec<Certification>> {
    let client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            return QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
        }
    };
    match list_inner(&client).await {
        Ok(rows) => QueryOutcome::Done(rows),
        Err(err) => QueryOutcome::Failed(err),
    }
}

#[instrument(level = "debug", name = "certification_list", skip_all)]
pub(super) async fn list_inner(client: &Client) -> Result<Vec<Certification>, QueryError> {
    let rows = client
        .query(
            "SELECT id, display_name, document, grant_event, revoke_event
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
            grant_event: row.get("grant_event"),
            revoke_event: row.get("revoke_event"),
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
pub(super) async fn get_player_inner(
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
        let outcome = run_grant(player_id, &cert_id).await;
        if let Err(e) = ctx.callback_data("skua:certification", "grant", outcome) {
            error!(error = ?e, "failed to dispatch certification:grant callback");
        }
    });
    QueryState::Processing
}

async fn run_grant(player_id: PlayerId, cert_id: &str) -> QueryOutcome<Vec<String>> {
    let mut client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            return QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
        }
    };
    match grant_inner(&mut client, player_id, cert_id).await {
        Ok(()) => QueryOutcome::Done(Vec::new()),
        Err(err) => QueryOutcome::Failed(err),
    }
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
        let outcome = run_revoke(player_id, &cert_id).await;
        if let Err(e) = ctx.callback_data("skua:certification", "revoke", outcome) {
            error!(error = ?e, "failed to dispatch certification:revoke callback");
        }
    });
    QueryState::Processing
}

async fn run_revoke(player_id: PlayerId, cert_id: &str) -> QueryOutcome<Vec<String>> {
    let client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            return QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
        }
    };
    match revoke_inner(&client, player_id, cert_id).await {
        Ok(()) => QueryOutcome::Done(Vec::new()),
        Err(err) => QueryOutcome::Failed(err),
    }
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
