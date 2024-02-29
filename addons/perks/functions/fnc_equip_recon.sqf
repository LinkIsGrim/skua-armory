#include "..\script_component.hpp"

params ["_unit"];

if (_unit getVariable QGVAR(equipped) == "recon") exitWith {};

if (([_unit, "HitChest"] call FUNC(getArmorLevel)) < RECON_MAX_ARMOR_LEVEL) then {
    [QEGVAR(common,setCamoCoef), [_unit, (uniform _unit) call FUNC(getUniformVisibility)]] call CBA_fnc_localEvent;
};