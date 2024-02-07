[
    QGVAR(enabledRemoteSensors),
    "CHECKBOX",
    [LSTRING(EnabledRemoteSensors), LSTRING(EnabledRemoteSensorsDesc)],
    ["Skua Mods", "Performance"],
    false,
    true,
    {
        if (isServer || {!hasInterface}) exitWith {};
        disableRemoteSensors !_this;
        INFO_1("disableRemoteSensors %1",!_this);
    }
] call CBA_fnc_addSetting;
