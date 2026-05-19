#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * CBA event handler for QEV_RANK_CHANGED. Fired when the extension's
 * `ranks:set_player` succeeds. Currently a placeholder — downstream consumers
 * (UI updates, audit log) subscribe to QEV_RANK_CHANGED directly rather than
 * routing through this handler.
 *
 * Arguments:
 * 0: Payload <HASHMAP>
 *   "player_id": <STRING> Steam ID
 *   "rank_id":   <NUMBER> SMALLINT rank id matching GVAR(map)
 *
 * Return Value:
 * None.
 *
 * Public: No
 */
params ["_data"];

private _playerID = _data get "player_id";
private _rankID = _data get "rank_id";

TRACE_2("Rank changed",_playerID,_rankID);
