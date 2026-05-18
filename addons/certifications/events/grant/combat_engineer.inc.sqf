["skua_cert_combat_engineer", {
    params ["_unit"];

    _unit setVariable [QGVAR(combatEngineer), true, true];
    _unit setVariable [QGVAR(mechanic), true, true];
    _unit setVariable [QGVAR(eod), true, true];

    // ACE Engineer level 2
    _unit setVariable ["ace_isEngineer", 2, true];

    // ACE Explosives Specialist
    _unit setVariable ["ace_isEOD", 1, true];
}] call CBA_fnc_addEventHandler;
