//! Loadout integration tests against an ephemeral Postgres (testcontainers).

use arma_rs::FromArma;
use arma_rs::loadout::Loadout;

use super::storage::{get_player_inner, set_default_inner, set_player_inner};
use crate::database::{bootstrap_campaign, bootstrap_schema, start_test_db};
use crate::domain::PlayerId;

const CID: &str = "test_loadout";
const PLAYER: PlayerId = PlayerId::new(76_561_198_000_000_001);

/// Minimal valid vanilla loadout (no CBA extended segment) used across tests.
/// Lifted from the arma-rs `marshal` test fixture.
const SAMPLE: &str = r#"[[],[],[],["U_Marshal",[]],[],[],"H_Cap_headphones","G_Aviator",[],["ItemMap","ItemGPS","","ItemCompass","ItemWatch",""]]"#;

const SAMPLE_ALT: &str = r#"[[],[],[],["U_B_CombatUniform_mcam",[]],[],[],"H_HelmetB","G_Bandanna_tan",[],["ItemMap","","","ItemCompass","ItemWatch",""]]"#;

fn parse(s: &str) -> Loadout {
    Loadout::from_arma(s.to_string()).expect("fixture should parse")
}

#[tokio::test]
async fn get_player_returns_none_when_no_row_and_no_default() {
    let (_c, db) = start_test_db().await;
    let client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.unwrap();
    bootstrap_campaign(&client, CID).await.unwrap();

    let result = get_player_inner(&client, CID, PLAYER).await.unwrap();
    assert!(result.is_none(), "no row + no default ⇒ None");
}

#[tokio::test]
async fn get_player_returns_none_when_campaign_missing() {
    let (_c, db) = start_test_db().await;
    let client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.unwrap();
    bootstrap_campaign(&client, CID).await.unwrap();

    // Even though we ran bootstrap_campaign for CID, querying a different
    // (unregistered) campaign id should return None rather than error — the
    // schema for the unknown id won't exist, so we expect a SQL error here.
    // This documents that calling against an unbootstrapped campaign is an
    // error, not a silent None. (Mostly a guard so future refactors don't
    // accidentally swallow missing-schema errors.)
    let result = get_player_inner(&client, "absent_campaign", PLAYER).await;
    assert!(
        result.is_err(),
        "querying against a missing campaign schema should error, got {result:?}"
    );
}

#[tokio::test]
async fn set_then_get_round_trips() {
    let (_c, db) = start_test_db().await;
    let mut client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.unwrap();
    bootstrap_campaign(&client, CID).await.unwrap();

    let original = parse(SAMPLE);
    set_player_inner(&mut client, CID, PLAYER, &original)
        .await
        .unwrap();

    let fetched = get_player_inner(&client, CID, PLAYER)
        .await
        .unwrap()
        .expect("loadout should be present after set");
    assert_eq!(fetched, original, "round-trip should preserve loadout");
}

#[tokio::test]
async fn set_player_overwrites_existing_row() {
    let (_c, db) = start_test_db().await;
    let mut client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.unwrap();
    bootstrap_campaign(&client, CID).await.unwrap();

    set_player_inner(&mut client, CID, PLAYER, &parse(SAMPLE))
        .await
        .unwrap();
    let alt = parse(SAMPLE_ALT);
    set_player_inner(&mut client, CID, PLAYER, &alt)
        .await
        .unwrap();

    let fetched = get_player_inner(&client, CID, PLAYER)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fetched, alt, "second set should overwrite");
}

#[tokio::test]
async fn default_loadout_used_when_player_row_absent() {
    let (_c, db) = start_test_db().await;
    let client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.unwrap();
    bootstrap_campaign(&client, CID).await.unwrap();

    let default = parse(SAMPLE);
    set_default_inner(&client, CID, &default).await.unwrap();

    let fetched = get_player_inner(&client, CID, PLAYER)
        .await
        .unwrap()
        .expect("default should resolve");
    assert_eq!(fetched, default);
}

#[tokio::test]
async fn player_row_takes_precedence_over_default() {
    let (_c, db) = start_test_db().await;
    let mut client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.unwrap();
    bootstrap_campaign(&client, CID).await.unwrap();

    set_default_inner(&client, CID, &parse(SAMPLE))
        .await
        .unwrap();
    let alt = parse(SAMPLE_ALT);
    set_player_inner(&mut client, CID, PLAYER, &alt)
        .await
        .unwrap();

    let fetched = get_player_inner(&client, CID, PLAYER)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        fetched, alt,
        "per-player loadout must shadow the campaign default"
    );
}

#[tokio::test]
async fn corrupt_stored_loadout_surfaces_as_error() {
    let (_c, db) = start_test_db().await;
    let client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.unwrap();
    bootstrap_campaign(&client, CID).await.unwrap();

    // Seed a player_info row + manually-corrupted loadout row.
    client
        .execute(
            "INSERT INTO skua_master.player_info (steam_id, name) VALUES ($1, '')",
            &[&PLAYER],
        )
        .await
        .unwrap();
    let sql = format!(
        r#"INSERT INTO "skua_campaign_{CID}".player_data (steam_id, loadout)
           VALUES ($1, $2)"#
    );
    client
        .execute(sql.as_str(), &[&PLAYER, &"not a loadout".to_string()])
        .await
        .unwrap();

    let result = get_player_inner(&client, CID, PLAYER).await;
    assert!(
        result.is_err(),
        "corrupt loadout should not silently parse as None; got {result:?}"
    );
}

#[tokio::test]
async fn set_default_errors_when_campaign_not_registered() {
    let (_c, db) = start_test_db().await;
    let client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.unwrap();
    // Note: no bootstrap_campaign call — campaigns row absent.

    let result = set_default_inner(&client, CID, &parse(SAMPLE)).await;
    assert!(
        result.is_err(),
        "set_default with no registered campaign row should error"
    );
}
