#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

[QGVAR(setUnitTrait), {
    params ["_unit", "_trait", "_value"];

    _unit setUnitTrait [_trait, _value];
}] call CBA_fnc_addEventHandler;

[QGVAR(setCamoCoef), {
    params ["_unit", "_camoCoef"];

    _unit setUnitTrait ["camouflageCoef", sqrt(_camoCoef)];
}] call CBA_fnc_addEventHandler;

GVAR(camoCoefMap) = createHashMap;

["unit", LINKFUNC(handlePlayerChanged)] call CBA_fnc_addEventHandler;

ADDON = true;