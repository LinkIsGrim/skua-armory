["skua_cert_eod", {
    params ["_unit"];

    _unit setVariable [QGVAR(eod), true, true];

    // ACE Explosives Specialist
    _unit setVariable ["ace_isEOD", 1, true];
}] call CBA_fnc_addEventHandler;
