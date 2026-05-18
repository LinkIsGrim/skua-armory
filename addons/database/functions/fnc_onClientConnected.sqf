#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Fired from QEGVAR(common,clientConnected) server-side when a client reports
 * being in-game. Defers the player_connect call via runAfterDatabaseInit so
 * connections arriving before bootstrap completes are queued and replayed
 * once QGVAR(initialized) fires.
 *
 * Arguments:
 * 0: Player UID <STRING>
 * 1: Has interface <BOOL>
 * 2: Player unit <OBJECT>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_uid", "_hasInterface", "_player"];

if (!_hasInterface) exitWith {};

[FUNC(playerConnect), [_uid, _hasInterface, _player]] call FUNC(runAfterDatabaseInit);
