#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Wired to the Revoke / Revoke (temp) buttons. Queues the change locally on
 * the display; nothing leaves the client until Commit is clicked.
 *
 * Arguments:
 * 0: Temp revoke <BOOL> (false = persistent, true = temp)
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params [["_temp", false, [false]]];

["revoke", IDC_ADMINCERT_HELD_LIST, !_temp] call FUNC(queueCertChange);
