#include "..\script_component.hpp"

params ["_data"];
INFO_1("Received grant/revoke result from server: %1",_data);

(parseSimpleArray _data) params ["_status", "_return"];
if (_status != 1) exitWith {
    ERROR_2("Failed to process certification update: %1: %2",_status,_return);
};

private _grantData = fromJSON _return;
private _playerID = _grantData get "player_id";
private _certID = _grantData get "cert_id";

private _event = GVAR(map) get _certID get "grant_event";

private _playerUnit = _playerID call BIS_fnc_getUnitByUID;
if (isNull _playerUnit) then {
    ERROR_1("Failed to find player unit for certification update: %1",_playerID);
} else {
    INFO_2("Executing certification event %1 for player %2",_event,_playerID);
    [_event, _playerUnit] call CBA_fnc_serverEvent;

    private _playerCerts = _playerUnit getVariable [QGVAR(list), []];
    _playerCerts pushBackUnique _certID;
    _playerUnit setVariable [QGVAR(list), _playerCerts, true];
    INFO_2("Certification %1 granted to player %2 successfully",_certID,_playerID);
};
