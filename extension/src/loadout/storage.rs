//! DB-facing storage helpers for loadout persistence.
//!
//! All loadout strings are validated by round-tripping through
//! [`arma_rs::Loadout`] before write and after read, so a corrupt row surfaces
//! as a [`QueryError`] instead of silently overwriting a player's kit with
//! garbage.

use arma_rs::Value;
use arma_rs::loadout::Loadout;
use arma_rs::{FromArma, IntoArma};
use tokio_postgres::Client;
use tracing::instrument;

use crate::domain::PlayerId;
use crate::error::{QueryError, transient_query_error};

/// Serialize a parsed [`Loadout`] into its canonical Arma array string for
/// storage. This is the same form `CBA_fnc_setLoadout` consumes.
pub(super) fn serialize(loadout: &Loadout) -> String {
    loadout.to_arma().to_string()
}

/// Parse a stored loadout string back into a [`Loadout`]. A parse failure here
/// means a stored row is corrupt — surface it instead of silently dropping.
fn parse_stored(raw: &str) -> Result<Loadout, QueryError> {
    Loadout::from_arma(raw.to_owned()).map_err(|e| QueryError {
        code: "INVALID_DATA".to_string(),
        message: format!("stored loadout failed to parse: {e:?}"),
        location: format!("{}:{}", file!(), line!()),
    })
}

/// Fetch the effective loadout for a player: per-player row if present,
/// otherwise the campaign's `default_loadout`. Returns `None` if both are
/// NULL or the campaign row is missing.
#[instrument(level = "debug", name = "loadout_get_player", skip(client), fields(player_id = %player_id, campaign_id = %campaign_id))]
pub(super) async fn get_player_inner(
    client: &Client,
    campaign_id: &str,
    player_id: PlayerId,
) -> Result<Option<Loadout>, QueryError> {
    let registered = format!("skua_campaign_{campaign_id}");
    let sql = format!(
        r#"SELECT COALESCE(p.loadout, c.default_loadout) AS loadout
           FROM skua_master.campaigns c
           LEFT JOIN "skua_campaign_{campaign_id}".player_data p
             ON p.steam_id = $1
           WHERE c.campaign_id = $2"#
    );

    let row = client
        .query_opt(sql.as_str(), &[&player_id, &registered])
        .await
        .map_err(|e| transient_query_error("Failed to query player loadout", e))?;

    let Some(row) = row else {
        return Ok(None);
    };
    let raw: Option<String> = row.get("loadout");
    raw.map(|s| parse_stored(&s)).transpose()
}

/// Upsert the player's loadout row. The `player_info` FK-stub mirrors
/// `certification::grant_inner`: defensive only, because `player_connect`
/// upserts the player row first in practice.
#[instrument(level = "debug", name = "loadout_set_player", skip(client, loadout), fields(player_id = %player_id, campaign_id = %campaign_id))]
pub(super) async fn set_player_inner(
    client: &mut Client,
    campaign_id: &str,
    player_id: PlayerId,
    loadout: &Loadout,
) -> Result<(), QueryError> {
    let serialized = serialize(loadout);

    let tx = client
        .transaction()
        .await
        .map_err(|e| transient_query_error("Failed to begin loadout transaction", e))?;

    tx.execute(
        "INSERT INTO skua_master.player_info (steam_id, name)
         VALUES ($1, '')
         ON CONFLICT (steam_id) DO NOTHING",
        &[&player_id],
    )
    .await
    .map_err(|e| transient_query_error("Failed to ensure player_info row", e))?;

    let sql = format!(
        r#"INSERT INTO "skua_campaign_{campaign_id}".player_data (steam_id, loadout)
           VALUES ($1, $2)
           ON CONFLICT (steam_id) DO UPDATE
               SET loadout = EXCLUDED.loadout,
                   last_updated = NOW()"#
    );
    tx.execute(sql.as_str(), &[&player_id, &serialized])
        .await
        .map_err(|e| transient_query_error("Failed to upsert player loadout", e))?;

    tx.commit()
        .await
        .map_err(|e| transient_query_error("Failed to commit loadout transaction", e))?;

    Ok(())
}

/// Set the campaign's default loadout. The campaign row must already exist
/// (bootstrap creates it); otherwise this returns an error rather than
/// silently no-op'ing.
#[instrument(level = "debug", name = "loadout_set_default", skip(client, loadout), fields(campaign_id = %campaign_id))]
pub(super) async fn set_default_inner(
    client: &Client,
    campaign_id: &str,
    loadout: &Loadout,
) -> Result<(), QueryError> {
    let serialized = serialize(loadout);
    let registered = format!("skua_campaign_{campaign_id}");

    let rows = client
        .execute(
            "UPDATE skua_master.campaigns
             SET default_loadout = $1
             WHERE campaign_id = $2",
            &[&serialized, &registered],
        )
        .await
        .map_err(|e| transient_query_error("Failed to update campaign default loadout", e))?;

    if rows == 0 {
        return Err(QueryError {
            code: "NOT_FOUND".to_string(),
            message: format!("campaign {registered} not registered; bootstrap it first"),
            location: format!("{}:{}", file!(), line!()),
        });
    }
    Ok(())
}
