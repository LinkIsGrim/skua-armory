#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Server-only handler for QGVAR(fetchRosterRequest). Fires the
 * `player_info:list` extension call; the result lands on `skua:player_info`
 * in fnc_extCallback which then global-broadcasts the parsed roster to
 * every machine via QGVAR(rosterPushed).
 *
 * No per-client tracking — the response is broadcast to all hasInterface
 * clients on QGVAR(rosterPushed) and non-admins simply have no UI to update.
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
