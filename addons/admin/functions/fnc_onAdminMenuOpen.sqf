#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Dialog onLoad handler for the Admin Menu. Builds the tab descriptor
 * registry, wires shared event handlers (player list, KeyDown), kicks off
 * the initial roster fetch, and applies the default tab.
 *
 * Tabs are described declaratively: each descriptor lives on the display at
 * QGVAR(tabs) keyed by tab id, and carries everything fnc_switchAdminTab
 * needs to swap panels in/out plus per-tab lifecycle hooks. Adding a new
 * tab = declare the IDCs + button in the .hpp and push one descriptor here.
 *
 * Descriptor fields (HashMap):
 *   label             STRING - tab button text (switcher prepends "> " when active)
 *   tabBtnIdc         NUMBER - IDC of this tab's button in the bar
 *   panelIdcs         ARRAY<NUMBER> - controls shown/hidden as a group
 *   onRefresh         CODE   - re-fired by the shared player-list LBSelChanged
 *   onActivate        CODE   - runs when this tab becomes active
 *   onDeactivate      CODE   - runs when leaving this tab (counterpart cleanup)
 *   forcesOnlineOnly  BOOL   - tab requires "Online only"; switcher pins/restores
 *   showsRefreshIcon  BOOL   - whether the roster-refresh icon stays visible
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
_display setVariable [QGVAR(activeTab), -1]; // No tab yet; switcher sets it.

// --- Tab registry --------------------------------------------------------
// Each ui/tabs/*.inc.sqf pushes one descriptor into _tabs. New tab = new
// .inc.sqf + line in ui/tabs.hpp.
private _tabs = createHashMap;
#include "..\ui\tabs.hpp"
_display setVariable [QGVAR(tabs), _tabs];

// --- Shared event wiring -------------------------------------------------
private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;
_playerList ctrlAddEventHandler ["LBSelChanged", {
    // Fire every tab's onRefresh — only the active one matters visually, but
    // the others stay in sync for cheap.
    private _display = findDisplay IDD_ADMIN_MENU;
    private _tabs = _display getVariable [QGVAR(tabs), createHashMap];
    {call (_y get "onRefresh")} forEach _tabs;
}];

// "Online only" default state is set in the dialog config (checked = 1);
// per-tab overrides come from each descriptor's forcesOnlineOnly flag.

_display displayAddEventHandler ["KeyDown", {_this call FUNC(onAdminMenuKeyDown)}];

// Kick off the historical-roster fetch.
call FUNC(fetchPlayerRoster);

// Apply default tab visibility once controls are settled.
[{[_this, ADMIN_TAB_CERTS] call FUNC(switchAdminTab)}, _display] call CBA_fnc_execNextFrame;
