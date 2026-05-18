["skua_cert_revoke_eod", {
    params ["_unit"];

    private _isCombatEngineer = _unit getVariable [QGVAR(combatEngineer), false];
    if (_isCombatEngineer) exitWith {
        INFO_1("Unit %1 has combat engineer certification, skipping revoke of EOD certification. Revoke combat engineer first. Good luck with that.",name _unit);
    };

    _unit setVariable [QGVAR(eod), false, true];

    _unit setVariable ["ace_isEOD", 0, true]; // Reset ACE Explosives Specialist level to 0 on revoke
}] call CBA_fnc_addEventHandler;
