//! File-driven reconciliation of `skua_master.certifications`.

use include_dir::{Dir, include_dir};
use tokio_postgres::Client;
use tracing::{info, instrument};

use super::types::CertificationFile;
use crate::error::{QueryResult, transient_error};

static CERT_FILES: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/../database/migrations/certifications");

const ENTITY_TYPE: &str = "certification";

/// Read embedded files + reconcile against the live DB. Called once per
/// bootstrap from [`crate::database::schema::bootstrap_master`].
#[instrument(level = "debug", name = "certification_migrate", skip(client))]
pub(crate) async fn migrate(client: &Client) -> Result<(), QueryResult> {
    let files = load_files().map_err(|msg| {
        transient_error(
            "Failed to load certification migration files",
            std::io::Error::other(msg),
        )
    })?;
    apply_migration(client, files).await
}

/// Walk the embedded `database/migrations/certifications/` dir, parse every
/// `*.json` file, and return `(id, content)` pairs sorted by id for
/// deterministic ordering.
pub(super) fn load_files() -> Result<Vec<(String, CertificationFile)>, String> {
    load_files_from_dir(&CERT_FILES)
}

pub(super) fn load_files_from_dir(
    dir: &Dir<'_>,
) -> Result<Vec<(String, CertificationFile)>, String> {
    let mut entries: Vec<(String, CertificationFile)> = Vec::new();
    for file in dir.files() {
        let path = file.path();
        if path.extension().is_none_or(|e| e != "json") {
            continue;
        }
        let id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("invalid filename: {}", path.display()))?
            .to_owned();
        let content = file
            .contents_utf8()
            .ok_or_else(|| format!("non-UTF8 file: {}", path.display()))?;
        let parsed: CertificationFile = serde_json::from_str(content)
            .map_err(|e| format!("parse {}: {}", path.display(), e))?;
        entries.push((id, parsed));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(entries)
}

/// Reconcile `files` into `skua_master.certifications`:
///   1. UPSERT every file's contents (preserves `created_at` on updates).
///   2. DELETE rows whose id isn't in `files` AND whose `created_at` predates
///      the last successful migration (preserves in-game additions).
///   3. Bump the `migration_state` marker.
///
/// Each statement is atomic on its own; if the process dies mid-way the next
/// bootstrap will resume cleanly.
#[instrument(level = "debug", name = "certification_apply", skip(client, files))]
pub(super) async fn apply_migration(
    client: &Client,
    files: Vec<(String, CertificationFile)>,
) -> Result<(), QueryResult> {
    let upsert_stmt = "
        INSERT INTO skua_master.certifications
            (id, display_name, document, grant_event, revoke_event)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE
            SET display_name = EXCLUDED.display_name,
                document     = EXCLUDED.document,
                grant_event  = EXCLUDED.grant_event,
                revoke_event = EXCLUDED.revoke_event";

    for (id, file) in &files {
        if let Err(e) = client
            .execute(
                upsert_stmt,
                &[
                    &id,
                    &file.display_name,
                    &file.document,
                    &file.grant_event,
                    &file.revoke_event,
                ],
            )
            .await
        {
            return Err(transient_error(
                &format!("Failed to upsert certification {id}"),
                e,
            ));
        }
    }

    let file_ids: Vec<&str> = files.iter().map(|(id, _)| id.as_str()).collect();
    let delete_stmt = "
        DELETE FROM skua_master.certifications
        WHERE NOT (id = ANY($1::text[]))
          AND created_at <= COALESCE(
              (SELECT last_migration_at FROM skua_master.migration_state
               WHERE entity_type = $2),
              '-infinity'::timestamptz
          )";
    if let Err(e) = client
        .execute(delete_stmt, &[&file_ids, &ENTITY_TYPE])
        .await
    {
        return Err(transient_error("Failed to prune certifications", e));
    }

    let bump_stmt = "
        INSERT INTO skua_master.migration_state (entity_type, last_migration_at)
        VALUES ($1, NOW())
        ON CONFLICT (entity_type) DO UPDATE
            SET last_migration_at = EXCLUDED.last_migration_at";
    if let Err(e) = client.execute(bump_stmt, &[&ENTITY_TYPE]).await {
        return Err(transient_error(
            "Failed to bump certification migration_state",
            e,
        ));
    }

    info!(count = files.len(), "certifications reconciled");
    Ok(())
}
