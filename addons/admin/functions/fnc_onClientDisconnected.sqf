#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Admin-client side of the disconnect fan-out. Prunes the cached client
 * addon map (uiNamespace) so the Addon List tab doesn't show stale entries
 * for someone who just left.
 *
 * The server broadcasts this from its own PlayerDisconnected mission EH —
 * we don't rely on QEV_PLAYER_DISCONNECTED because that's gated on the
 * database extension being up, and admin functionality must work without
 * the database.
 *
 * Arguments:
 * 0: Disconnected client UID <STRING>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_uid"];

private _map = uiNamespace getVariable [QGVAR(clientAddonMap), nil];
if (isNil "_map") exitWith {}; // Never opened the Addons tab — nothing cached.

_map deleteAt _uid;

// If the admin menu is open, redraw — selection may still point at the now-gone player.
if (!isNull (findDisplay IDD_ADMIN_MENU)) then {
    [QGVAR(addonMapLoaded)] call CBA_fnc_localEvent;
};
