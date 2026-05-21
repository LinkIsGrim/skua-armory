#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Wired to BtnAction / BtnActionTemp. Reads which cert listbox the admin
 * clicked last from the display's QGVAR(focusedCertList) variable and queues
 * the matching op:
 *   - Held last clicked    → revoke
 *   - Available last clicked → grant
 *
 * No-op if no cert listbox has been focused yet, or if the focused listbox
 * has no selection. UI button-enable state already gates this in practice
 * (fnc_updateActionButtons), but the guard is here too for safety.
 *
 * Arguments:
 * 0: Temp <BOOL> (false = persistent, true = temp)
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params [["_temp", false, [false]]];

private _display = findDisplay IDD_ADMIN_MENU;
if (isNull _display) exitWith {};

private _focused = _display getVariable [QGVAR(focusedCertList), -1];
private _op = switch (_focused) do {
    case IDC_ADMINCERT_HELD_LIST:      {"revoke"};
    case IDC_ADMINCERT_AVAILABLE_LIST: {"grant"};
    default {""};
};

if (_op isEqualTo "") exitWith {
    INFO("Action clicked with no focused cert list");
};

[_op, _focused, !_temp] call FUNC(queueCertChange);
