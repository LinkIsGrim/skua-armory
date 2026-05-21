#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Syncs BtnAction / BtnActionTemp labels + enable state with the currently
 * focused cert listbox (Held → Revoke, Available → Grant). Temp button stays
 * disabled when the selected grantee is offline (no unit to attach tempCerts
 * to / no live perk to tear down).
 *
 * Called from: fnc_onAdminMenuOpen (initial state), the listbox LBSelChanged
 * hook (focus change), and fnc_refreshCertLists (after a player selection
 * change so the buttons reflect online/offline status).
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

private _display = findDisplay IDD_ADMIN_MENU;
if (isNull _display) exitWith {};

private _btn = _display displayCtrl IDC_ADMINCERT_BTN_ACTION;
private _btnTemp = _display displayCtrl IDC_ADMINCERT_BTN_ACTION_TEMP;

private _focused = _display getVariable [QGVAR(focusedCertList), -1];
private _focusedCtrl = if (_focused > 0) then {_display displayCtrl _focused} else {controlNull};

// No focused cert list, or focused list has no selection: buttons stay
// disabled with a hint label so the admin knows what's missing.
if (isNull _focusedCtrl || {lbCurSel _focusedCtrl < 0}) exitWith {
    _btn ctrlSetText "Grant";
    _btnTemp ctrlSetText "Grant (temp)";
    _btn ctrlEnable false;
    _btnTemp ctrlEnable false;
};

// Check whether the selected grantee is online — temp ops require a live unit.
private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;
private _pSel = lbCurSel _playerList;
private _isOnline = if (_pSel < 0) then {false} else {
    !isNull ((_playerList lbData _pSel) call BIS_fnc_getUnitByUID)
};

switch (_focused) do {
    case IDC_ADMINCERT_HELD_LIST: {
        _btn ctrlSetText "Revoke";
        _btnTemp ctrlSetText "Revoke (temp)";
    };
    case IDC_ADMINCERT_AVAILABLE_LIST: {
        _btn ctrlSetText "Grant";
        _btnTemp ctrlSetText "Grant (temp)";
    };
};
_btn ctrlEnable true;
_btnTemp ctrlEnable _isOnline;
