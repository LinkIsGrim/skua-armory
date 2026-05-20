#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Server-side handler for QGVAR(revokeRequest). Mirror of
 * fnc_handleGrantRequest:
 *   - persistent: forwards to the extension's `certification:revoke` (deletes
 *     from skua_master.player_certs; the resulting QEV_CERTIFICATION_REVOKED
 *     event propagates).
 *   - temp: runs the cert's revoke_event on the grantee and removes the cert
 *     id from QEGVAR(certifications,tempCerts). Refuses if the cert is also
 *     present in the persistent QEGVAR(certifications,list) — those must be
 *     revoked persistently to avoid leaving the DB row orphaned with the
 *     perk torn down.
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
    ERROR_1("Rejecting revoke request from non-admin UID %1",_granterUID);
};

private _certData = EGVAR(certifications,map) get _certID;
if (isNil "_certData") exitWith {
    ERROR_2("Rejecting revoke request for unknown cert %1 (admin %2)",_certID,_granterUID);
};

private _grantee = _granteeUID call BIS_fnc_getUnitByUID;
if (isNull _grantee) exitWith {
    ERROR_2("Cannot locate grantee unit for UID %1 (cert %2)",_granteeUID,_certID);
};

if (_persistent) exitWith {
    INFO_3("Admin %1 revoking persistent cert %2 from %3",_granterUID,_certID,_granteeUID);

    // Optimistic event so the revoke_event applies and admin menus refresh
    // without waiting for the extension's DELETE to land. The extension
    // re-emits after the DELETE completes; revoke_event is idempotent and
    // processRevoke's `_persistentCerts - [_certID]` is a no-op the second
    // time through.
    [
        QEV_CERTIFICATION_REVOKED,
        [createHashMapFromArray [["player_id", _granteeUID], ["cert_id", _certID]]]
    ] call CBA_fnc_localEvent;

    "skua" callExtension ["certification:revoke", [_granteeUID, _certID]];
};

// Temp revoke path.
private _persistentCerts = _grantee getVariable [QEGVAR(certifications,list), []];
if (_certID in _persistentCerts) exitWith {
    WARNING_3("Admin %1 attempted temp-revoke of persistent cert %2 from %3 — refused",_granterUID,_certID,_granteeUID);
};

private _tempCerts = _grantee getVariable [QEGVAR(certifications,tempCerts), []];
if !(_certID in _tempCerts) exitWith {
    INFO_2("Cert %1 not in tempCerts for %2, skipping",_certID,_granteeUID);
};

_tempCerts = _tempCerts - [_certID];
_grantee setVariable [QEGVAR(certifications,tempCerts), _tempCerts, true];

private _event = _certData get "revoke_event";
INFO_3("Admin %1 temp-revoking cert %2 from %3",_granterUID,_certID,_granteeUID);
[_event, _grantee] call CBA_fnc_serverEvent;

[QEV_CERTIFICATION_REVOKED_GLOBAL, [createHashMapFromArray [["player_id", _granteeUID], ["cert_id", _certID]]]] call CBA_fnc_globalEvent;
