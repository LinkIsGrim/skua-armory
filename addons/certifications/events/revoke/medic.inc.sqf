["skua_cert_revoke_medic", {
    params ["_unit"];

    private _isDoctor = _unit getVariable [QGVAR(doctor), false];
    if (_isDoctor) exitWith {
        INFO_1("Unit %1 has doctor certification, skipping revoke of medic certification. Revoke doctor first. Good luck with that.",name _unit);
    };

    _unit setVariable [QGVAR(medic), false, true];

    _unit setVariable ["ace_medical_medicClass", 0, true]; // Reset medic class to 0 on revoke
}] call CBA_fnc_addEventHandler;
