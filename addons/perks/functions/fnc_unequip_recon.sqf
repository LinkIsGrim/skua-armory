#include "..\script_component.hpp"

params ["_unit"];

if (_unit getVariable QGVAR(equipped) != "recon") exitWith {};

[QEGVAR(common,setCamoCoef), [_unit, (uniform _unit) call FUNC(getUniformVisibility)]] call CBA_fnc_localEvent;