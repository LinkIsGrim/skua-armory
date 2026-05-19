//! Database integration tests.
//!
//! Integration tests exercise the bootstrap logic against a real Postgres
//! instance via testcontainers, so they require Docker. Unit tests (sync
//! check, `sanitize_key`, `parse_campaign_arg`, and arma roundtrip tests have no external
//! dependencies.

#[cfg(test)]
mod arg_parsing {
    use super::super::schema::parse_campaign_arg;

    #[test]
    fn empty_string_means_master_only() {
        assert_eq!(parse_campaign_arg(""), Ok(None));
    }

    #[test]
    fn valid_key_round_trips() {
        assert_eq!(
            parse_campaign_arg("valid_key"),
            Ok(Some("valid_key".to_string()))
        );
    }

    #[test]
    fn dashes_and_spaces_normalize() {
        assert_eq!(
            parse_campaign_arg("My-Campaign Name"),
            Ok(Some("my_campaign_name".to_string()))
        );
    }

    #[test]
    fn invalid_chars_reject() {
        assert!(parse_campaign_arg("BAD!key").is_err());
        assert!(parse_campaign_arg("bad/key").is_err());
    }
}

#[cfg(test)]
mod arma_roundtrip {
    use super::super::state::DatabaseState;
    use arma_rs::{FromArma, IntoArma, Value};

    fn assert_round_trip(state: DatabaseState, expected_u8: u8) {
        assert_eq!(state.to_arma(), Value::Number(expected_u8.into()));
        let parsed = DatabaseState::from_arma(expected_u8.to_string()).expect("from_arma");
        assert_eq!(parsed, state);
    }

    #[test]
    fn all_variants_round_trip() {
        assert_round_trip(DatabaseState::AwaitConnect, 0);
        assert_round_trip(DatabaseState::ConnectedInit, 1);
        assert_round_trip(DatabaseState::ConnectedAwaitInit, 2);
        assert_round_trip(DatabaseState::Failed, 3);
    }

    #[test]
    fn unknown_value_rejected() {
        assert!(DatabaseState::from_arma("99".into()).is_err());
    }

