-- Tracks the last time each file-driven entity type was reconciled with its
-- migration files. The migration logic uses this to distinguish rows that
-- should be deleted (no file, predates last sync) from rows that should be
-- preserved (no file, but inserted in-game after the last sync).
CREATE TABLE IF NOT EXISTS skua_master.migration_state (
    entity_type        TEXT PRIMARY KEY,
    last_migration_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
