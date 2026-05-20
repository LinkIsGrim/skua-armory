#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Dialog onLoad handler. Wires up:
 *  - LBSelChanged on the player listbox → cert listbox refresh
 *  - "Online only" checkbox default state (checked)
 *  - KeyDown handler for Ctrl-C → copy selected Steam UID
 *  - Initial roster fetch via the server
 *
 * The first refresh runs on the next frame so `findDisplay IDD` resolves
 * cleanly (it sometimes returns null mid-load).
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
_display setVariable [QGVAR(focusedCertList), -1];

private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;
_playerList ctrlAddEventHandler ["LBSelChanged", {call FUNC(refreshCertLists)}];

// Track which cert listbox was last clicked so BtnAction / BtnActionTemp
// know whether to read from Held (revoke) or Available (grant). The action
// buttons' text + enable state syncs on each click.
private _heldList = _display displayCtrl IDC_ADMINCERT_HELD_LIST;
_heldList ctrlAddEventHandler ["LBSelChanged", {
    private _display = findDisplay IDD_ADMIN_CERT_MENU;
    _display setVariable [QGVAR(focusedCertList), IDC_ADMINCERT_HELD_LIST];
    call FUNC(updateActionButtons);
}];
private _availList = _display displayCtrl IDC_ADMINCERT_AVAILABLE_LIST;
_availList ctrlAddEventHandler ["LBSelChanged", {
    private _display = findDisplay IDD_ADMIN_CERT_MENU;
    _display setVariable [QGVAR(focusedCertList), IDC_ADMINCERT_AVAILABLE_LIST];
    call FUNC(updateActionButtons);
}];

private _onlineOnly = _display displayCtrl IDC_ADMINCERT_CHK_ONLINE_ONLY;
_onlineOnly cbSetChecked true;

_display displayAddEventHandler ["KeyDown", {_this call FUNC(onCertMenuKeyDown)}];

// Kick off the historical-roster fetch. Even with the online-only filter on
// by default, the roster is needed so toggling the filter doesn't have to
// round-trip first. The cached list arrives via QGVAR(rosterPushed).
call FUNC(fetchPlayerRoster);

[{call FUNC(refreshCertMenu)}] call CBA_fnc_execNextFrame;
