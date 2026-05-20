#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Repopulates the Held and Available cert listboxes for the currently
 * selected player. Projects the pending change queue onto the underlying
 * persistent/temp holdings so the admin sees the *result* of clicking Grant
 * / Revoke immediately, with a `*` suffix marking entries that are still
 * uncommitted.
 *
 * Held = projected persistent (plain label) + projected temp (with [T]
 * prefix). Available = static cert list minus projected persistent;
 * projected-temp entries stay listed with `(promote)`.
 *
 * No-op if the dialog isn't open. lbData on each row stores the cert id so
 * the click handlers can read it without a separate id<->index map.
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

private _selIdx = lbCurSel _playerList;
if (_selIdx < 0) exitWith {};

private _granteeUID = _playerList lbData _selIdx;
private _grantee = _granteeUID call BIS_fnc_getUnitByUID;
if (isNull _grantee) exitWith {
    TRACE_1("Selected grantee not present locally",_granteeUID);
};

private _persistentCerts = +(_grantee getVariable [QEGVAR(certifications,list), []]);
private _tempCerts = +(_grantee getVariable [QEGVAR(certifications,tempCerts), []]);

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
