#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Server-side handler for QGVAR(grantRequest). Validates the granter is an
 * admin and the cert exists, then either:
 *   - persistent: forwards to the extension's `certification:grant` (writes
 *     to skua_master.player_certs, fires QEV_CERTIFICATION_GRANTED via the
 *     unified event channel which propagates to clients as _GLOBAL),
 *   - temp: globally fires QEV_CERTIFICATION_GRANTED_GLOBAL with the same
 *     payload shape AND appends the cert id to the grantee's
 *     QEGVAR(certifications,tempCerts) variable (broadcast), then runs the
 *     cert's grant_event so perks apply. Does NOT touch
 *     QEGVAR(certifications,list) — temp grants are tracked separately so
 *     they don't survive reconnect.
 *
 * Arguments:
 * 0: Grantee Steam UID <STRING>
 * 1: Cert ID <STRING>
 * 2: Granter Steam UID <STRING>
 * 3: Persistent <BOOL>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params [["_granteeUID", "", [""]], ["_certID", "", [""]], ["_granterUID", "", [""]], ["_persistent", false, [false]]];

if !(_granterUID in GVAR(admins)) exitWith {
    ERROR_1("Rejecting grant request from non-admin UID %1",_granterUID);
};

private _certData = EGVAR(certifications,map) get _certID;
if (isNil "_certData") exitWith {
    ERROR_2("Rejecting grant request for unknown cert %1 (admin %2)",_certID,_granterUID);
};

private _grantee = _granteeUID call BIS_fnc_getUnitByUID;
if (isNull _grantee) exitWith {
    ERROR_2("Cannot locate grantee unit for UID %1 (cert %2)",_granteeUID,_certID);
};

if (_persistent) exitWith {
    // If this cert was previously temp-granted, clear it from tempCerts so
    // it doesn't double-count once the persistent grant lands.
    private _tempCerts = _grantee getVariable [QEGVAR(certifications,tempCerts), []];
    if (_certID in _tempCerts) then {
        _tempCerts = _tempCerts - [_certID];
        _grantee setVariable [QEGVAR(certifications,tempCerts), _tempCerts, true];
        INFO_3("Admin %1 promoting temp cert %2 on %3 to persistent",_granterUID,_certID,_granteeUID);
    } else {
        INFO_3("Admin %1 granting persistent cert %2 to %3",_granterUID,_certID,_granteeUID);
    };

    // Fire the canonical event optimistically so the perk applies and admin
    // menus refresh without waiting for the extension's DB round-trip. The
    // extension will re-emit the same event after the INSERT lands; the
    // grant_event is idempotent and processGrant's pushBackUnique swallows
    // the duplicate on QGVAR(list).
    [
        QEV_CERTIFICATION_GRANTED,
        [createHashMapFromArray [["player_id", _granteeUID], ["cert_id", _certID]]]
    ] call CBA_fnc_localEvent;

    "skua" callExtension ["certification:grant", [_granteeUID, _certID, _granterUID]];
};

// Temp grant path.
private _tempCerts = _grantee getVariable [QEGVAR(certifications,tempCerts), []];
if (_certID in _tempCerts) exitWith {
    INFO_2("Cert %1 already temp-granted to %2, skipping",_certID,_granteeUID);
};

_tempCerts pushBack _certID;
_grantee setVariable [QEGVAR(certifications,tempCerts), _tempCerts, true];

private _event = _certData get "grant_event";
INFO_3("Admin %1 temp-granting cert %2 to %3",_granterUID,_certID,_granteeUID);
[_event, _grantee] call CBA_fnc_serverEvent;

// Notify open admin menus on hasInterface clients to refresh.
[QEV_CERTIFICATION_GRANTED_GLOBAL, [createHashMapFromArray [["player_id", _granteeUID], ["cert_id", _certID]]]] call CBA_fnc_globalEvent;
