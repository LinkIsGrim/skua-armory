#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * CBA event handler for QEV_PLAYER_CONNECTED. Receives the upserted
 * `PlayerInfo` row, attaches it to the player unit, and fires QGVAR(playerReady)
 * exactly once per join — duplicate firings (from strand-retry resends) are
 * suppressed by the presence of QGVAR(info) on the unit.
 *
 * Arguments:
 * 0: PlayerInfo payload <HASHMAP>
 *   "steam_id":   <STRING> Steam ID
 *   "name":       <STRING>
 *   "first_seen": <STRING> RFC3339
 *   "last_seen":  <STRING> RFC3339
 *   "is_admin":   <BOOL>
 *   "is_banned":  <BOOL>
 *   "rank":       <NUMBER>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */
params ["_info"];

private _uid = _info get "steam_id";
private _player = _uid call BIS_fnc_getUnitByUID;

if (isNull _player) exitWith {
    WARNING_1("player_connected event for unknown player UID %1; dropping.",_uid);
};

// Idempotency: if a strand-retry resent the call before this event landed,
// the variable is already set — skip playerReady to avoid double-firing
// consumers that are not idempotent.
if (!isNil {_player getVariable QGVAR(info)}) exitWith {
    TRACE_1("Duplicate player_connected for UID %1; already processed",_uid);
};

_player setVariable [QGVAR(info), _info, true];
[QGVAR(playerReady), [_player, _info]] call CBA_fnc_globalEventJIP;
