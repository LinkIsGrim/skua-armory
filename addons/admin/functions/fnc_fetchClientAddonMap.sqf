#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Ask the server for the full client addon map (every connected client's
 * extras/missing diff). Result is cached in uiNamespace so the admin menu's
 * Addon List tab doesn't round-trip when switching between players.
 *
 * Must be run scheduled.
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * None — result lands in uiNamespace at QGVAR(clientAddonMap) via
 * fnc_onClientAddonMapCallback, and QGVAR(addonMapLoaded) is fired.
 *
 * Example:
 * [] spawn skua_admin_fnc_fetchClientAddonMap;
 *
 * Public: No
 */

[QGVAR(requestClientAddonMap), [getPlayerUID player, player]] call CBA_fnc_serverEvent;
