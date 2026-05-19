#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * CBA event handler for QEV_CERTIFICATION_REVOKED. Mirrors
 * fnc_onCertificationGranted for revocations. Fired by the extension on
 * ad-hoc revokes and watchdog-detected drift.
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

// Defensive: parallel to fnc_onCertificationGranted. The watchdog can emit
// revokes between bootstrap and the cert-list push landing.
if (!GVAR(loaded)) exitWith {
    GVAR(pendingCertEvents) pushBack ["revoke", _playerID, _certID];
    TRACE_2("Queued revoke pending cert list load",_playerID,_certID);
};

[_playerID, _certID] call FUNC(processRevoke);
