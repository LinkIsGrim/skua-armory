[
    QGVAR(loadoutMode), "LIST",
    ["Loadout Persistence", "How player loadouts are saved/loaded across sessions. Requires a campaign key in the Database settings."],
    ["Skua Mods", "Loadout Persistence"],
    [[0, 1, 2], ["None", "Read-only", "Read and Write"], 2],
    true // server-forced
] call CBA_fnc_addSetting;
