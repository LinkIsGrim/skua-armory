#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Invoked from the HandleDisconnect mission EH. Captures the player's current
 * loadout (before the engine destroys the unit) and pushes it to the
 * extension. No-op unless persistence mode is Read+Write and a campaign key
 * is configured.
 *
 * Arguments:
 * 0: The departing unit <OBJECT>
 * 1: The player UID <STRING>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */
params ["_unit", "_uid"];

if (GVAR(loadoutMode) != 2) exitWith {};
if (EGVAR(database,campaignKey) isEqualTo "") exitWith {
    TRACE_1("loadout: skipping save, no campaign key configured",_uid);
};
if (isNull _unit) exitWith {
    TRACE_1("loadout: skipping save, null unit",_uid);
};

private _loadout = _unit call CBA_fnc_getLoadout;
"skua" callExtension ["loadout:set_player", [EGVAR(database,campaignKey), _uid, _loadout]];
TRACE_2("loadout: save requested",_uid,EGVAR(database,campaignKey));
