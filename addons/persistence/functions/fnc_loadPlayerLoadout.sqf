#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Subscribed to QGVAR(playerReady) from the database addon (fires once per
 * player connect, post-bootstrap). If loadout persistence is enabled and a
 * campaign key is configured, asks the extension for the stored loadout.
 * The response lands on skua:loadout/get_player and is routed by
 * fnc_extCallback.
 *
 * Arguments:
 * 0: The player object <OBJECT>
 * 1: PlayerInfo payload <HASHMAP>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */
params ["_player", "_info"];

if (GVAR(loadoutMode) == 0) exitWith {};
if (EGVAR(database,campaignKey) isEqualTo "") exitWith {
    TRACE_1("loadout: skipping fetch, no campaign key configured",_info);
};

private _uid = _info get "steam_id";
"skua" callExtension ["loadout:get_player", [EGVAR(database,campaignKey), _uid]];
TRACE_2("loadout: fetch requested",_uid,EGVAR(database,campaignKey));
