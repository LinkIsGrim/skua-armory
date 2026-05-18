["skua_cert_revoke_combat_engineer", {
    params ["_unit"];
    _unit setVariable [QGVAR(combatEngineer), false, true];

    private _isMechanic = _unit getVariable [QGVAR(mechanic), false];
    _unit setVariable ["ace_isEngineer", parseNumber _isMechanic, true]; // Reset ACE Engineer level to 0 on revoke
}] call CBA_fnc_addEventHandler;
