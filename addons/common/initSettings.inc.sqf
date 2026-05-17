[
    QGVAR(extensionLogLevel), "LIST",
    ["Extension Log Level", "Determines the level of logging for the extension."],
    ["Skua Mods", "Logging"],
    [["ERROR", "INFO", "DEBUG", "TRACE"], ["ERROR", "INFO", "DEBUG", "TRACE"], 1],
    true, // global, intentionally
    LINKFUNC(setLogLevelFromSettings) // update log level when setting is changed
] call CBA_fnc_addSetting;
