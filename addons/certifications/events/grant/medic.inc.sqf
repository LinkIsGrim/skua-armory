["skua_cert_medic", {
    params ["_unit"];
    
    _unit setVariable [QGVAR(medic), true, true];

    private _isDoctor = _unit getVariable [QGVAR(doctor), false];
    if (_isDoctor) exitWith {
        INFO_1("Unit %1 already has doctor certification, skipping medic certification",name _unit);
    };

    private _currentLevel = _unit getVariable ["ace_medical_medicClass", 0];
    if (_currentLevel >= 1) exitWith {
        INFO_2("Unit %1 already has medic certification level %2 (doctor or forced through mission), skipping",name _unit,_currentLevel);
    };

    _unit setVariable ["ace_medical_medicClass", 1, true];
}] call CBA_fnc_addEventHandler;
