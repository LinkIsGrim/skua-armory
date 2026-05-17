//! Certification tests — unit + integration (testcontainers Postgres).

#[cfg(test)]
mod wire_format {
    use super::super::types::Certification;

    fn sample() -> Certification {
        Certification {
            id: "pilot".into(),
            display_name: "Pilot".into(),
            document: "https://docs.example/pilot".into(),
            grant_event: "skua_cert_pilot".into(),
            revoke_event: "skua_cert_revoke_pilot".into(),
        }
    }

    /// `run_list` serializes `Vec<Certification>` to JSON; SQF parses with
    /// `fromJSON`. Lock the field names + shape so a struct rename can't
    /// silently break SQF callers (the SQF side reads `certID`, not `id`).
    #[test]
    fn list_payload_is_json_array_of_objects() {
        let json = serde_json::to_string(&vec![sample()]).expect("serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse back");

        let arr = parsed.as_array().expect("top-level array");
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().expect("element is JSON object");

        for field in [
            "id",
            "display_name",
            "document",
            "grant_event",
            "revoke_event",
        ] {
            assert!(
                obj.contains_key(field),
                "expected JSON object to contain field {field}, got {obj:?}"
            );
        }
        assert_eq!(obj["id"], "pilot");
        assert_eq!(obj["display_name"], "Pilot");
        assert_eq!(obj["grant_event"], "skua_cert_pilot");
        assert_eq!(obj["revoke_event"], "skua_cert_revoke_pilot");
    }

    /// `grant` / `load_player` callbacks ship a `{playerID, id}` JSON
    /// object consumed by `fnc_onGrantReturn`. Lock the key names + types
    /// (player_id must be a string so `BIS_fnc_getUnitByUID` is happy).
    #[test]
    fn grant_event_payload_shape() {
        use crate::domain::PlayerId;

        #[derive(serde::Serialize)]
        struct Event<'a> {
            #[serde(rename = "playerID")]
            player_id: PlayerId,
            #[serde(rename = "id")]
            id: &'a str,
        }

        let json = serde_json::to_string(&Event {
            player_id: PlayerId::new(76_561_198_000_000_000),
            id: "pilot",
        })
        .expect("serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse back");
        let obj = parsed.as_object().expect("object");

        assert_eq!(obj.len(), 2, "exactly two fields expected");
        assert_eq!(obj["playerID"], "76561198000000000");
        assert!(
            obj["playerID"].is_string(),
            "playerID must serialize as a JSON string for BIS_fnc_getUnitByUID"
        );
        assert_eq!(obj["id"], "pilot");
    }
}

#[cfg(test)]
mod file_parsing {
    use super::super::migration::load_files;
    use super::super::types::CertificationFile;

    #[test]
    fn embedded_pilot_loads_cleanly() {
        let files = load_files().expect("load_files should succeed for prod fixtures");
        let pilot = files
            .iter()
            .find(|(id, _)| id == "pilot")
            .expect("pilot.json should be embedded");
        assert_eq!(pilot.1.display_name, "Pilot");
        assert_eq!(pilot.1.grant_event, "skua_cert_pilot");
        assert_eq!(pilot.1.revoke_event, "skua_cert_revoke_pilot");
        assert!(!pilot.1.document.is_empty());
    }

    #[test]
    fn missing_display_name_rejected() {
        let r: Result<CertificationFile, _> =
            serde_json::from_str(r#"{"document": "x", "grant_event": "g", "revoke_event": "r"}"#);
        assert!(r.is_err());
    }

    #[test]
    fn missing_document_rejected() {
        let r: Result<CertificationFile, _> = serde_json::from_str(
            r#"{"display_name": "Pilot", "grant_event": "g", "revoke_event": "r"}"#,
        );
        assert!(r.is_err());
    }

    #[test]
    fn missing_grant_event_rejected() {
        let r: Result<CertificationFile, _> = serde_json::from_str(
            r#"{"display_name": "Pilot", "document": "x", "revoke_event": "r"}"#,
        );
        assert!(r.is_err());
    }

    #[test]
    fn missing_revoke_event_rejected() {
        let r: Result<CertificationFile, _> = serde_json::from_str(
            r#"{"display_name": "Pilot", "document": "x", "grant_event": "g"}"#,
        );
        assert!(r.is_err());
    }

    #[test]
    fn malformed_json_rejected() {
        let r: Result<CertificationFile, _> = serde_json::from_str("{not json}");
        assert!(r.is_err());
    }

    #[test]
    fn extra_fields_ignored() {
        let r: Result<CertificationFile, _> = serde_json::from_str(
            r#"{"display_name": "Pilot", "document": "x", "grant_event": "g", "revoke_event": "r", "extra": "ignored"}"#,
        );
        let parsed = r.unwrap();
        assert_eq!(parsed.display_name, "Pilot");
    }
}

#[cfg(test)]
mod db_tests {
    use deadpool_postgres::{Manager, Pool};
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers::ContainerAsync;
    use testcontainers_modules::testcontainers::ImageExt;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;
    use tokio_postgres::{Config, NoTls};

