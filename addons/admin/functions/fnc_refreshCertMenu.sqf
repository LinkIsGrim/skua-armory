#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Full repopulation of the admin cert menu: rebuilds the player listbox from
 * the current set of interface-having players, preserving the previously
 * selected player UID across the refresh if they're still online, then
 * refreshes the cert listboxes for whichever player ends up selected.
 *
 * No-op if the dialog isn't open. Safe to call from event handlers that fire
 * regardless of dialog state.
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

private _display = findDisplay IDD_ADMIN_CERT_MENU;
if (isNull _display) exitWith {};

private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;

// Preserve selection across refresh.
private _prevSelIdx = lbCurSel _playerList;
private _prevUID = if (_prevSelIdx >= 0) then {
    _playerList lbData _prevSelIdx
} else {
    ""
};

lbClear _playerList;

private _players = call CBA_fnc_players;
{
    private _idx = _playerList lbAdd name _x;
    _playerList lbSetData [_idx, getPlayerUID _x];
} forEach _players;

// Restore selection by UID, or pick the first entry.
private _restoreIdx = -1;
if (_prevUID isNotEqualTo "") then {
    for "_i" from 0 to (lbSize _playerList) - 1 do {
        if ((_playerList lbData _i) isEqualTo _prevUID) exitWith {_restoreIdx = _i};
    };
};
if (_restoreIdx < 0 && {lbSize _playerList > 0}) then {_restoreIdx = 0};
if (_restoreIdx >= 0) then {_playerList lbSetCurSel _restoreIdx};

call FUNC(refreshCertLists);
