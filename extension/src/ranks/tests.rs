//! Rank tests — unit + integration (testcontainers Postgres).

#[cfg(test)]
mod types_arma {
    use super::super::types::Rank;
    use arma_rs::{IntoArma, Value};

    #[test]
    fn into_arma_emits_struct_map_fields() {
        let rank = Rank {
            id: 5,
            display_name: "Sergeant".into(),
        };
        let arma = rank.to_arma();
        let Value::Array(items) = arma else {
            panic!("Rank should serialize as Value::Array, got {arma:?}");
        };
        assert_eq!(items.len(), 2);
        assert!(
            items.contains(&Value::Array(vec![
                Value::String("id".into()),
                Value::Number(5.0)
            ])),
            "missing id field in IntoArma output: {items:?}"
        );
        assert!(
            items.contains(&Value::Array(vec![
                Value::String("display_name".into()),
                Value::String("Sergeant".into())
            ])),
            "missing display_name field in IntoArma output: {items:?}"
        );
    }
}

#[cfg(test)]
mod file_parsing {
    use super::super::migration::load_files;
    use super::super::types::RankFile;

    #[test]
    fn embedded_unranked_loads_cleanly() {
        let files = load_files().expect("load_files should succeed for prod fixtures");
        let unranked = files
            .iter()
            .find(|r| r.id == 0)
            .expect("id=0 rank should be embedded");
        assert_eq!(unranked.display_name, "Unranked");
    }

    #[test]
    fn missing_id_rejected() {
        let r: Result<RankFile, _> = serde_json::from_str(r#"{"display_name": "Sergeant"}"#);
        assert!(r.is_err());
    }

    #[test]
    fn missing_display_name_rejected() {
        let r: Result<RankFile, _> = serde_json::from_str(r#"{"id": 5}"#);
        assert!(r.is_err());
    }

    #[test]
    fn malformed_json_rejected() {
        let r: Result<RankFile, _> = serde_json::from_str("{not json}");
        assert!(r.is_err());
    }

    #[test]
    fn non_integer_id_rejected() {
        let r: Result<RankFile, _> =
            serde_json::from_str(r#"{"id": "five", "display_name": "Five"}"#);
        assert!(r.is_err());
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

    use super::super::commands::{get_player_inner, list_inner, set_player_inner};
    use super::super::migration::apply_migration;
    use super::super::types::RankFile;
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

    fn rank(id: i16, name: &str) -> RankFile {
        RankFile {
            id,
            display_name: name.into(),
        }
    }

    // -- apply_migration --

    #[tokio::test]
    async fn apply_migration_empty_set_keeps_default_rank() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        // Default (0, 'Unranked') is seeded by bootstrap_schema and is
        // protected from migration pruning.
        apply_migration(&client, vec![]).await.expect("apply empty");

        let rows = list_inner(&client).await.unwrap();
        let ids: Vec<i16> = rows.iter().map(|r| r.id).collect();
        assert_eq!(ids, [0], "default rank should survive empty migration");

        let marker: i64 = client
            .query_one(
                "SELECT COUNT(*)::BIGINT FROM skua_master.migration_state
                 WHERE entity_type = 'rank'",
                &[],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(marker, 1);
    }

    #[tokio::test]
    async fn apply_migration_upserts_ranks() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        apply_migration(
            &client,
            vec![rank(0, "Unranked"), rank(1, "Recruit"), rank(5, "Sergeant")],
        )
        .await
        .expect("apply");

        let rows = list_inner(&client).await.unwrap();
        let ids: Vec<i16> = rows.iter().map(|r| r.id).collect();
        assert_eq!(ids, [0, 1, 5]);
        let sergeant = rows.iter().find(|r| r.id == 5).unwrap();
        assert_eq!(sergeant.display_name, "Sergeant");
    }

    #[tokio::test]
    async fn apply_migration_updates_display_name() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        apply_migration(&client, vec![rank(1, "Recruit")])
            .await
            .unwrap();
        apply_migration(&client, vec![rank(1, "Private")])
            .await
            .unwrap();

        let rows = list_inner(&client).await.unwrap();
        let r1 = rows.iter().find(|r| r.id == 1).unwrap();
        assert_eq!(r1.display_name, "Private");
    }

    #[tokio::test]
    async fn apply_migration_deletes_orphan_rank() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        apply_migration(&client, vec![rank(1, "Recruit"), rank(2, "Corporal")])
            .await
            .unwrap();

        // Backdate so the second migration's delete guard treats id=2 as
        // predating the last marker.
        client
            .execute(
                "UPDATE skua_master.ranks SET created_at = NOW() - interval '1 hour'
                 WHERE id = 2",
                &[],
            )
            .await
            .unwrap();

        apply_migration(&client, vec![rank(1, "Recruit")])
            .await
            .unwrap();

        let rows = list_inner(&client).await.unwrap();
        let ids: Vec<i16> = rows.iter().map(|r| r.id).collect();
        assert_eq!(ids, [0, 1], "default and kept ranks should remain");
    }

    #[tokio::test]
    async fn apply_migration_preserves_in_game_rank() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        apply_migration(&client, vec![rank(1, "Recruit")])
            .await
            .unwrap();

        // Insert an in-game rank after the last sync; its created_at = NOW()
        // > last_migration_at so it must survive a missing-file pass.
        client
            .execute(
                "INSERT INTO skua_master.ranks (id, display_name)
                 VALUES (99, 'WIP')",
                &[],
            )
            .await
            .unwrap();

        apply_migration(&client, vec![rank(1, "Recruit")])
            .await
            .unwrap();

        let rows = list_inner(&client).await.unwrap();
        assert!(rows.iter().any(|r| r.id == 99));
    }