    use super::super::commands::{get_player_inner, grant_inner, list_inner, revoke_inner};
    use super::super::migration::apply_migration;
    use super::super::types::CertificationFile;
    use crate::database::bootstrap_schema;
    use crate::domain::PlayerId;

    async fn start_pg() -> (ContainerAsync<Postgres>, Pool) {
        let container = Postgres::default()
            .with_tag("18-alpine")
            .start()
            .await
            .expect("failed to start postgres container");
        let host = container.get_host().await.unwrap();
        let port = container.get_host_port_ipv4(5432).await.unwrap();

        let mut cfg = Config::new();
        cfg.host(host.to_string());
        cfg.port(port);
        cfg.user("postgres");
        cfg.password("postgres");
        cfg.dbname("postgres");
        let manager = Manager::new(cfg, NoTls);
        let pool = Pool::builder(manager)
            .max_size(4)
            .build()
            .expect("failed to build test pool");
        (container, pool)
    }

    fn cert(id: &str, display: &str) -> (String, CertificationFile) {
        (
            id.to_string(),
            CertificationFile {
                display_name: display.to_string(),
                document: format!("https://docs.example/{id}"),
                grant_event: format!("skua_cert_{id}"),
                revoke_event: format!("skua_cert_revoke_{id}"),
            },
        )
    }

    // -- apply_migration --

    #[tokio::test]
    async fn apply_migration_empty_set_creates_no_rows() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        apply_migration(&client, vec![]).await.expect("apply empty");

        let count: i64 = client
            .query_one(
                "SELECT COUNT(*)::BIGINT FROM skua_master.certifications",
                &[],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(count, 0);

        let marker: i64 = client
            .query_one(
                "SELECT COUNT(*)::BIGINT FROM skua_master.migration_state
                 WHERE entity_type = 'certification'",
                &[],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(
            marker, 1,
            "migration_state row should be bumped even on empty set"
        );
    }

    #[tokio::test]
    async fn apply_migration_upserts_rows() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        let files = vec![cert("pilot", "Pilot"), cert("medic", "Medic")];
        apply_migration(&client, files).await.expect("apply");

        let rows = list_inner(&client).await.expect("list");
        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["medic", "pilot"], "rows should be present (sorted)");
        let pilot = rows.iter().find(|r| r.id == "pilot").unwrap();
        assert_eq!(pilot.display_name, "Pilot");
        assert_eq!(pilot.grant_event, "skua_cert_pilot");
        assert_eq!(pilot.revoke_event, "skua_cert_revoke_pilot");
    }

    #[tokio::test]
    async fn apply_migration_updates_existing_row_on_repeat() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .expect("apply 1");

        let mut updated = cert("pilot", "Pilot");
        updated.1.document = "https://docs.example/updated".into();
        apply_migration(&client, vec![updated])
            .await
            .expect("apply 2");

