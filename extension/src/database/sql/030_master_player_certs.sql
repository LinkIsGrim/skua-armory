-- Player certifications junction table
CREATE TABLE IF NOT EXISTS skua_master.player_certs (
    steam_id    BIGINT NOT NULL REFERENCES skua_master.player_info(steam_id) ON DELETE CASCADE,
    cert_id     TEXT NOT NULL REFERENCES skua_master.certifications(id) ON DELETE CASCADE,
    granted_by  BIGINT REFERENCES skua_master.player_info(steam_id) ON DELETE SET NULL,
    granted_on  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (steam_id, cert_id)
);
