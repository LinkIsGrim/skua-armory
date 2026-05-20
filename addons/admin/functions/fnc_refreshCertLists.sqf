#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Repopulates the Held and Available cert listboxes for the currently
 * selected player. Sources cert state from one of two places depending on
 * whether the grantee is online:
 *  - online:  unit setVariables (skua_certifications_list + tempCerts).
 *  - offline: uiNamespace cache populated by fnc_onCertificationGetPlayerCallback.
 *             Triggers a lazy fetch if the cache misses; placeholder text shows
 *             until the callback arrives.
 *
 * Projects the pending change queue onto the underlying persistent/temp
 * holdings so the admin sees the *result* of clicking Grant / Revoke
 * immediately, with a `*` suffix marking uncommitted entries. Temp
 * grant/revoke buttons are disabled for offline grantees (no unit to attach
 * tempCerts to, and temp-revoke has no DB row to remove).
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
private _heldList = _display displayCtrl IDC_ADMINCERT_HELD_LIST;
private _availList = _display displayCtrl IDC_ADMINCERT_AVAILABLE_LIST;

lbClear _heldList;
lbClear _availList;

// Selection in either cert listbox is wiped by lbClear, so the focused-list
// pointer is stale until the admin clicks something again.
_display setVariable [QGVAR(focusedCertList), -1];

private _selIdx = lbCurSel _playerList;
if (_selIdx < 0) exitWith {
    call FUNC(updateActionButtons);
};

private _granteeUID = _playerList lbData _selIdx;
private _grantee = _granteeUID call BIS_fnc_getUnitByUID;
private _isOnline = !isNull _grantee;

private _persistentCerts = [];
private _tempCerts = [];

if (_isOnline) then {
    _persistentCerts = +(_grantee getVariable [QEGVAR(certifications,list), []]);
    _tempCerts = +(_grantee getVariable [QEGVAR(certifications,tempCerts), []]);
} else {
    // Offline source: uiNamespace cache. Lazy-fetch on miss.
    private _cache = uiNamespace getVariable [QGVAR(offlineCerts), createHashMap];
    if (_cache getOrDefault [_granteeUID, "MISS"] isEqualTo "MISS") exitWith {
        _heldList lbAdd "Loading...";
        [_granteeUID] call FUNC(fetchOfflineCerts);
    };
    _persistentCerts = +(_cache get _granteeUID);
    // _tempCerts stays empty — temp grants don't exist for offline players.
};

// Project pending changes. Track every cert id that has at least one pending
// op for the current grantee so we can `*`-decorate it in either listbox.
private _pendingMap = _display getVariable [QGVAR(pendingChanges), createHashMap];
private _queue = _pendingMap getOrDefault [_granteeUID, []];
private _pendingIds = [];
{
    _x params ["_op", "_certID", "_persistent"];
    _pendingIds pushBackUnique _certID;
    switch (true) do {
        case (_op == "grant" && _persistent): {
            _persistentCerts pushBackUnique _certID;
            _tempCerts = _tempCerts - [_certID];
        };
        case (_op == "grant" && !_persistent): {
            if !(_certID in _persistentCerts) then {
                _tempCerts pushBackUnique _certID;
            };
        };
        case (_op == "revoke" && _persistent): {
            _persistentCerts = _persistentCerts - [_certID];
        };
        case (_op == "revoke" && !_persistent): {
            _tempCerts = _tempCerts - [_certID];
        };
    };
} forEach _queue;

private _certList = EGVAR(certifications,list);
private _certMap = EGVAR(certifications,map);

private _decorate = {
    params ["_label", "_certID"];
    if (_certID in _pendingIds) then {
        format ["%1 *", _label]
    } else {
        _label
    }
};

// Held list: projected persistent first, then projected temp (with [T] prefix).
{
    private _data = _certMap get _x;
    private _label = if (isNil "_data") then {_x} else {_data get "display_name"};
    private _idx = _heldList lbAdd ([_label, _x] call _decorate);
    _heldList lbSetData [_idx, _x];
} forEach _persistentCerts;

{
    if !(_x in _persistentCerts) then {
        private _data = _certMap get _x;
        private _label = if (isNil "_data") then {_x} else {_data get "display_name"};
        private _idx = _heldList lbAdd ([format ["[T] %1", _label], _x] call _decorate);
        _heldList lbSetData [_idx, _x];
    };
} forEach _tempCerts;

// Available: certs not in projected persistent. Projected-temp entries stay
// listed with `(promote)` so the admin can queue a promote.
{
    private _id = _x get "id";
    if !(_id in _persistentCerts) then {
        private _label = _x get "display_name";
        if (_id in _tempCerts) then {
            _label = format ["%1 (promote)", _label];
        };
        private _idx = _availList lbAdd ([_label, _id] call _decorate);
        _availList lbSetData [_idx, _id];
    };
} forEach _certList;

call FUNC(updateActionButtons);
