#include "..\script_component.hpp"

if (is3DEN) exitWith {};

["loadout", GVAR(loadoutEH)] call CBA_fnc_removePlayerEventHandler;
GVAR(loadoutEH) = nil;

GVAR(arsenalDisplay) = nil;
