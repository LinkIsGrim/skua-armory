["skua_cert_mechanic", {
    params ["_unit"];

    _unit setVariable [QGVAR(mechanic), true, true];

    // Check if they're a combat engineer
    private _isCombatEngineer = _unit getVariable [QGVAR(combatEngineer), false];
    if (_isCombatEngineer) exitWith {
        INFO_1("Unit %1 already has combat engineer certification, skipping mechanic certification",name _unit);
    };

    // ACE Engineer level 1
    _unit setVariable ["ace_isEngineer", 1, true];
}] call CBA_fnc_addEventHandler;
