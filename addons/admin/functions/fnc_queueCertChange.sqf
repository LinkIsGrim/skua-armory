#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Internal helper for the admin cert menu. Reads the player + cert selection
 * from the menu and appends a change record to the per-grantee pending queue
 * stored on the display. Refreshes the listboxes so the queued change is
 * reflected immediately with a `*` decoration.
 *
 * Pending record shape: [_op, _certID, _persistent].
 *  _op:         "grant" | "revoke"
 *  _certID:     <STRING>
 *  _persistent: <BOOL>  (false for temp grant / temp revoke)
 *
 * Arguments:
 * 0: Op <STRING>      — "grant" or "revoke"
 * 1: Source listbox idc <NUMBER> — read cert id from this listbox
 * 2: Persistent <BOOL>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params [["_op", "", [""]], ["_sourceIdc", 0, [0]], ["_persistent", false, [false]]];

private _display = findDisplay IDD_ADMIN_CERT_MENU;
if (isNull _display) exitWith {};

private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;
private _source = _display displayCtrl _sourceIdc;

private _pSel = lbCurSel _playerList;
private _cSel = lbCurSel _source;
if (_pSel < 0 || _cSel < 0) exitWith {
    INFO_2("Queue cert change without selection (op=%1, src=%2)",_op,_sourceIdc);
};

private _granteeUID = _playerList lbData _pSel;
private _certID = _source lbData _cSel;

private _pending = _display getVariable [QGVAR(pendingChanges), createHashMap];
private _queue = _pending getOrDefault [_granteeUID, [], true];
_queue pushBack [_op, _certID, _persistent];
_display setVariable [QGVAR(pendingChanges), _pending];

TRACE_4("Queued cert change",_op,_certID,_persistent,_granteeUID);

call FUNC(refreshCertLists);
