#include "script_component.hpp"

["CBA_loadoutGet", {
    params ["_unit", "_loadoutArray", "_extendedInfo"];

}] call CBA_fnc_addEventHandler;

["CBA_loadoutSet", {
    params ["_unit", "_loadoutArray", "_extendedInfo"];

}] call CBA_fnc_addEventHandler;

player setVariable [QGVAR(equipped), "", true];

[QGVAR(perkChanged), LINKFUNC(perkChanged)] call CBA_fnc_addEventHandler;

["CAManBase", "Suppressed", {
    params ["_unit", "", "", "_shooter"];
    if (!local _unit) exitWith {};

    if ((_shooter getVariable [QGVAR(equipped), ""]) == "suppression") then {
        _unit setSuppression ((getSuppression _unit) * SUPPRESSION_BONUS_COEF);
    };
}] call CBA_fnc_addClassEventHandler;

GVAR(weaponMassMap) = createHashMap;