["skua_cert_doctor", {
    params ["_unit"];

    _unit setVariable ["ace_medical_medicClass", 2, true];

    _unit setVariable [QGVAR(doctor), true, true];

    _unit setVariable [QGVAR(medic), true, true];
}] call CBA_fnc_addEventHandler;
