#include "..\script_component.hpp"

params ["_data"];
INFO_1("Received revoke result from server: %1",_data);

(parseSimpleArray _data) params ["_status", "_return"];
if (_status != 1) exitWith {
    ERROR_2("Failed to process certification revoke: %1: %2",_status,_return);
};

private _revokeData = fromJSON _return;
private _playerID = _revokeData get "player_id";
private _certID = _revokeData get "cert_id";

// Defensive: parallel to fnc_onGrantReturn. The watchdog can emit revokes
// between bootstrap and the cert-list push landing.
if (!GVAR(loaded)) exitWith {
    GVAR(pendingCertEvents) pushBack ["revoke", _playerID, _certID];
    TRACE_2("Queued revoke pending cert list load",_playerID,_certID);
};

[_playerID, _certID] call FUNC(processRevoke);
