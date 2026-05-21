#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Send client's addon list to server on connect. Also includes a per-addon
 * map of source-mod info so the server can surface extras grouped by mod —
 * the server doesn't have these addons loaded and can't resolve their sources
 * locally.
 *
 * Return Value:
 * None
 *
 * Example:
 * call skua_admin_fnc_sendClientAddons;
 *
 * Public: No
 */

if (!hasInterface) exitWith {}; // We don't care about headless clients

private _addons = cba_common_addons; // CBA is reliable
private _modMap = _addons call FUNC(resolveAddonModMap);

[QGVAR(addons), [getPlayerUID player, player, _addons, _modMap]] call CBA_fnc_serverEvent;
