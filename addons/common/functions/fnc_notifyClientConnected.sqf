#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Notify the server this client has connected. May run for self-hosted servers.
 * Delayed by a frame, safe to call in postInit.
 *
 * Return Value:
 * None
 *
 * Example:
 * call skua_common_fnc_notifyClientConnected;
 *
 * Public: No
 */

if (isDedicated) exitWith {};
// Singleplayer's getPlayerUID returns "_SP_PLAYER_"; no point notifying since
// the database/persistence chain skips bootstrap in SP anyway.
if (!isMultiplayer) exitWith {};

[{[QGVAR(clientConnected), [getPlayerUID player, hasInterface, player]] call CBA_fnc_serverEvent}] call CBA_fnc_execNextFrame;
