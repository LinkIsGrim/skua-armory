#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Wired to the Grant / Grant (temp) buttons. Queues the change locally on
 * the display; nothing leaves the client until Commit is clicked.
 *
 * Arguments:
 * 0: Temp grant <BOOL> (false = persistent, true = temp)
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params [["_temp", false, [false]]];

["grant", IDC_ADMINCERT_AVAILABLE_LIST, !_temp] call FUNC(queueCertChange);
