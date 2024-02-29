#include "..\script_component.hpp"

params ["_unit"];

if (_unit getVariable QGVAR(equipped) != "packMule") exitWith {};

["loadout", GVAR(packMulePlayerEH)] call CBA_fnc_removePlayerEventHandler;
[_unit, _unit, GVAR(packMuleIgnoredWeight)] call ACEFUNC(movement,addLoadToUnitContainer);
GVAR(packMulePlayerEH) = nil;
GVAR(packMuleIgnoredWeight) = nil;
