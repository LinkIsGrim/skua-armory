[
    QGVAR(campaignKey), "EDITBOX",
    ["Campaign Key", "Schema key for the campaign-specific database. Must be 3-49 chars, [a-z0-9_]. Empty = master-only."],
    ["Skua Mods", "Database"],
    "prod",
    true // global, intentionally
] call CBA_fnc_addSetting;
