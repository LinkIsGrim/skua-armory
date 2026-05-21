#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Server-only handler for QGVAR(fetchRosterRequest). Fires the
 * `player_info:list` extension call; the result lands on `skua:player_info`
 * in fnc_extCallback which then targets the parsed roster at admin clients
 * only via QGVAR(rosterPushed) (see fnc_adminEvent).
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

"skua" callExtension ["player_info:list", []];
