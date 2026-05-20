#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Full repopulation of the admin cert menu's player listbox. Honors the
 * "Online only" checkbox:
 *  - checked: only currently-online players (CBA_fnc_players).
 *  - unchecked: union of online + historical roster (uiNamespace cache from
 *    player_info:list). Online players are labeled by their live name; offline
 *    players use the cached DB name and get a " (offline)" suffix.
 *
 * Sets each row's tooltip to the player's Steam UID so hovering reveals it
 * and Ctrl-C-on-selection has something to copy. Preserves the selected UID
 * across refreshes when possible, then triggers cert-listbox repopulation.
 *
 * No-op if the dialog isn't open.
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
private _onlineOnlyCtrl = _display displayCtrl IDC_ADMINCERT_CHK_ONLINE_ONLY;
private _onlineOnly = (cbChecked _onlineOnlyCtrl);

// Preserve selection across refresh.
private _prevSelIdx = lbCurSel _playerList;
private _prevUID = if (_prevSelIdx >= 0) then {
    _playerList lbData _prevSelIdx
} else {
    ""
};

lbClear _playerList;

// Build online-UID set first so we can label/skip offline duplicates cheaply.
private _onlinePlayers = call CBA_fnc_players;
private _onlineUIDs = createHashMap;
{
    _onlineUIDs set [getPlayerUID _x, _x];
} forEach _onlinePlayers;

// Add online players (label = live name, no suffix).
{
    private _uid = getPlayerUID _x;
    private _idx = _playerList lbAdd (name _x);
    _playerList lbSetData [_idx, _uid];
    _playerList lbSetTooltip [_idx, _uid];
} forEach _onlinePlayers;

// Append historical roster entries that aren't already in the online set.
if (!_onlineOnly) then {
    private _roster = uiNamespace getVariable [QGVAR(playerRoster), []];
    {
        private _uid = _x get "steam_id";
        if !(_uid in _onlineUIDs) then {
            private _idx = _playerList lbAdd format ["%1 (offline)", _x get "name"];
            _playerList lbSetData [_idx, _uid];
            _playerList lbSetTooltip [_idx, _uid];
            // Dim offline entries so they're visually distinct from live ones.
            _playerList lbSetColor [_idx, [0.7, 0.7, 0.7, 1]];
        };
    } forEach _roster;
};

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
