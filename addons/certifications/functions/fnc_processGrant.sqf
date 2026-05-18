#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Applies a single cert grant to the player unit: fires the cert's grant_event
 * via CBA_fnc_serverEvent and records the cert in the player's local list.
 *
 * Assumes GVAR(map) is populated — caller is responsible for queueing the
 * grant onto GVAR(pendingCertEvents) and replaying after fnc_onCertificationListReturn
 * if the static list hasn't loaded yet.
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
    ERROR_2("Cannot process grant for unknown cert %1 (player %2)",_certID,_playerID);
};

private _playerUnit = _playerID call BIS_fnc_getUnitByUID;
if (isNull _playerUnit) exitWith {
    ERROR_1("Failed to find player unit for certification grant: %1",_playerID);
};

private _event = _certData get "grant_event";
INFO_2("Executing certification event %1 for player %2",_event,_playerID);
[_event, _playerUnit] call CBA_fnc_serverEvent;

private _playerCerts = _playerUnit getVariable [QGVAR(list), []];
_playerCerts pushBackUnique _certID;
_playerUnit setVariable [QGVAR(list), _playerCerts, true];
INFO_2("Certification %1 granted to player %2 successfully",_certID,_playerID);