    #[test]
    fn non_numeric_rejected() {
        assert!(DatabaseState::from_arma("nope".into()).is_err());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::super::schema::{bootstrap_campaign, bootstrap_master, sanitize_key};
    use crate::database::start_test_db;

    const SCHEMA_EXISTS_QUERY: &str = r"
        SELECT EXISTS (
            SELECT 1 FROM information_schema.schemata
            WHERE schema_name = $1
        )
    ";

    const TABLE_EXISTS_QUERY: &str = r"
        SELECT EXISTS (
            SELECT 1 FROM information_schema.tables
            WHERE table_schema = $1 AND table_name = $2
        )
    ";

    const INDEX_EXISTS_QUERY: &str = r"
        SELECT EXISTS (
            SELECT 1 FROM pg_indexes
            WHERE schemaname = $1 AND indexname = $2
        )
    ";

    // -------------------------------------------------------------------------
    // sanitize_key (pure)

    #[test]
    fn sanitize_key_lowercases_and_replaces() {
        assert_eq!(
            sanitize_key("My-Campaign Name").unwrap(),
            "my_campaign_name"
        );
    }

    #[test]
    fn sanitize_key_rejects_too_short() {
        assert!(sanitize_key("ab").is_err());
    }

    #[test]
    fn sanitize_key_rejects_too_long() {
        let too_long = "a".repeat(50);
        assert!(sanitize_key(&too_long).is_err());
    }

    #[test]
    fn sanitize_key_rejects_invalid_chars() {
        assert!(sanitize_key("bad/key").is_err());
        assert!(sanitize_key("bad.key").is_err());
        assert!(sanitize_key("bad!key").is_err());
    }

    // -------------------------------------------------------------------------
    // bootstrap_master

    #[tokio::test]
    async fn bootstrap_master_creates_schema_and_tables() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.expect("Failed to get client");

        bootstrap_master(&client)
            .await
            .expect("bootstrap_master should succeed");

        let schema_exists: bool = client
            .query_one(SCHEMA_EXISTS_QUERY, &[&"skua_master"])
            .await
            .expect("Schema query failed")
            .get(0);
        assert!(schema_exists, "skua_master schema should exist");

        let expected_tables = [
            "ranks",
            "certifications",
            "campaigns",
            "player_info",
            "player_certs",
            "migration_state",
        ];

        for table in expected_tables {
            let exists: bool = client
                .query_one(TABLE_EXISTS_QUERY, &[&"skua_master", &table])
                .await
                .unwrap_or_else(|e| panic!("Table query failed for {table}: {e}"))
                .get(0);
            assert!(exists, "Table {table} should exist in skua_master");
        }

        let expected_indexes = [
            ("skua_master", "idx_player_info_is_admin"),
            ("skua_master", "idx_player_info_is_banned"),
            ("skua_master", "idx_player_certs_steam_id"),
            ("skua_master", "idx_player_certs_cert_id"),
        ];

        for (schema, index) in expected_indexes {
            let exists: bool = client
                .query_one(INDEX_EXISTS_QUERY, &[&schema, &index])
                .await
                .unwrap_or_else(|e| panic!("Index query failed for {index}: {e}"))
                .get(0);
            assert!(exists, "Index {index} should exist in {schema}");
        }

        // Default rank seeded by bootstrap_schema (called by bootstrap_master).
        let default_rank_name: String = client
            .query_one(
                "SELECT display_name FROM skua_master.ranks WHERE id = 0",
                &[],
            )
            .await
            .expect("default rank query failed")
            .get(0);
        assert_eq!(
            default_rank_name, "Unranked",
            "bootstrap should seed (0, 'Unranked')"
        );

        // The embedded rifleman.json cert should be present after migration.
        let rifleman_exists: bool = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM skua_master.certifications WHERE id = 'rifleman')",
                &[],
            )
            .await
            .expect("rifleman cert query failed")
            .get(0);
        assert!(
            rifleman_exists,
            "embedded rifleman.json should have been migrated in"
        );

        // migration_state should have rows for both entity types.
        for entity in ["certification", "rank"] {
            let bumped: bool = client
                .query_one(
                    "SELECT EXISTS(SELECT 1 FROM skua_master.migration_state
                     WHERE entity_type = $1)",
                    &[&entity],
                )
                .await
                .expect("migration_state query failed")
                .get(0);
            assert!(bumped, "migration_state should have entry for {entity}");
        }
    }

    #[tokio::test]
    async fn bootstrap_master_is_idempotent() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.expect("Failed to get client");

        bootstrap_master(&client)
            .await
            .expect("first bootstrap_master should succeed");
        bootstrap_master(&client)
            .await
            .expect("second bootstrap_master should succeed (idempotent)");

        let schema_exists: bool = client
            .query_one(SCHEMA_EXISTS_QUERY, &[&"skua_master"])
            .await
            .expect("Schema query failed")
            .get(0);
        assert!(schema_exists);
    }

    // -------------------------------------------------------------------------
    // bootstrap_campaign

    #[tokio::test]
    async fn bootstrap_campaign_creates_schema_and_tables() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.expect("Failed to get client");

        bootstrap_master(&client)
            .await
            .expect("bootstrap_master should succeed");
        bootstrap_campaign(&client, "test_campaign")
            .await
            .expect("bootstrap_campaign should succeed");

        let schema_name = "skua_campaign_test_campaign";
        let schema_exists: bool = client
            .query_one(SCHEMA_EXISTS_QUERY, &[&schema_name])
            .await
            .expect("Schema query failed")
            .get(0);
        assert!(schema_exists, "Campaign schema {schema_name} should exist");

        let expected_tables = ["player_data", "player_world_data", "world_data"];
        for table in expected_tables {
            let exists: bool = client
                .query_one(TABLE_EXISTS_QUERY, &[&schema_name, &table])
                .await
                .unwrap_or_else(|e| panic!("Table query failed for {table}: {e}"))
                .get(0);
            assert!(exists, "Table {table} should exist in {schema_name}");
        }

        let expected_indexes = ["idx_player_world_data_world", "idx_world_data_world"];
        for index in expected_indexes {
            let exists: bool = client
                .query_one(INDEX_EXISTS_QUERY, &[&schema_name, &index])
                .await
                .unwrap_or_else(|e| panic!("Index query failed for {index}: {e}"))
                .get(0);
            assert!(exists, "Index {index} should exist in {schema_name}");
        }

        let registered: bool = client
            .query_one(
                "SELECT EXISTS (SELECT 1 FROM skua_master.campaigns WHERE campaign_id = $1)",
                &[&schema_name],
            )
            .await
            .expect("Registration query failed")
            .get(0);
        assert!(
            registered,
            "Campaign should be registered in skua_master.campaigns"
        );
    }

    #[tokio::test]
    async fn bootstrap_campaign_is_idempotent() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.expect("Failed to get client");

        bootstrap_master(&client)
            .await
            .expect("bootstrap_master should succeed");
        bootstrap_campaign(&client, "idempotent_test")
            .await
            .expect("first bootstrap_campaign should succeed");
        bootstrap_campaign(&client, "idempotent_test")
            .await
            .expect("second bootstrap_campaign should succeed (idempotent)");
    }

    // -------------------------------------------------------------------------
    // upsert_player

    use super::super::player::upsert_player_inner;
    use crate::domain::PlayerId;

    const TEST_STEAM_ID: u64 = 76_561_198_000_000_001;

    #[tokio::test]
    async fn upsert_player_inserts_on_first_call() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.expect("Failed to get client");
        bootstrap_master(&client)
            .await
            .expect("bootstrap_master should succeed");

        let pid = PlayerId::new(TEST_STEAM_ID);
        let info = upsert_player_inner(&client, pid, "TestPlayer")
            .await
            .expect("upsert should succeed");

        assert_eq!(info.steam_id, pid);
        assert_eq!(info.name, "TestPlayer");
        assert!(!info.is_admin);
        assert!(!info.is_banned);
        assert_eq!(info.rank, 0);
        // On insert, both timestamps default to NOW() in the same statement, so
        // they should be equal (or essentially so).
        assert_eq!(
            info.first_seen, info.last_seen,
            "first_seen should equal last_seen on insert"
        );
    }

    #[tokio::test]
    async fn upsert_player_updates_name_and_last_seen() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.expect("Failed to get client");
        bootstrap_master(&client)
            .await
            .expect("bootstrap_master should succeed");

        let pid = PlayerId::new(TEST_STEAM_ID);
        let first = upsert_player_inner(&client, pid, "OriginalName")
            .await
            .expect("first upsert should succeed");

        // Sleep so last_seen NOW() has measurable progress.
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let second = upsert_player_inner(&client, pid, "RenamedPlayer")
            .await
            .expect("second upsert should succeed");

        assert_eq!(second.steam_id, pid);
        assert_eq!(second.name, "RenamedPlayer");
        assert_eq!(
            second.first_seen, first.first_seen,
            "first_seen must be preserved across upserts"
        );
        assert!(
            second.last_seen > first.last_seen,
            "last_seen should advance on conflict-update; was {:?}, now {:?}",
            first.last_seen,
            second.last_seen,
        );
    }

    #[tokio::test]
    async fn upsert_player_preserves_admin_banned_rank_overrides() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.expect("Failed to get client");
        bootstrap_master(&client)
            .await
            .expect("bootstrap_master should succeed");

        let pid = PlayerId::new(TEST_STEAM_ID);
        upsert_player_inner(&client, pid, "Player")
            .await
            .expect("insert should succeed");

        // Out-of-band admin flag set (e.g. by an admin tool); the next upsert
        // must NOT clobber it back to false.
        client
            .execute(
                "UPDATE skua_master.player_info
                 SET is_admin = TRUE, is_banned = TRUE
                 WHERE steam_id = $1",
                &[&pid],
            )
            .await
            .expect("admin flag update should succeed");

        let info = upsert_player_inner(&client, pid, "Player")
            .await
            .expect("second upsert should succeed");

        assert!(
            info.is_admin,
            "is_admin must survive upsert; only name and last_seen are touched"
        );
        assert!(
            info.is_banned,
            "is_banned must survive upsert; only name and last_seen are touched"
        );
    }

    #[tokio::test]
    async fn multiple_campaigns_coexist() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.expect("Failed to get client");

        bootstrap_master(&client)
            .await
            .expect("bootstrap_master should succeed");

        let campaigns = ["campaign_alpha", "campaign_beta", "campaign_gamma"];
        for c in campaigns {
            bootstrap_campaign(&client, c)
                .await
                .unwrap_or_else(|e| panic!("bootstrap_campaign {c} failed: {e:?}"));
        }

        for c in campaigns {
            let schema = format!("skua_campaign_{c}");
            let exists: bool = client
                .query_one(SCHEMA_EXISTS_QUERY, &[&schema])
                .await
                .expect("Query failed")
                .get(0);
            assert!(exists, "Campaign schema {schema} should exist");
        }
    }
}

