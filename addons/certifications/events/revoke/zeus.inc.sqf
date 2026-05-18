["skua_cert_revoke_zeus", {
    params ["_unit"];

    _unit setVariable [QGVAR(zeus), false, true];
}] call CBA_fnc_addEventHandler;
