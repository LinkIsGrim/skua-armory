#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Client-side: asks the server for the player roster. Used at dialog open
 * (initial fetch) and from the refresh icon button. The server handles the
 * extension call and global-broadcasts the result; this client receives it
 * via fnc_onPlayerInfoListCallback through the QGVAR(rosterPushed) handler.
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

if (!hasInterface) exitWith {};
TRACE_1("requesting player roster",diag_tickTime);
[QGVAR(fetchRosterRequest), []] call CBA_fnc_serverEvent;
