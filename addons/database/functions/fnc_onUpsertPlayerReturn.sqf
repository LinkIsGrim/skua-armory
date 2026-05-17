#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Handles the extension callback for database:upsert_player. Parses the
 * returned player_info row, attaches it to the player unit (public broadcast),
 * and fires QGVAR(playerReady) so other addons can chain off it.
 *
 * Arguments:
 * 0: Data returned from the extension <STRING>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_data"];
TRACE_1("fnc_onUpsertPlayerReturn",_this);

(parseSimpleArray _data) params ["_status", "_return"];

if (_status != QUERYSTATE_DONE) exitWith {
    ERROR_2("Player UPSERT failed: %1: %2",_status,_return);
};

private _info = fromJSON _return;
private _uid = _info get "steam_id";
private _player = _uid call BIS_fnc_getUnitByUID;

if (isNull _player) exitWith {
    WARNING_1("UPSERT returned for unknown player UID %1; dropping.",_uid);
};

_player setVariable [QGVAR(info), _info, true];
[QGVAR(playerReady), [_player, _info]] call CBA_fnc_globalEventJIP;
