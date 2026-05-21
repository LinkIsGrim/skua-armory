#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Switch the Admin Menu's active tab. Reads the tab registry built in
 * fnc_onAdminMenuOpen (QGVAR(tabs)) and uniformly:
 *   1. Toggles panel control visibility per descriptor's panelIdcs.
 *   2. Updates each tab button's label chrome (active tab gets a "> " prefix).
 *   3. Applies shared-control state (online-only override, refresh icon)
 *      from the descriptor's flags. Saves/restores the user's prior
 *      "Online only" choice across forced-online transitions.
 *   4. Calls the leaving tab's onDeactivate, then the entering tab's
 *      onActivate.
 *
 * Adding a new tab does not touch this file — push a descriptor in
 * fnc_onAdminMenuOpen and ensure the button + IDCs exist in the .hpp.
 *
 * Arguments:
 * 0: Display <DISPLAY>
 * 1: Tab id <NUMBER>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_display", "_tab"];

if (isNull _display) exitWith {};

private _tabs = _display getVariable [QGVAR(tabs), createHashMap];
if !(_tab in _tabs) exitWith {
    ERROR_1("Unknown admin menu tab id %1",_tab);
};

private _prevTab = _display getVariable [QGVAR(activeTab), -1];
_display setVariable [QGVAR(activeTab), _tab];

// 1 + 2: Panel visibility + tab-button chrome — one pass over the registry.
{
    private _isActive = _x isEqualTo _tab;
    {(_display displayCtrl _x) ctrlShow _isActive} forEach (_y get "panelIdcs");
    private _btn = _display displayCtrl (_y get "tabBtnIdc");
    private _label = _y get "label";
    _btn ctrlSetText ([_label, "> " + _label] select _isActive);
} forEach _tabs;

// 3: Shared controls driven by descriptor flags.
private _activeDesc  = _tabs get _tab;
private _forceOnline = _activeDesc get "forcesOnlineOnly";
private _showRefresh = _activeDesc get "showsRefreshIcon";

private _onlineOnly = _display displayCtrl IDC_ADMINCERT_CHK_ONLINE_ONLY;
private _refresh    = _display displayCtrl IDC_ADMINCERT_BTN_REFRESH;
_refresh ctrlShow _showRefresh;

if (_forceOnline) then {
    // Stash the user's prior choice exactly once so a Cert → Addons → Cert
    // round trip restores correctly even if they toggle Addons multiple times.
    if (isNil {_display getVariable QGVAR(savedOnlineOnly)}) then {
        _display setVariable [QGVAR(savedOnlineOnly), cbChecked _onlineOnly];
    };
    _onlineOnly cbSetChecked true;
    _onlineOnly ctrlEnable false;
} else {
    if (!isNil {_display getVariable QGVAR(savedOnlineOnly)}) then {
        _onlineOnly cbSetChecked (_display getVariable QGVAR(savedOnlineOnly));
        _display setVariable [QGVAR(savedOnlineOnly), nil];
    };
    _onlineOnly ctrlEnable true;
};

// 4: Lifecycle hooks. Deactivate runs first so onActivate sees fully-restored
// shared state.
if (_prevTab >= 0 && {_prevTab in _tabs}) then {
    call (_tabs get _prevTab get "onDeactivate");
};
call (_activeDesc get "onActivate");
