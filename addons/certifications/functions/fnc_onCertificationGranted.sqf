#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * CBA event handler for QEV_CERTIFICATION_GRANTED. Receives the parsed
 * fromJSON payload as a single arg. Fired by the extension on ad-hoc grants,
 * cert hydration during player_connect, and watchdog-detected drift — same
 * shape regardless of source.
 *
 * Arguments:
 * 0: Payload <HASHMAP>
 *   "player_id": <STRING> Steam ID
 *   "cert_id":   <STRING>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */
params ["_data"];

private _playerID = _data get "player_id";
private _certID = _data get "cert_id";

// Defensive: the extension fires per-player grants during player_connect and
// from the live-load watchdog. Either may land before the post-bootstrap
// cert-list push has been processed. Queue and let fnc_onCertificationListChanged
// replay once GVAR(map) is populated.
if (!GVAR(loaded)) exitWith {
    GVAR(pendingCertEvents) pushBack ["grant", _playerID, _certID];
    TRACE_2("Queued grant pending cert list load",_playerID,_certID);
};

[_playerID, _certID] call FUNC(processGrant);