/// Live database tests against a running Postgres (e.g. via compose.yaml).
///
/// These are `#[ignore]` by default so they don't block CI, but you can run them
/// when you have a live database:
///
/// ```sh
/// # Terminal 1: Start the database
/// cd database && docker compose up -d
///
/// # Terminal 2: Run the live tests
/// cargo test --lib database::tests::live_db -- --ignored --nocapture
/// ```
///
/// Environment variables (optional):
/// - `DATABASE_HOST` (default: 127.0.0.1)
/// - `DATABASE_PORT` (default: 55432)
/// - `DATABASE_USER` (default: postgres)
/// - `DATABASE_PASSWORD` (default: changeit)
/// - `DATABASE_NAME` (default: postgres)
#[cfg(test)]
mod live_db {
    use std::env;

    use deadpool_postgres::{Manager, Pool};
    use tokio_postgres::Config;
    use tokio_postgres::NoTls;

    use super::super::schema::{bootstrap_campaign, bootstrap_master};

    fn create_live_pool() -> Result<Pool, Box<dyn std::error::Error>> {
        let host = env::var("DATABASE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("DATABASE_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(55432);
        let user = env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
        let password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "changeit".to_string());
        let dbname = env::var("DATABASE_NAME").unwrap_or_else(|_| "postgres".to_string());

        println!("Connecting to live database: {user}@{host}:{port}/{dbname}");

        let mut cfg = Config::new();
        cfg.host(&host)
            .port(port)
            .user(&user)
            .password(&password)
            .dbname(&dbname);

        let mgr = Manager::new(cfg, NoTls);
        let pool = Pool::builder(mgr).build()?;
        Ok(pool)
    }