        let rows = list_inner(&client).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].document, "https://docs.example/updated");
    }

    #[tokio::test]
    async fn apply_migration_deletes_row_without_file() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        apply_migration(
            &client,
            vec![cert("pilot", "Pilot"), cert("medic", "Medic")],
        )
        .await
        .expect("apply 1");

        // Backdate the pilot row so it predates last_migration_at; otherwise
        // the second migration's delete guard would protect it as "new since
        // last sync".
        client
            .execute(
                "UPDATE skua_master.certifications
                 SET created_at = NOW() - interval '1 hour'
                 WHERE id = 'pilot'",
                &[],
            )
            .await
            .unwrap();

        apply_migration(&client, vec![cert("medic", "Medic")])
            .await
            .expect("apply 2");

        let rows = list_inner(&client).await.unwrap();
        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["medic"]);
    }

    #[tokio::test]
    async fn apply_migration_preserves_in_game_addition() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .expect("apply 1");

        // Simulate an in-game admin adding a cert after the last sync; the
        // row's default created_at = NOW() puts it after last_migration_at.
        client
            .execute(
                "INSERT INTO skua_master.certifications
                    (id, display_name, document, grant_event, revoke_event)
                 VALUES ('xenoarchaeologist', 'Xeno', 'https://x', 'skua_cert_xeno', 'skua_cert_revoke_xeno')",
                &[],
            )
            .await
            .unwrap();

        // Second pass without xenoarchaeologist as a file — should NOT delete it.
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .expect("apply 2");

        let rows = list_inner(&client).await.unwrap();
        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(
            ids,
            ["pilot", "xenoarchaeologist"],
            "in-game addition should survive missing-file pass"
        );
    }

    // -- list_inner --

    #[tokio::test]
    async fn list_inner_empty_returns_empty_vec() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        let rows = list_inner(&client).await.unwrap();
        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn list_inner_returns_seeded_certs() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(
            &client,
            vec![cert("pilot", "Pilot"), cert("medic", "Medic")],
        )
        .await
        .unwrap();

        let rows = list_inner(&client).await.unwrap();
        assert_eq!(rows.len(), 2);
        // sorted by id ascending
        assert_eq!(rows[0].id, "medic");
        assert_eq!(rows[1].id, "pilot");
    }

    // -- grant_inner / get_player_inner / revoke_inner --

    #[tokio::test]
    async fn grant_inner_autocreates_player_and_inserts_cert() {
        let (_c, pool) = start_pg().await;
        let mut client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        let player = PlayerId::new(76_561_198_000_000_000);
        grant_inner(&mut client, player, "pilot")
            .await
            .expect("grant");

        let info_exists: bool = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM skua_master.player_info WHERE steam_id = $1)",
                &[&player],
            )
            .await
            .unwrap()
            .get(0);
        assert!(info_exists, "player_info row should be auto-created");

        let cert_exists: bool = client
            .query_one(
                "SELECT EXISTS(
                    SELECT 1 FROM skua_master.player_certs
                    WHERE steam_id = $1 AND cert_id = $2
                )",
                &[&player, &"pilot"],
            )
            .await
            .unwrap()
            .get(0);
        assert!(cert_exists);
    }

    #[tokio::test]
    async fn grant_inner_duplicate_is_noop() {
        let (_c, pool) = start_pg().await;
        let mut client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        let player = PlayerId::new(1);
        grant_inner(&mut client, player, "pilot")
            .await
            .expect("grant 1");
        grant_inner(&mut client, player, "pilot")
            .await
            .expect("grant 2 (dup)");

        let count: i64 = client
            .query_one(
                "SELECT COUNT(*)::BIGINT FROM skua_master.player_certs WHERE steam_id = $1",
                &[&player],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(count, 1, "duplicate grant should not insert a second row");
    }

    #[tokio::test]
    async fn grant_inner_unknown_cert_fails() {
        let (_c, pool) = start_pg().await;
        let mut client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        // No certs migrated — the FK on player_certs.cert_id will violate.

        let player = PlayerId::new(42);
        let result = grant_inner(&mut client, player, "ghost").await;
        assert!(
            result.is_err(),
            "granting non-existent cert should fail (FK violation)"
        );
    }

    #[tokio::test]
    async fn get_player_inner_returns_granted_ids() {
        let (_c, pool) = start_pg().await;
        let mut client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(
            &client,
            vec![cert("pilot", "Pilot"), cert("medic", "Medic")],
        )
        .await
        .unwrap();

        let player = PlayerId::new(99);
        grant_inner(&mut client, player, "pilot").await.unwrap();
        grant_inner(&mut client, player, "medic").await.unwrap();

        let ids = get_player_inner(&client, player).await.unwrap();
        assert_eq!(ids, vec!["medic".to_string(), "pilot".to_string()]);
    }

    #[tokio::test]
    async fn get_player_inner_empty_for_unknown_player() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        let ids = get_player_inner(&client, PlayerId::new(404)).await.unwrap();
        assert!(ids.is_empty());
    }

    #[tokio::test]
    async fn revoke_inner_removes_row() {
        let (_c, pool) = start_pg().await;
        let mut client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        let player = PlayerId::new(123);
        grant_inner(&mut client, player, "pilot").await.unwrap();
        revoke_inner(&client, player, "pilot").await.unwrap();

        let ids = get_player_inner(&client, player).await.unwrap();
        assert!(ids.is_empty());
    }

    #[tokio::test]
    async fn revoke_inner_unknown_grant_is_noop() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        // Revoking a cert that was never granted should succeed silently.
        revoke_inner(&client, PlayerId::new(7), "pilot")
            .await
            .expect("revoke noop");
    }
}
