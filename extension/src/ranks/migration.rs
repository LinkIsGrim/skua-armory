//! File-driven reconciliation of `skua_master.ranks`.

use include_dir::{Dir, include_dir};
use tokio_postgres::Client;
use tracing::{info, instrument};

use super::types::RankFile;
use crate::error::{QueryResult, transient_error};

static RANK_FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../database/migrations/ranks");

const ENTITY_TYPE: &str = "rank";

/// Read embedded files + reconcile against the live DB. Called once per
/// bootstrap from [`crate::database::schema::bootstrap_master`].
#[instrument(level = "debug", name = "rank_migrate", skip(client))]
pub(crate) async fn migrate(client: &Client) -> Result<(), QueryResult> {
    let files = load_files().map_err(|msg| {
        transient_error(
            "Failed to load rank migration files",
            std::io::Error::other(msg),
        )
    })?;
    apply_migration(client, files).await
}

/// Walk the embedded `database/migrations/ranks/` dir, parse every `*.json`
/// file, return them sorted by id. (Filename is just a sort hint; the
/// authoritative id lives in the JSON body.)
pub(super) fn load_files() -> Result<Vec<RankFile>, String> {
    load_files_from_dir(&RANK_FILES)
}

pub(super) fn load_files_from_dir(dir: &Dir<'_>) -> Result<Vec<RankFile>, String> {
    let mut entries: Vec<RankFile> = Vec::new();
    for file in dir.files() {
        let path = file.path();
        if path.extension().is_none_or(|e| e != "json") {
            continue;
        }
        let content = file
            .contents_utf8()
            .ok_or_else(|| format!("non-UTF8 file: {}", path.display()))?;
        let parsed: RankFile = serde_json::from_str(content)
            .map_err(|e| format!("parse {}: {}", path.display(), e))?;
        entries.push(parsed);
    }
    entries.sort_by_key(|r| r.id);
    Ok(entries)
}

/// Reconcile `files` into `skua_master.ranks`:
///   1. UPSERT every file's contents.
///   2. DELETE rows whose id isn't in `files` AND whose `created_at` predates
///      the last successful migration.
///   3. Bump the `migration_state` marker.
///
/// Default rank (id=0) is seeded by [`crate::database::schema::bootstrap_schema`]
/// before this runs, so an empty file set won't delete it on cold start (its
/// `created_at` equals `NOW()` at seed time, which is also `NOW()` for the
/// first marker bump — the `<=` guard preserves the seed unless the operator
/// explicitly removes it later via in-DB editing).
#[instrument(level = "debug", name = "rank_apply", skip(client, files))]
pub(super) async fn apply_migration(
    client: &Client,
    files: Vec<RankFile>,
) -> Result<(), QueryResult> {
    let upsert_stmt = "
        INSERT INTO skua_master.ranks (id, display_name)
        VALUES ($1, $2)
        ON CONFLICT (id) DO UPDATE
            SET display_name = EXCLUDED.display_name";

    for rank in &files {
        if let Err(e) = client
            .execute(upsert_stmt, &[&rank.id, &rank.display_name])
            .await
        {
            return Err(transient_error(
                &format!("Failed to upsert rank {}", rank.id),
                e,
            ));
        }
    }

    let file_ids: Vec<i16> = files.iter().map(|r| r.id).collect();
    let delete_stmt = "
        DELETE FROM skua_master.ranks
        WHERE NOT (id = ANY($1::smallint[]))
          AND created_at <= COALESCE(
              (SELECT last_migration_at FROM skua_master.migration_state
               WHERE entity_type = $2),
              '-infinity'::timestamptz
          )";
    if let Err(e) = client
        .execute(delete_stmt, &[&file_ids, &ENTITY_TYPE])
        .await
    {
        return Err(transient_error("Failed to prune ranks", e));
    }

    let bump_stmt = "
        INSERT INTO skua_master.migration_state (entity_type, last_migration_at)
        VALUES ($1, NOW())
        ON CONFLICT (entity_type) DO UPDATE
            SET last_migration_at = EXCLUDED.last_migration_at";
    if let Err(e) = client.execute(bump_stmt, &[&ENTITY_TYPE]).await {
        return Err(transient_error("Failed to bump rank migration_state", e));
    }

    info!(count = files.len(), "ranks reconciled");
    Ok(())
}
