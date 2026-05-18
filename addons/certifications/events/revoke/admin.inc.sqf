["skua_cert_revoke_admin", {
    params ["_unit"];
    _unit setVariable [QGVAR(admin), false, true];
}] call CBA_fnc_addEventHandler;
