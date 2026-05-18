["skua_cert_revoke_doctor", {
    params ["_unit"];

    _unit setVariable [QGVAR(doctor), false, true];

    private _isMedic = _unit getVariable [QGVAR(medic), false];
    
    _unit setVariable ["ace_medical_medicClass", parseNumber _isMedic, true]; // Reset to 1 if medic, 0 if not
}] call CBA_fnc_addEventHandler;
