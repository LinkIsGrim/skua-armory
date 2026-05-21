#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Switch the Admin Menu's active tab. Toggles visibility on the certs panel
 * vs the addons panel and updates the tab buttons' visual selection state.
 *
 * On switching to the Addons tab: forces "Online only" checked + disabled
 * (offline players can't report their addons by definition), and triggers a
 * bulk fetch of the client addon map if uiNamespace has nothing cached.
 * The map then lands via QGVAR(addonMapLoaded) → fnc_refreshAddonLists.
 *
 * Arguments:
 * 0: Display <DISPLAY>
 * 1: Tab id (ADMIN_TAB_CERTS / ADMIN_TAB_ADDONS) <NUMBER>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_display", "_tab"];

if (isNull _display) exitWith {};

_display setVariable [QGVAR(activeTab), _tab];

private _certs  = _tab isEqualTo ADMIN_TAB_CERTS;
private _addons = _tab isEqualTo ADMIN_TAB_ADDONS;

// Certs panel
{
    (_display displayCtrl _x) ctrlShow _certs;
} forEach [
    IDC_ADMINCERT_HELD_TITLE,
    IDC_ADMINCERT_HELD_LIST,
    IDC_ADMINCERT_AVAILABLE_TITLE,
    IDC_ADMINCERT_AVAILABLE_LIST,
    IDC_ADMINCERT_BTN_ACTION,
    IDC_ADMINCERT_BTN_ACTION_TEMP,
    IDC_ADMINCERT_BTN_COMMIT
];

// Addons panel
{
    (_display displayCtrl _x) ctrlShow _addons;
} forEach [
    IDC_ADMINMENU_MISSING_TITLE,
    IDC_ADMINMENU_MISSING_LIST,
    IDC_ADMINMENU_EXTRA_MODS_TITLE,
    IDC_ADMINMENU_EXTRA_MODS_LIST,
    IDC_ADMINMENU_EXTRA_ADDONS_TITLE,
    IDC_ADMINMENU_EXTRA_ADDONS_LIST
];

// Online-only filter is forced on the Addons tab (offline players have no
// reported addons). Save the previous state on first switch into Addons so
// switching back restores user intent.
private _onlineOnly = _display displayCtrl IDC_ADMINCERT_CHK_ONLINE_ONLY;
private _refresh    = _display displayCtrl IDC_ADMINCERT_BTN_REFRESH;
if (_addons) then {
    if (isNil {_display getVariable QGVAR(savedOnlineOnly)}) then {
        _display setVariable [QGVAR(savedOnlineOnly), cbChecked _onlineOnly];
    };
    _onlineOnly cbSetChecked true;
    _onlineOnly ctrlEnable false;
    // Refresh icon is roster-only; addon map is event-invalidated.
    _refresh ctrlShow false;
} else {
    if (!isNil {_display getVariable QGVAR(savedOnlineOnly)}) then {
        _onlineOnly cbSetChecked (_display getVariable QGVAR(savedOnlineOnly));
        _display setVariable [QGVAR(savedOnlineOnly), nil];
    };
    _onlineOnly ctrlEnable true;
    _refresh ctrlShow true;
};

// Tab-button selection chrome — leading "> " marks the active tab.
private _certBtn   = _display displayCtrl IDC_ADMINMENU_BTN_TAB_CERTS;
private _addonsBtn = _display displayCtrl IDC_ADMINMENU_BTN_TAB_ADDONS;
_certBtn   ctrlSetText (["Certifications", "> Certifications"] select _certs);
_addonsBtn ctrlSetText (["Addon List",    "> Addon List"]    select _addons);

if (_addons) then {
    // Bulk fetch on first switch; otherwise refresh from cache.
    if (isNil {uiNamespace getVariable QGVAR(clientAddonMap)}) then {
        call FUNC(fetchClientAddonMap);
    } else {
        call FUNC(refreshAddonLists);
    };
    // Force a roster refresh so offline entries get filtered out now that
    // online-only is on.
    call FUNC(refreshCertMenu);
} else {
    // Switching back — re-run cert list refresh for the current selection.
    call FUNC(refreshCertMenu);
};
