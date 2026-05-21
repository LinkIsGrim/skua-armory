#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Dialog onLoad handler for the Admin Menu. Wires up:
 *  - LBSelChanged on the player listbox → refresh whichever tab is active
 *  - LBSelChanged on the cert listboxes → action-button sync (Certs tab)
 *  - "Online only" checkbox default state (checked)
 *  - KeyDown handler for Ctrl-C → copy selected Steam UID
 *  - QGVAR(addonMapLoaded) listener so disconnect-driven cache invalidations
 *    and the initial fetch land back in the UI
 *  - Initial roster fetch via the server
 *  - Default active tab (Certifications)
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
_display setVariable [QGVAR(activeTab), ADMIN_TAB_CERTS];

private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;
_playerList ctrlAddEventHandler ["LBSelChanged", {
    call FUNC(refreshCertLists);
    call FUNC(refreshAddonLists);
}];

// Track which cert listbox was last clicked so BtnAction / BtnActionTemp
// know whether to read from Held (revoke) or Available (grant). The action
// buttons' text + enable state syncs on each click.
private _heldList = _display displayCtrl IDC_ADMINCERT_HELD_LIST;
_heldList ctrlAddEventHandler ["LBSelChanged", {
    private _display = findDisplay IDD_ADMIN_MENU;
    _display setVariable [QGVAR(focusedCertList), IDC_ADMINCERT_HELD_LIST];
    call FUNC(updateActionButtons);
}];
private _availList = _display displayCtrl IDC_ADMINCERT_AVAILABLE_LIST;
_availList ctrlAddEventHandler ["LBSelChanged", {
    private _display = findDisplay IDD_ADMIN_MENU;
    _display setVariable [QGVAR(focusedCertList), IDC_ADMINCERT_AVAILABLE_LIST];
    call FUNC(updateActionButtons);
}];

// Selecting an Extra Mods entry repopulates Extra Addons with that mod's
// addons. The "<unresolved>" pseudo-entry routes the same way.
private _extraModsList = _display displayCtrl IDC_ADMINMENU_EXTRA_MODS_LIST;
_extraModsList ctrlAddEventHandler ["LBSelChanged", {call FUNC(refreshExtraAddons)}];

private _onlineOnly = _display displayCtrl IDC_ADMINCERT_CHK_ONLINE_ONLY;
_onlineOnly cbSetChecked true;

_display displayAddEventHandler ["KeyDown", {_this call FUNC(onAdminMenuKeyDown)}];

// Kick off the historical-roster fetch. Even with the online-only filter on
// by default, the roster is needed so toggling the filter doesn't have to
// round-trip first. The cached list arrives via QGVAR(rosterPushed).
call FUNC(fetchPlayerRoster);

// Apply default tab visibility once controls are settled.
[{[_this, ADMIN_TAB_CERTS] call FUNC(switchAdminTab)}, _display] call CBA_fnc_execNextFrame;
[{call FUNC(refreshPlayerList)}] call CBA_fnc_execNextFrame;
