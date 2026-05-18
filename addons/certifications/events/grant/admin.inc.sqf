["skua_cert_admin", {
    params ["_unit"];

    _unit setVariable [QGVAR(admin), true, true];
}] call CBA_fnc_addEventHandler;
