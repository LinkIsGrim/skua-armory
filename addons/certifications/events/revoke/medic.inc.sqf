["skua_cert_revoke_medic", {
    params ["_unit"];

    private _currentLevel = _unit getVariable ["ace_medical_medicClass", 0];
    if (_currentLevel == 1) then {
        _unit setVariable ["ace_medical_medicClass", 0, true];
    } else {
        INFO_1("Unit %1 does not have medic certification level 1, skipping revoke",name _unit);
        _unit setVariable ["ace_medical_medicClass", _currentLevel, true]; // Ensure variable is set globally
    };
}] call CBA_fnc_addEventHandler;
