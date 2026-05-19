//! Arma-callable rank commands.
//!
//! - `list`: queries the rank set and emits [`Event::RankListChanged`]. Used by
//!   the post-bootstrap push; SQF subscribes to the event rather than waiting
//!   on a per-call reply.
//! - `get_player`: returns the player's rank id via `skua:ranks/get_player`
//!   callback. Caller-ack style; no event (the rank hasn't changed, only the
//!   caller's view of it).
//! - `set_player`: mutates the rank and emits [`Event::RankChanged`] on
//!   success. On failure, fires a `skua:ranks/set_player` callback with the
//!   error so the caller can distinguish "request failed" from "rank set to a
//!   value that happens to equal the old one".

use arma_rs::{Context, Group};
use tokio_postgres::Client;
use tracing::{error, instrument};

use super::types::Rank;
use crate::core::RUNTIME;
use crate::database::get_client;
use crate::domain::PlayerId;
use crate::error::{QueryError, QueryOutcome, QueryState, transient_query_error};
use crate::event::{self, Event};

pub fn group() -> Group {
    Group::new()
        .command("list", list)
        .command("get_player", get_player)
        .command("set_player", set_player)
}

// -- list --

fn list(ctx: Context) -> QueryState {
    RUNTIME.spawn(async move {
        let client = match get_client().await {
            Ok(c) => c,
            Err(e) => {
                error!(error = ?e, "ranks:list failed to get database client");
                return;
            }
        };
        push_list(&ctx, &client).await;
    });
    QueryState::Processing
}

/// Queries the ranks table and emits [`Event::RankListChanged`] with the full
/// list. Reused by [`crate::sync::push_post_bootstrap`] so the same emit path
/// serves ad-hoc SQF requests and post-bootstrap pushes.
pub(crate) async fn push_list(ctx: &Context, client: &Client) {
    match list_inner(client).await {
        Ok(rows) => event::emit(ctx, &Event::RankListChanged { ranks: rows }),
        Err(err) => error!(error = %err, "failed to query ranks for push"),
    }
}

#[instrument(level = "debug", name = "ranks_list", skip_all)]
pub(super) async fn list_inner(client: &Client) -> Result<Vec<Rank>, QueryError> {
    let rows = client
        .query(
            "SELECT id, display_name FROM skua_master.ranks ORDER BY id",
            &[],
        )
        .await
        .map_err(|e| transient_query_error("Failed to query ranks", e))?;

    Ok(rows
        .iter()
        .map(|row| Rank {
            id: row.get("id"),
            display_name: row.get("display_name"),
        })
        .collect())
}

// -- get_player --

fn get_player(ctx: Context, player_id: PlayerId) -> QueryState {
    RUNTIME.spawn(async move {
        let outcome = run_get_player(player_id).await;
        if let Err(e) = ctx.callback_data("skua:ranks", "get_player", outcome) {
            error!(error = ?e, "failed to dispatch ranks:get_player callback");
        }
    });
    QueryState::Processing
}

async fn run_get_player(player_id: PlayerId) -> QueryOutcome<i16> {
    let client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            return QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
        }
    };
    match get_player_inner(&client, player_id).await {
        Ok(rank) => QueryOutcome::Done(rank),
        Err(err) => QueryOutcome::Failed(err),
    }
}

/// Returns the player's rank id, or `0` (default rank) if the player has no
/// `player_info` row yet.
#[instrument(level = "debug", name = "ranks_get_player", skip(client), fields(player_id = %player_id))]
pub(super) async fn get_player_inner(
    client: &Client,
    player_id: PlayerId,
) -> Result<i16, QueryError> {
    let row = client
        .query_opt(
            "SELECT rank FROM skua_master.player_info WHERE steam_id = $1",
            &[&player_id],
        )
        .await
        .map_err(|e| transient_query_error("Failed to query player rank", e))?;

    Ok(row.map_or(0, |r| r.get::<_, i16>("rank")))
}

// -- set_player --

fn set_player(ctx: Context, player_id: PlayerId, rank_id: i16) -> QueryState {
    RUNTIME.spawn(async move {
        match run_set_player(player_id, rank_id).await {
            Ok(()) => event::emit(&ctx, &Event::RankChanged { player_id, rank_id }),
            Err(err) => {
                let outcome: QueryOutcome<Vec<String>> = QueryOutcome::Failed(err);
                if let Err(e) = ctx.callback_data("skua:ranks", "set_player", outcome) {
                    error!(error = ?e, "failed to dispatch ranks:set_player failure");
                }
            }
        }
    });
    QueryState::Processing
}

async fn run_set_player(player_id: PlayerId, rank_id: i16) -> Result<(), QueryError> {
    let mut client = get_client()
        .await
        .map_err(|e| transient_query_error("Failed to get database client", e))?;
    set_player_inner(&mut client, player_id, rank_id).await
}

/// Auto-creates the `player_info` row if absent (with `name = ''`, `rank =
/// rank_id`), or updates the rank on an existing row.
#[instrument(level = "debug", name = "ranks_set_player", skip(client), fields(player_id = %player_id, rank_id))]
pub(super) async fn set_player_inner(
    client: &mut Client,
    player_id: PlayerId,
    rank_id: i16,
) -> Result<(), QueryError> {
    let tx = client
        .transaction()
        .await
        .map_err(|e| transient_query_error("Failed to begin set_player transaction", e))?;

    tx.execute(
        "INSERT INTO skua_master.player_info (steam_id, name, rank)
         VALUES ($1, '', $2)
         ON CONFLICT (steam_id) DO UPDATE
             SET rank = EXCLUDED.rank",
        &[&player_id, &rank_id],
    )
    .await
    .map_err(|e| transient_query_error("Failed to set player rank", e))?;

    tx.commit()
        .await
        .map_err(|e| transient_query_error("Failed to commit set_player transaction", e))?;

    Ok(())
}