    #[tokio::test]
    #[ignore = "Requires a live database; see module docs for instructions"]
    async fn bootstrap_master_on_live_db() {
        let pool = create_live_pool().expect("Failed to create pool");
        let client = pool
            .get()
            .await
            .expect("Failed to get connection from live database");

        println!("Testing bootstrap_master on live database...");
        match bootstrap_master(&client).await {
            Ok(()) => {
                println!("✓ bootstrap_master succeeded!");
            }
            Err(e) => {
                eprintln!("✗ bootstrap_master failed: {e:?}");
                panic!("bootstrap_master failed: {e:?}");
            }
        }
    }

    #[tokio::test]
    #[ignore = "Requires a live database; see module docs for instructions"]
    async fn bootstrap_campaign_on_live_db() {
        let pool = create_live_pool().expect("Failed to create pool");
        let client = pool
            .get()
            .await
            .expect("Failed to get connection from live database");

        // First bootstrap master
        println!("Pre-requisite: bootstrap_master...");
        bootstrap_master(&client)
            .await
            .expect("bootstrap_master must succeed first");

        // Now test campaign bootstrap
        println!("Testing bootstrap_campaign on live database...");
        match bootstrap_campaign(&client, "test_campaign_live").await {
            Ok(()) => {
                println!("✓ bootstrap_campaign succeeded!");
            }
            Err(e) => {
                eprintln!("✗ bootstrap_campaign failed: {e:?}");
                panic!("bootstrap_campaign failed: {e:?}");
            }
        }
    }
}
