//! Arma-callable loadout commands.
//!
//! `get_player(campaign_id, player_id)` fires a callback on
//! `skua:loadout/get_player`:
//! - `[Done, [<player_id>, <loadout array>]]` when a per-player row or
//!   campaign default exists.
//! - `[Done, [<player_id>, []]]` when neither is set — the caller should
//!   leave the unit's current loadout alone. (Empty-array sentinel rather
//!   than `null`, so `parseSimpleArray` accepts the wire string.)
//! - `[TransientFailure, error]` on any DB / parse failure.
//!
//! The player id round-trips in the payload so the SQF callback can route
//! the loadout back to the right unit (`BIS_fnc_getUnitByUID`) without
//! tracking pending requests itself.
//!
//! `set_player(campaign_id, player_id, loadout)` and
//! `set_default(campaign_id, loadout)` mirror `certification::grant`'s
//! failure-only ack pattern: a callback fires on `skua:loadout/<fn>` only when
//! the write fails. Success is silent — there is no SQF subscriber that needs
//! confirmation.
//!
//! `campaign_id` is the raw (unsanitized) campaign key the SQF side already
//! holds in `skua_database_campaignKey`; we sanitize via
//! [`crate::database::sanitize_key`] so the schema-name interpolation in the
//! storage layer stays safe.

use arma_rs::loadout::Loadout;
use arma_rs::{Context, Group, IntoArma, Value};
use tracing::{error, instrument};

use super::storage;
use crate::core::RUNTIME;
use crate::database::{get_client, sanitize_key};
use crate::domain::PlayerId;
use crate::error::{QueryError, QueryOutcome, QueryState, transient_query_error};

pub fn group() -> Group {
    Group::new()
        .command("get_player", get_player)
        .command("set_player", set_player)
        .command("set_default", set_default)
}

/// Wire wrapper for the `get_player` response. Carries the queried player id
/// so the SQF callback can look up the unit. `Value::Null` would render as
/// `null` and break `parseSimpleArray`, so a missing loadout is emitted as
/// the empty-array sentinel `[]`.
struct LoadoutResponse {
    player_id: PlayerId,
    loadout: Option<Loadout>,
}

impl IntoArma for LoadoutResponse {
    fn to_arma(&self) -> Value {
        let loadout_value = match &self.loadout {
            Some(loadout) => loadout.to_arma(),
            None => Value::Array(Vec::new()),
        };
        Value::Array(vec![self.player_id.to_arma(), loadout_value])
    }
}

fn dispatch_failure(ctx: &Context, function: &'static str, err: QueryError) {
    let outcome: QueryOutcome<String> = QueryOutcome::Failed(err);
    if let Err(e) = ctx.callback_data("skua:loadout", function, outcome) {
        error!(error = ?e, function, "failed to dispatch loadout failure callback");
    }
}

fn invalid_campaign(reason: &'static str) -> QueryError {
    QueryError {
        code: "INVALID_ARGUMENT".to_string(),
        message: format!("invalid campaign_id: {reason}"),
        location: format!("{}:{}", file!(), line!()),
    }
}

// -- get_player --

#[instrument(level = "trace", name = "loadout_cmd_get_player", skip(ctx))]
#[allow(clippy::needless_pass_by_value)] // (Arma commands must take owned args.)
fn get_player(ctx: Context, campaign_id: String, player_id: PlayerId) -> QueryState {
    let cid = match sanitize_key(&campaign_id) {
        Ok(c) => c,
        Err(reason) => {
            dispatch_failure(&ctx, "get_player", invalid_campaign(reason));
            return QueryState::InvalidArgument;
        }
    };

    RUNTIME.spawn(async move {
        let outcome = run_get_player(&cid, player_id).await;
        if let Err(e) = ctx.callback_data("skua:loadout", "get_player", outcome) {
            error!(error = ?e, "failed to dispatch loadout:get_player callback");
        }
    });
    QueryState::Processing
}

async fn run_get_player(campaign_id: &str, player_id: PlayerId) -> QueryOutcome<LoadoutResponse> {
    let client = match get_client().await {
        Ok(c) => c,
        Err(e) => {
            return QueryOutcome::Failed(transient_query_error("Failed to get database client", e));
        }
    };
    match storage::get_player_inner(&client, campaign_id, player_id).await {
        Ok(loadout) => QueryOutcome::Done(LoadoutResponse { player_id, loadout }),
        Err(err) => QueryOutcome::Failed(err),
    }
}

// -- set_player --

#[instrument(level = "trace", name = "loadout_cmd_set_player", skip(ctx, loadout))]
#[allow(clippy::needless_pass_by_value)] // (Arma commands must take owned args.)
fn set_player(
    ctx: Context,
    campaign_id: String,
    player_id: PlayerId,
    loadout: Loadout,
) -> QueryState {
    let cid = match sanitize_key(&campaign_id) {
        Ok(c) => c,
        Err(reason) => {
            dispatch_failure(&ctx, "set_player", invalid_campaign(reason));
            return QueryState::InvalidArgument;
        }
    };

    RUNTIME.spawn(async move {
        let mut client = match get_client().await {
            Ok(c) => c,
            Err(e) => {
                dispatch_failure(
                    &ctx,
                    "set_player",
                    transient_query_error("Failed to get database client", e),
                );
                return;
            }
        };
        if let Err(err) = storage::set_player_inner(&mut client, &cid, player_id, &loadout).await {
            dispatch_failure(&ctx, "set_player", err);
        }
    });
    QueryState::Processing
}

// -- set_default --

#[instrument(level = "trace", name = "loadout_cmd_set_default", skip(ctx, loadout))]
#[allow(clippy::needless_pass_by_value)] // (Arma commands must take owned args.)
fn set_default(ctx: Context, campaign_id: String, loadout: Loadout) -> QueryState {
    let cid = match sanitize_key(&campaign_id) {
        Ok(c) => c,
        Err(reason) => {
            dispatch_failure(&ctx, "set_default", invalid_campaign(reason));
            return QueryState::InvalidArgument;
        }
    };

    RUNTIME.spawn(async move {
        let client = match get_client().await {
            Ok(c) => c,
            Err(e) => {
                dispatch_failure(
                    &ctx,
                    "set_default",
                    transient_query_error("Failed to get database client", e),
                );
                return;
            }
        };
        if let Err(err) = storage::set_default_inner(&client, &cid, &loadout).await {
            dispatch_failure(&ctx, "set_default", err);
        }
    });
    QueryState::Processing
}
