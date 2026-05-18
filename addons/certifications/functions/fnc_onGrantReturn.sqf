#include "..\script_component.hpp"

params ["_data"];
INFO_1("Received grant result from server: %1",_data);

(parseSimpleArray _data) params ["_status", "_return"];
if (_status != 1) exitWith {
    ERROR_2("Failed to process certification grant: %1: %2",_status,_return);
};

private _grantData = fromJSON _return;
private _playerID = _grantData get "player_id";
private _certID = _grantData get "cert_id";

// Defensive: the extension fires per-player grants during player_connect and
// from the live-load watchdog. Either may land before the post-bootstrap
// cert-list push has been processed. Queue and let fnc_onCertificationListReturn
// replay once GVAR(map) is populated.
if (!GVAR(loaded)) exitWith {
    GVAR(pendingCertEvents) pushBack ["grant", _playerID, _certID];
    TRACE_2("Queued grant pending cert list load",_playerID,_certID);
};

[_playerID, _certID] call FUNC(processGrant);
