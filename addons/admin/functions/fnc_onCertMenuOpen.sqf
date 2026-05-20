#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Dialog onLoad handler. Wires the player listbox lbSelChanged handler and
 * triggers the initial population. We can't rely on `findDisplay IDD` here —
 * it sometimes returns null mid-load — so the first refresh runs on the next
 * frame, by which point findDisplay resolves.
 *
 * Arguments:
 * 0: Display <DISPLAY>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_display"];

_display setVariable [QGVAR(pendingChanges), createHashMap];

private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;
_playerList ctrlAddEventHandler ["LBSelChanged", {call FUNC(refreshCertLists)}];

[{call FUNC(refreshCertMenu)}] call CBA_fnc_execNextFrame;
