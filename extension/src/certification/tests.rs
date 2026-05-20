//! Certification tests — unit + integration (testcontainers Postgres).

#[cfg(test)]
mod wire_format {
    use super::super::types::Certification;

    fn sample() -> Certification {
        Certification {
            id: "pilot".into(),
            display_name: "Pilot".into(),
            document: "https://docs.example/pilot".into(),
            description: "Pilot training.".into(),
            perk: "Access to rotary aircraft.".into(),
            pay_bonus: 1000,
            grant_event: "skua_cert_pilot".into(),
            revoke_event: "skua_cert_revoke_pilot".into(),
            requires: vec!["rifleman".into()],
        }
    }

    /// `push_list` serializes `Vec<Certification>` to JSON; SQF parses with
    /// `fromJSON`. Lock the field names + shape so a struct rename can't
    /// silently break SQF callers.
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
            "description",
            "perk",
            "pay_bonus",
            "grant_event",
            "revoke_event",
            "requires",
        ] {
            assert!(
                obj.contains_key(field),
                "expected JSON object to contain field {field}, got {obj:?}"
            );
        }
        assert_eq!(obj["id"], "pilot");
        assert_eq!(obj["display_name"], "Pilot");
        assert_eq!(obj["pay_bonus"], 1000);
        assert_eq!(obj["grant_event"], "skua_cert_pilot");
        assert_eq!(obj["revoke_event"], "skua_cert_revoke_pilot");
        assert_eq!(
            obj["requires"]
                .as_array()
                .expect("requires is JSON array")
                .len(),
            1
        );
    }

    /// `CertificationGranted` events (both ad-hoc grants and `push_player_certs`
    /// replays and watchdog-detected drift) ship a `{player_id, cert_id}` JSON
    /// object consumed by `fnc_onCertificationGranted`. Lock the key names +
    /// types (`player_id` must be a string so `BIS_fnc_getUnitByUID` is happy)
    /// by serializing through the actual `Event` payload path.
    #[test]
    fn grant_event_payload_shape() {
        use crate::domain::PlayerId;
        use crate::event::Event;

        let json = Event::CertificationGranted {
            player_id: PlayerId::new(76_561_198_000_000_000),
            cert_id: "pilot".into(),
        }
        .payload()
        .expect("serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse back");
        let obj = parsed.as_object().expect("object");

        assert_eq!(obj.len(), 2, "exactly two fields expected");
        assert_eq!(obj["player_id"], "76561198000000000");
        assert!(
            obj["player_id"].is_string(),
            "player_id must serialize as a JSON string for BIS_fnc_getUnitByUID"
        );
        assert_eq!(obj["cert_id"], "pilot");
    }

    /// `certification:get_player` callback payload carries the queried
    /// `player_id` alongside the cert id list so the SQF callback can route
    /// the response back to whichever player it asked about. Lock the JSON
    /// keys + `player_id` stringification through the `IntoArma` impl on
    /// `PlayerCerts`.
    #[test]
    fn get_player_callback_payload_shape() {
        use super::super::types::PlayerCerts;
        use crate::domain::PlayerId;
        use arma_rs::IntoArma;

        let payload = PlayerCerts {
            player_id: PlayerId::new(76_561_198_000_000_000),
            cert_ids: vec!["pilot".into(), "medic".into()],
        };

        let value = payload.to_arma();
        let arma_rs::Value::String(json) = value else {
            panic!("expected JSON string, got {value:?}");
        };

        let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse back");
        let obj = parsed.as_object().expect("object");
        assert_eq!(obj.len(), 2, "exactly two fields expected");
        assert_eq!(obj["player_id"], "76561198000000000");
        assert!(
            obj["player_id"].is_string(),
            "player_id must serialize as a JSON string for BIS_fnc_getUnitByUID"
        );
        let cert_ids = obj["cert_ids"].as_array().expect("cert_ids is array");
        assert_eq!(cert_ids.len(), 2);
        assert_eq!(cert_ids[0], "pilot");
        assert_eq!(cert_ids[1], "medic");
    }
}

#[cfg(test)]
mod file_parsing {
    use super::super::migration::load_files;
    use super::super::types::CertificationFile;

