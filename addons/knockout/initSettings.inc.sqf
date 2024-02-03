[
    QGVAR(knockOutPlayers), "LIST",
    ["Knock Out Players", "Determines who has access to knocking out players."],
    ["Skua Mods", "Knockout"],
    [[2,1,0], ["Disabled", "Admins Only", "Everyone"], 1],
    true
] call CBA_fnc_addSetting;

[
    QGVAR(knockOutAI), "LIST",
    ["Knock Out AI", "Determines who has access to knocking out AI."],
    ["Skua Mods", "Knockout"],
    [[2,1,0], ["Disabled", "Admins Only", "Everyone"], 2],
    true
] call CBA_fnc_addSetting;

[
    QGVAR(timePlayers), "TIME",
    "Player Knockout Time",
    ["Skua Mods", "Knockout"],
    [1, 90, 30],
    false
] call CBA_fnc_addSetting;

[
    QGVAR(timeAI), "TIME",
    "AI Knockout Time",
    ["Skua Mods", "Knockout"],
    [1, 90, 30],
    false
] call CBA_fnc_addSetting;
