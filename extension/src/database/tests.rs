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

/// Verifies that `addons/main/script_macros_enums.hpp` stays in sync with the
/// Rust `DatabaseState` enum. If you add/rename/reorder a variant, either run
/// `cargo test enum_sync -- --nocapture` to see the expected file contents, or
/// edit both at once.
#[cfg(test)]
mod enum_sync {
    use std::fmt::Write;

    use super::super::state::DatabaseState;

    /// Source of truth: must contain every `DatabaseState` variant.
    /// New variants MUST be appended here AND in `script_macros_enums.hpp`.
    const VARIANTS: &[(&str, DatabaseState)] = &[
        ("AWAITCONNECT", DatabaseState::AwaitConnect),
        ("CONNECTEDINIT", DatabaseState::ConnectedInit),
        ("CONNECTEDAWAITINIT", DatabaseState::ConnectedAwaitInit),
        ("FAILED", DatabaseState::Failed),
    ];

    fn expected_hpp() -> String {
        let mut out = String::new();
        out.push_str("// parseNumber is slower than comparing the string directly, so we'll just deal with it\n");
        out.push_str("// these MUST match the Rust extension's DatabaseState enum (see extension/src/database/state.rs)\n");
        let width = VARIANTS.iter().map(|(n, _)| n.len()).max().unwrap_or(0);
        for (name, state) in VARIANTS {
            let _ = writeln!(
                out,
                "#define DATABASESTATE_{name:<width$} (\"{value}\")",
                name = name,
                width = width,
                value = *state as u8,
            );
        }
        out
    }

    fn hpp_path() -> std::path::PathBuf {
        // CARGO_MANIFEST_DIR = .../extension
        let manifest = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest)
            .parent()
            .expect("manifest dir has parent")
            .join("addons/main/script_macros_enums.hpp")
    }

    #[test]
    fn sqf_macros_match_rust_enum() {
        let path = hpp_path();
        let actual = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
        let expected = expected_hpp();

        if actual != expected {
            eprintln!(
                "\n--- expected ({})\n{}\n--- actual\n{}\n",
                path.display(),
                expected,
                actual
            );
            panic!(
                "{} is out of sync with DatabaseState. Replace its contents with the 'expected' block above.",
                path.display()
            );
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use deadpool_postgres::{Manager, Pool};
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers::ImageExt;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;
    use tokio_postgres::{Config, NoTls};

    use super::super::schema::{bootstrap_campaign, bootstrap_master, sanitize_key};

    fn create_test_pool(host: &str, port: u16) -> Pool {
        let mut cfg = Config::new();
        cfg.host(host);
        cfg.port(port);
        cfg.user("postgres");
        cfg.password("postgres");
        cfg.dbname("postgres");

        let manager = Manager::new(cfg, NoTls);
        Pool::builder(manager)
            .max_size(4)
            .build()
            .expect("Failed to build test pool")
    }

    /// Starts a fresh Postgres testcontainer and returns the container handle
    /// alongside a connection pool. Keep the handle alive for the duration of
    /// the test (drop = container stops).
    async fn start_pg() -> (
        testcontainers_modules::testcontainers::ContainerAsync<Postgres>,
        Pool,
    ) {
        let container = Postgres::default()
            .with_tag("18-alpine")
            .start()
            .await
            .expect("Failed to start postgres container");

        let host = container.get_host().await.expect("Failed to get host");
        let port = container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get port");

        let pool = create_test_pool(&host.to_string(), port);
        (container, pool)
    }

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
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.expect("Failed to get client");

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

        // The embedded pilot.json cert should be present after migration.
        let pilot_exists: bool = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM skua_master.certifications WHERE id = 'pilot')",
                &[],
            )
            .await
            .expect("pilot cert query failed")
            .get(0);
        assert!(
            pilot_exists,
            "embedded pilot.json should have been migrated in"
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
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.expect("Failed to get client");

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
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.expect("Failed to get client");

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
        assert!(
            schema_exists,
            "Campaign schema {schema_name} should exist"
        );

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
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.expect("Failed to get client");

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

    #[tokio::test]
    async fn multiple_campaigns_coexist() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.expect("Failed to get client");

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
