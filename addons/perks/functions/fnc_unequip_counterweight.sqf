#include "..\script_component.hpp"

params ["_unit"];

if (_unit getVariable QGVAR(equipped) != "counterweight") exitWith {};

["loadout", GVAR(counterweightPlayerEH)] call CBA_fnc_removePlayerEventHandler;
[_unit, _unit, -GVAR(counterweightPenalty)] call ACEFUNC(movement,addLoadToUnitContainer);
GVAR(counterweightPlayerEH) = nil;
