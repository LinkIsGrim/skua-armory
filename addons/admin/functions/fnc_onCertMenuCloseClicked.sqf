#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Close-button click handler. If there are pending uncommitted changes,
 * pops a confirmation modal; otherwise closes immediately.
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
private _pendingCount = 0;
{ _pendingCount = _pendingCount + count _y } forEach _pending;

if (_pendingCount == 0) exitWith {
    _display closeDisplay 0;
};

// Discard-or-stay confirmation. The modal lives in a sibling display, so it
// doesn't tear down the admin menu; we close it ourselves on OK.
[
    ["Uncommitted Changes", format ["%1 pending", _pendingCount]],
    [
        format ["You have %1 pending change(s). Discard and close?", _pendingCount],
        ["Click OK to discard and close, Cancel to return to the menu.", 0.9, [0.7, 0.7, 0.7]]
    ],
    false,
    {},
    {
        params ["", "_exitCode"];
        if (_exitCode == 1) then {
            private _adminMenu = findDisplay IDD_ADMIN_MENU;
            if (!isNull _adminMenu) then {
                _adminMenu closeDisplay 0;
            };
        };
    }
] call EFUNC(common,modal);
