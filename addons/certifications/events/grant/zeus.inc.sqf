["skua_cert_zeus", {
    params ["_unit"];

    _unit setVariable [QGVAR(zeus), true, true];
}] call CBA_fnc_addEventHandler;
