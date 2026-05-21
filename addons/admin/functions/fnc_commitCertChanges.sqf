#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Fires every queued grant/revoke change as the underlying CBA server event,
 * then clears the pending queue. Iterates each grantee's queue in order so
 * temp-grant-then-promote chains apply in the sequence the admin clicked.
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

private _pending = _display getVariable [QGVAR(pendingChanges), createHashMap];
if (count _pending == 0) exitWith {
    INFO("Commit clicked with no pending changes");
};

private _total = 0;
{
    private _granteeUID = _x;
    {
        _x params ["_op", "_certID", "_persistent"];
        switch (_op) do {
            case "grant":  { [_granteeUID, _certID, _persistent] call FUNC(requestGrant); };
            case "revoke": { [_granteeUID, _certID, _persistent] call FUNC(requestRevoke); };
            default { ERROR_1("Unknown queued op: %1",_op); };
        };
        _total = _total + 1;
    } forEach _y;
} forEach _pending;

INFO_1("Committed %1 pending cert change(s)",_total);
_display setVariable [QGVAR(pendingChanges), createHashMap];

// Server processing is async — listboxes will refresh via the _GLOBAL event
// handler once the events round-trip back. Refresh now so the `*`
// decorations clear immediately.
call FUNC(refreshCertLists);
