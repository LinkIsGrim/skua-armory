//! `player_info` tests — integration (testcontainers Postgres).

use super::commands::list_inner;
use crate::database::{bootstrap_schema, start_test_db};
use crate::domain::PlayerId;

async fn upsert(client: &tokio_postgres::Client, id: u64, name: &str) {
    client
        .execute(
            "INSERT INTO skua_master.player_info (steam_id, name)
             VALUES ($1, $2)
             ON CONFLICT (steam_id) DO UPDATE SET name = EXCLUDED.name",
            &[&PlayerId::new(id), &name],
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn list_inner_empty_returns_empty_roster() {
    let (_c, db) = start_test_db().await;
    let client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.expect("schema");

    let roster = list_inner(&client).await.unwrap();
    assert!(roster.0.is_empty());
}

#[tokio::test]
async fn list_inner_skips_empty_name_rows() {
    let (_c, db) = start_test_db().await;
    let client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.expect("schema");

    upsert(&client, 1, "Alice").await;
    upsert(&client, 2, "").await; // stub upsert from grant_inner FK fallback
    upsert(&client, 3, "Bob").await;

    let roster = list_inner(&client).await.unwrap();
    let names: Vec<&str> = roster.0.iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"Alice"));
    assert!(names.contains(&"Bob"));
    assert!(
        !names.iter().any(|n| n.is_empty()),
        "empty-name stub should be filtered out"
    );
    assert_eq!(roster.0.len(), 2);
}

#[tokio::test]
async fn list_inner_orders_by_last_seen_desc() {
    let (_c, db) = start_test_db().await;
    let client = db.get_conn().await.unwrap();
    bootstrap_schema(&client).await.expect("schema");

    // Insert in reverse-chronological order, then backdate first one so the
    // ORDER BY actually has to reorder them.
    upsert(&client, 1, "Oldest").await;
    upsert(&client, 2, "Middle").await;
    upsert(&client, 3, "Newest").await;

    client
        .execute(
            "UPDATE skua_master.player_info
             SET last_seen = NOW() - interval '2 hours'
             WHERE steam_id = $1",
            &[&PlayerId::new(1)],
        )
        .await
        .unwrap();
    client
        .execute(
            "UPDATE skua_master.player_info
             SET last_seen = NOW() - interval '1 hour'
             WHERE steam_id = $1",
            &[&PlayerId::new(2)],
        )
        .await
        .unwrap();

    let roster = list_inner(&client).await.unwrap();
    let names: Vec<&str> = roster.0.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, vec!["Newest", "Middle", "Oldest"]);
}

/// Lock the JSON wire shape. SQF does `fromJSON _payload` and reads
/// `_x get "steam_id"` / `_x get "name"`. Field renames or `steam_id`
/// becoming a JSON number (Steam IDs exceed JS safe-int range, so they MUST
/// stay as strings) would silently break SQF lookups. Tests the `IntoArma`
/// impl on `Roster` directly so the wire format and the type stay in lockstep.
#[test]
fn roster_into_arma_emits_expected_json() {
    use super::types::{Roster, RosterEntry};
    use arma_rs::IntoArma;

    let payload = Roster(vec![RosterEntry {
        steam_id: PlayerId::new(76_561_198_000_000_000),
        name: "Alice".into(),
    }]);

    let value = payload.to_arma();
    let arma_rs::Value::String(json) = value else {
        panic!("expected JSON string, got {value:?}");
    };

    let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse back");
    let arr = parsed.as_array().expect("top-level array");
    assert_eq!(arr.len(), 1);
    let obj = arr[0].as_object().expect("element is JSON object");

    assert_eq!(obj.len(), 2, "exactly two fields expected");
    assert_eq!(obj["steam_id"], "76561198000000000");
    assert!(
        obj["steam_id"].is_string(),
        "steam_id must be a JSON string (Steam IDs exceed JS safe-int)"
    );
    assert_eq!(obj["name"], "Alice");
}
