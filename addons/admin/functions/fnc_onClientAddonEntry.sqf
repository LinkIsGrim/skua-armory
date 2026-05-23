#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Admin-client side of the addon connect fan-out. Merges a single player's
 * addon entry into the cached client addon map so the Addon List tab picks
 * up new joiners without a full re-fetch.
 *
 * Arguments:
 * 0: Player UID <STRING>
 * 1: Addon entry [extras, missing, extrasModMap, missingModMap] <ARRAY>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_uid", "_entry"];

private _map = uiNamespace getVariable [QGVAR(clientAddonMap), nil];
if (isNil "_map") exitWith {};

_map set [_uid, _entry];

if (!isNull (findDisplay IDD_ADMIN_MENU)) then {
    [QGVAR(addonMapLoaded)] call CBA_fnc_localEvent;
};