    /// Full JSON body with every required field set; tests omit one field at
    /// a time to verify each is `NOT NULL` at the deserializer.
    const COMPLETE_JSON: &str = r#"{
        "display_name": "Pilot",
        "document": "x",
        "description": "d",
        "perk": "p",
        "pay_bonus": 100,
        "grant_event": "g",
        "revoke_event": "r",
        "requires": []
    }"#;

    #[test]
    fn embedded_rifleman_loads_cleanly() {
        let files = load_files().expect("load_files should succeed for prod fixtures");
        let rifleman = files
            .iter()
            .find(|(id, _)| id == "rifleman")
            .expect("rifleman.json should be embedded");
        assert_eq!(rifleman.1.display_name, "Rifleman");
        assert_eq!(rifleman.1.grant_event, "skua_cert_rifleman");
        assert_eq!(rifleman.1.revoke_event, "skua_cert_revoke_rifleman");
        assert!(!rifleman.1.document.is_empty());
        assert!(!rifleman.1.description.is_empty());
        assert!(!rifleman.1.perk.is_empty());
        assert_eq!(rifleman.1.pay_bonus, 2000);
        assert!(rifleman.1.requires.is_empty(), "rifleman has no prereqs");
    }

    #[test]
    fn complete_json_parses() {
        let parsed: CertificationFile = serde_json::from_str(COMPLETE_JSON).unwrap();
        assert_eq!(parsed.display_name, "Pilot");
        assert_eq!(parsed.pay_bonus, 100);
        assert!(parsed.requires.is_empty());
    }

    #[test]
    fn missing_display_name_rejected() {
        let body = COMPLETE_JSON.replace(r#""display_name": "Pilot","#, "");
        assert!(serde_json::from_str::<CertificationFile>(&body).is_err());
    }

    #[test]
    fn missing_document_rejected() {
        let body = COMPLETE_JSON.replace(r#""document": "x","#, "");
        assert!(serde_json::from_str::<CertificationFile>(&body).is_err());
    }

    #[test]
    fn missing_description_rejected() {
        let body = COMPLETE_JSON.replace(r#""description": "d","#, "");
        assert!(serde_json::from_str::<CertificationFile>(&body).is_err());
    }

    #[test]
    fn missing_perk_rejected() {
        let body = COMPLETE_JSON.replace(r#""perk": "p","#, "");
        assert!(serde_json::from_str::<CertificationFile>(&body).is_err());
    }

    #[test]
    fn missing_pay_bonus_rejected() {
        let body = COMPLETE_JSON.replace(r#""pay_bonus": 100,"#, "");
        assert!(serde_json::from_str::<CertificationFile>(&body).is_err());
    }

    #[test]
    fn missing_grant_event_rejected() {
        let body = COMPLETE_JSON.replace(r#""grant_event": "g","#, "");
        assert!(serde_json::from_str::<CertificationFile>(&body).is_err());
    }

    #[test]
    fn missing_revoke_event_rejected() {
        let body = COMPLETE_JSON.replace(r#""revoke_event": "r","#, "");
        assert!(serde_json::from_str::<CertificationFile>(&body).is_err());
    }

    #[test]
    fn missing_requires_rejected() {
        let body = COMPLETE_JSON.replace(r#""requires": []"#, "");
        assert!(serde_json::from_str::<CertificationFile>(&body).is_err());
    }

    #[test]
    fn malformed_json_rejected() {
        let r: Result<CertificationFile, _> = serde_json::from_str("{not json}");
        assert!(r.is_err());
    }

    #[test]
    fn extra_fields_ignored() {
        // combat_engineer.json uses perk_antistasi/perk_liberation alongside
        // the canonical schema; those extras must round-trip silently.
        let body = COMPLETE_JSON.replace(
            r#""requires": []"#,
            r#""requires": [], "perk_antistasi": "ignored""#,
        );
        let parsed: CertificationFile = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed.display_name, "Pilot");
    }
}

#[cfg(test)]
mod db_tests {
    use super::super::commands::{get_player_inner, grant_inner, list_inner, revoke_inner};
    use super::super::migration::apply_migration;
    use super::super::types::CertificationFile;
    use crate::database::{bootstrap_schema, start_test_db};
    use crate::domain::PlayerId;

    fn cert(id: &str, display: &str) -> (String, CertificationFile) {
        (
            id.to_string(),
            CertificationFile {
                display_name: display.to_string(),
                document: format!("https://docs.example/{id}"),
                description: format!("{display} certification."),
                perk: format!("{display} perks."),
                pay_bonus: 0,
                grant_event: format!("skua_cert_{id}"),
                revoke_event: format!("skua_cert_revoke_{id}"),
                requires: Vec::new(),
            },
        )
    }

    // -- apply_migration --

    #[tokio::test]
    async fn apply_migration_empty_set_creates_no_rows() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.unwrap();
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
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.unwrap();
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
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.unwrap();
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
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.unwrap();
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
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .expect("apply 1");

        // Simulate an in-game admin adding a cert after the last sync; the
        // row's default created_at = NOW() puts it after last_migration_at.
        client
            .execute(
                "INSERT INTO skua_master.certifications
                    (id, display_name, document, description, perk,
                     grant_event, revoke_event)
                 VALUES ('xenoarchaeologist', 'Xeno', 'https://x', 'd', 'p',
                         'skua_cert_xeno', 'skua_cert_revoke_xeno')",
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
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        let rows = list_inner(&client).await.unwrap();
        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn list_inner_returns_seeded_certs() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.unwrap();
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

    const ADMIN: PlayerId = PlayerId::new(76_561_198_111_111_111);

    #[tokio::test]
    async fn grant_inner_autocreates_player_and_inserts_cert() {
        let (_c, db) = start_test_db().await;
        let mut client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        let player = PlayerId::new(76_561_198_000_000_000);
        grant_inner(&mut client, player, "pilot", ADMIN)
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
        let (_c, db) = start_test_db().await;
        let mut client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        let player = PlayerId::new(1);
        grant_inner(&mut client, player, "pilot", ADMIN)
            .await
            .expect("grant 1");
        grant_inner(&mut client, player, "pilot", ADMIN)
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
        let (_c, db) = start_test_db().await;
        let mut client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        // No certs migrated — the FK on player_certs.cert_id will violate.

        let player = PlayerId::new(42);
        let result = grant_inner(&mut client, player, "ghost", ADMIN).await;
        assert!(
            result.is_err(),
            "granting non-existent cert should fail (FK violation)"
        );
    }

    #[tokio::test]
    async fn grant_inner_records_granted_by() {
        let (_c, db) = start_test_db().await;
        let mut client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        let player = PlayerId::new(555);
        grant_inner(&mut client, player, "pilot", ADMIN)
            .await
            .expect("grant");

        let granted_by_raw: i64 = client
            .query_one(
                "SELECT granted_by FROM skua_master.player_certs
                 WHERE steam_id = $1 AND cert_id = $2",
                &[&player, &"pilot"],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(PlayerId::from_i64(granted_by_raw), ADMIN);
    }

    #[tokio::test]
    async fn grant_inner_creates_granter_player_info_row() {
        let (_c, db) = start_test_db().await;
        let mut client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        let player = PlayerId::new(800);
        // Admin has no prior player_info row — grant must upsert one to
        // satisfy player_certs.granted_by FK.
        grant_inner(&mut client, player, "pilot", ADMIN)
            .await
            .expect("grant");

        let admin_exists: bool = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM skua_master.player_info WHERE steam_id = $1)",
                &[&ADMIN],
            )
            .await
            .unwrap()
            .get(0);
        assert!(
            admin_exists,
            "granter player_info row should be auto-created"
        );
    }

    #[tokio::test]
    async fn grant_inner_idempotent_preserves_first_granter() {
        let (_c, db) = start_test_db().await;
        let mut client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        let player = PlayerId::new(900);
        let admin_b = PlayerId::new(76_561_198_222_222_222);

        grant_inner(&mut client, player, "pilot", ADMIN)
            .await
            .expect("grant 1");
        grant_inner(&mut client, player, "pilot", admin_b)
            .await
            .expect("grant 2 (dup, different admin)");

        let granted_by_raw: i64 = client
            .query_one(
                "SELECT granted_by FROM skua_master.player_certs
                 WHERE steam_id = $1 AND cert_id = $2",
                &[&player, &"pilot"],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(
            PlayerId::from_i64(granted_by_raw),
            ADMIN,
            "ON CONFLICT DO NOTHING should preserve the original granter"
        );
    }

    #[tokio::test]
    async fn get_player_inner_returns_granted_ids() {
        let (_c, db) = start_test_db().await;
        let mut client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(
            &client,
            vec![cert("pilot", "Pilot"), cert("medic", "Medic")],
        )
        .await
        .unwrap();

        let player = PlayerId::new(99);
        grant_inner(&mut client, player, "pilot", ADMIN)
            .await
            .unwrap();
        grant_inner(&mut client, player, "medic", ADMIN)
            .await
            .unwrap();

        let ids = get_player_inner(&client, player).await.unwrap();
        assert_eq!(ids, vec!["medic".to_string(), "pilot".to_string()]);
    }

    #[tokio::test]
    async fn get_player_inner_empty_for_unknown_player() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        let ids = get_player_inner(&client, PlayerId::new(404)).await.unwrap();
        assert!(ids.is_empty());
    }

    #[tokio::test]
    async fn revoke_inner_removes_row() {
        let (_c, db) = start_test_db().await;
        let mut client = db.get_conn().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![cert("pilot", "Pilot")])
            .await
            .unwrap();

        let player = PlayerId::new(123);
        grant_inner(&mut client, player, "pilot", ADMIN)
            .await
            .unwrap();
        revoke_inner(&client, player, "pilot").await.unwrap();

        let ids = get_player_inner(&client, player).await.unwrap();
        assert!(ids.is_empty());
    }

    #[tokio::test]
    async fn revoke_inner_unknown_grant_is_noop() {
        let (_c, db) = start_test_db().await;
        let client = db.get_conn().await.unwrap();
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
