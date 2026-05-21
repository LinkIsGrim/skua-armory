-- Forward-compat migrations for skua_master.campaigns.
-- Each ALTER must be idempotent (IF NOT EXISTS) so re-running on a fresh DB
-- after 012_master_campaigns.sql already added the column is a no-op.

ALTER TABLE skua_master.campaigns
    ADD COLUMN IF NOT EXISTS default_loadout TEXT;
