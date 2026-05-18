["skua_cert_revoke_mechanic", {
    params ["_unit"];

    private _isCombatEngineer = _unit getVariable [QGVAR(combatEngineer), false];
    if (_isCombatEngineer) exitWith {
        INFO_1("Unit %1 has combat engineer certification, skipping revoke of mechanic certification. Revoke combat engineer first. Good luck with that.",name _unit);
    };

    _unit setVariable [QGVAR(mechanic), false, true];

    _unit setVariable ["ace_isEngineer", 0, true]; // Reset ACE Engineer level to 0 on revoke
}] call CBA_fnc_addEventHandler;
