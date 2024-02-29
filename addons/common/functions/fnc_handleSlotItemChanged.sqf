#include "..\script_component.hpp"

params ["_unit", "_name", "_slot", "_assigned"];

if (_slot != TYPE_UNIFORM) exitWith {};
if (!_assigned) exitWith {
    _unit setUnitTrait ["camouflageCoef", 1];
};

[QEGVAR(common,setCamoCoef), [_unit, (uniform _unit) call FUNC(getUniformVisibility)]] call CBA_fnc_localEvent;