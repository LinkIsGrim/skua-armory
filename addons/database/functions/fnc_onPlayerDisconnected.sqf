#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * CBA event handler for QEV_PLAYER_DISCONNECTED. Currently a placeholder for
 * downstream cleanup consumers (shop cart abandonment, inventory persistence
 * close-out, etc.) — the engine's own HandleDisconnect drives the
 * fnc_playerDisconnect ext call; this event surfaces "the DB acknowledged it".
 *
 * Arguments:
 * 0: Payload <HASHMAP>
 *   "player_id": <STRING> Steam ID
 *
 * Return Value:
 * None.
 *
 * Public: No
 */
params ["_data"];

private _playerID = _data get "player_id";

TRACE_1("Player disconnected event acknowledged",_playerID);
