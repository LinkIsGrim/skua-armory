-- Ranks table - stores available ranks.
-- Rows are reconciled from `database/migrations/ranks/*.json` at bootstrap;
-- `created_at` is compared against `skua_master.migration_state.last_migration_at`
-- to decide whether a row without a backing file should be deleted (rows
-- inserted in-game after the last sync are preserved).
CREATE TABLE IF NOT EXISTS skua_master.ranks (
    id              SMALLINT PRIMARY KEY,
    display_name    TEXT NOT NULL UNIQUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
