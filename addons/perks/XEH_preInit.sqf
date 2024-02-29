#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

#include "initSettings.inc.sqf"

[QEGVAR(common,setCamoCoef), 0] call CBA_fnc_removeEventHandler;

[QEGVAR(common,setCamoCoef), {
    params ["_unit", "_camoCoef"];

    if (
        _unit getVariable QGVAR(equipped) == "recon" &&
        {([_unit, "HitChest"] call FUNC(getArmorLevel)) < RECON_MAX_ARMOR_LEVEL}
    ) then {
        _camoCoef = _camoCoef * RECON_VISIBILITY_COEF;
    };

    _unit setUnitTrait ["camouflageCoef", sqrt(_camoCoef)];
}] call CBA_fnc_addEventHandler;

ADDON = true;