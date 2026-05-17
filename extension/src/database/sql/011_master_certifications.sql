-- Certifications table - stores available certifications.
-- Rows are reconciled from `database/migrations/certifications/*.json` at
-- bootstrap; `created_at` is compared against
-- `skua_master.migration_state.last_migration_at` to decide whether a row
-- without a backing file should be deleted.
CREATE TABLE IF NOT EXISTS skua_master.certifications (
    id              TEXT PRIMARY KEY,
    display_name    TEXT NOT NULL UNIQUE,
    document        TEXT NOT NULL,
    description     TEXT NOT NULL,
    perk            TEXT NOT NULL,
    pay_bonus       INTEGER NOT NULL DEFAULT 0,
    grant_event     TEXT NOT NULL,
    revoke_event    TEXT NOT NULL,
    requires        TEXT[] NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
