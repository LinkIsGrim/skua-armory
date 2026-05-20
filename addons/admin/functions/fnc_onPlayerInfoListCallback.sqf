#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Receives the decoded `player_info:list` payload (array of
 * HashMaps with "steam_id" / "name" keys). Caches it in uiNamespace under
 * QGVAR(playerRoster) and fires a local QGVAR(rosterLoaded) event so the
 * admin menu refreshes if open.
 *
 * Arguments:
 * 0: Roster <ARRAY of HASHMAP>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_roster"];

uiNamespace setVariable [QGVAR(playerRoster), _roster];
TRACE_1("roster cached",count _roster);
[QGVAR(rosterLoaded)] call CBA_fnc_localEvent;
