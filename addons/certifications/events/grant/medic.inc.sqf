["skua_cert_medic", {
    params ["_unit"];

    private _currentLevel = _unit getVariable ["ace_medical_medicClass", 0];
    if (_currentLevel >= 1) exitWith {
        INFO_2("Unit %1 already has medic certification level %2 (doctor or forced through mission), skipping",name _unit,_currentLevel);
        _unit setVariable ["ace_medical_medicClass", _currentLevel, true]; // Ensure variable is set globally
    };

    _unit setVariable ["ace_medical_medicClass", 1, true];
}] call CBA_fnc_addEventHandler;
