#include "..\script_component.hpp"

params ["_display"];

if (is3DEN) exitWith {};

GVAR(loadoutEH) = ["loadout", FUNC(handleLoadout), true] call CBA_fnc_addPlayerEventHandler;
GVAR(arsenalDisplay) = [_display];

private _loadout = getUnitLoadout ACE_player;
[ACE_player, _loadout, _loadout] call FUNC(handleLoadout);