    // -- list_inner --

    #[tokio::test]
    async fn list_inner_returns_seeded_default() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        // bootstrap_schema seeds (0, 'Unranked')
        let rows = list_inner(&client).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, 0);
        assert_eq!(rows[0].display_name, "Unranked");
    }

    // -- get_player_inner --

    #[tokio::test]
    async fn get_player_inner_unknown_returns_zero() {
        let (_c, pool) = start_pg().await;
        let client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        let rank = get_player_inner(&client, PlayerId::new(404)).await.unwrap();
        assert_eq!(rank, 0, "unknown player should report default rank 0");
    }

    #[tokio::test]
    async fn get_player_inner_reflects_set() {
        let (_c, pool) = start_pg().await;
        let mut client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![rank(1, "Recruit"), rank(5, "Sergeant")])
            .await
            .unwrap();

        let player = PlayerId::new(42);
        set_player_inner(&mut client, player, 5).await.unwrap();

        let r = get_player_inner(&client, player).await.unwrap();
        assert_eq!(r, 5);
    }

    // -- set_player_inner --

    #[tokio::test]
    async fn set_player_inner_autocreates_player() {
        let (_c, pool) = start_pg().await;
        let mut client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![rank(1, "Recruit")])
            .await
            .unwrap();

        let player = PlayerId::new(7);
        set_player_inner(&mut client, player, 1).await.unwrap();

        let info_exists: bool = client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM skua_master.player_info WHERE steam_id = $1)",
                &[&player],
            )
            .await
            .unwrap()
            .get(0);
        assert!(info_exists);
    }

    #[tokio::test]
    async fn set_player_inner_updates_existing() {
        let (_c, pool) = start_pg().await;
        let mut client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");
        apply_migration(&client, vec![rank(1, "Recruit"), rank(5, "Sergeant")])
            .await
            .unwrap();

        let player = PlayerId::new(7);
        set_player_inner(&mut client, player, 1).await.unwrap();
        set_player_inner(&mut client, player, 5).await.unwrap();

        let r = get_player_inner(&client, player).await.unwrap();
        assert_eq!(r, 5);
    }

    #[tokio::test]
    async fn set_player_inner_unknown_rank_fails() {
        let (_c, pool) = start_pg().await;
        let mut client = pool.get().await.unwrap();
        bootstrap_schema(&client).await.expect("schema");

        // Only the seeded default rank (0) exists. rank=99 violates the FK.
        let result = set_player_inner(&mut client, PlayerId::new(7), 99).await;
        assert!(result.is_err());
    }
}
