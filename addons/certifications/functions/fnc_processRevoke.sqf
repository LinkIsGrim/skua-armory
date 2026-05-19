#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Applies a single cert revoke to the player unit: fires the cert's
 * revoke_event via CBA_fnc_serverEvent and removes the cert from the player's
 * local list. Mirror of fnc_processGrant.
 *
 * Assumes GVAR(map) is populated — caller is responsible for queueing the
 * revoke onto GVAR(pendingCertEvents) and replaying after
 * fnc_onCertificationListChanged if the static list hasn't loaded yet.
 *
 * Arguments:
 * 0: Player Steam ID <STRING>
 * 1: Cert ID <STRING>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_playerID", "_certID"];

private _certData = GVAR(map) get _certID;
if (isNil "_certData") exitWith {
    ERROR_2("Cannot process revoke for unknown cert %1 (player %2)",_certID,_playerID);
};

private _playerUnit = _playerID call BIS_fnc_getUnitByUID;
if (isNull _playerUnit) exitWith {
    ERROR_1("Failed to find player unit for certification revoke: %1",_playerID);
};

private _event = _certData get "revoke_event";
INFO_2("Executing certification revoke event %1 for player %2",_event,_playerID);
[_event, _playerUnit] call CBA_fnc_serverEvent;

private _playerCerts = _playerUnit getVariable [QGVAR(list), []];
private _idx = _playerCerts find _certID;
if (_idx > -1) then {
    _playerCerts deleteAt _idx;
    _playerUnit setVariable [QGVAR(list), _playerCerts, true];
};
INFO_2("Certification %1 revoked from player %2 successfully",_certID,_playerID);
