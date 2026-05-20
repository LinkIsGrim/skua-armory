#include "script_component.hpp"

if (!isServer) exitWith {};

[QGVAR(saveObject), LINKFUNC(saveObject_position)] call CBA_fnc_addEventHandler;
[QGVAR(saveUnit), LINKFUNC(saveUnit_loadout)] call CBA_fnc_addEventHandler;
[QGVAR(saveUnit), LINKFUNC(saveUnit_medical)] call CBA_fnc_addEventHandler;

addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback)];

// Load on connect: QGVAR(playerReady) is from the database addon and fires
// once per player after the player_info upsert completes.
[QEGVAR(database,playerReady), LINKFUNC(loadPlayerLoadout)] call CBA_fnc_addEventHandler;

// Save on disconnect: HandleDisconnect runs server-side just before the unit
// is cleaned up — capture the loadout while the unit still exists.
addMissionEventHandler ["HandleDisconnect", {
    params ["_unit", "", "_uid"];
    [_unit, _uid] call FUNC(savePlayerLoadout);
    false
}];
