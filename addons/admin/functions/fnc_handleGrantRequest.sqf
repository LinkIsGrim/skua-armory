#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Server-side handler for QGVAR(grantRequest). Validates the granter is an
 * admin and the cert exists, then either:
 *   - persistent: forwards to the extension's `certification:grant` (writes
 *     to skua_master.player_certs, fires QEV_CERTIFICATION_GRANTED via the
 *     unified event channel which propagates to clients as _GLOBAL).
 *     For online grantees, also fires the canonical event locally as an
 *     optimistic short-circuit so the perk applies without waiting for the
 *     DB round-trip.
 *   - temp: appends the cert id to the grantee's tempCerts variable, runs
 *     the cert's grant_event, and globalEvents the _GLOBAL refresh hook.
 *     Temp grants require a live unit — rejected for offline grantees.
 *
 * Offline grantees can still receive persistent grants; the cert hydrates on
 * their next connect via push_player_certs and the optimistic local event is
 * skipped (no unit to apply perks to anyway).
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
private _isOnline = !isNull _grantee;

if (_persistent) exitWith {
    // If this cert was previously temp-granted (only possible while online),
    // clear it from tempCerts so it doesn't double-count once the persistent
    // grant lands.
    if (_isOnline) then {
        private _tempCerts = _grantee getVariable [QEGVAR(certifications,tempCerts), []];
        if (_certID in _tempCerts) then {
            _tempCerts = _tempCerts - [_certID];
            _grantee setVariable [QEGVAR(certifications,tempCerts), _tempCerts, true];
            INFO_3("Admin %1 promoting temp cert %2 on %3 to persistent",_granterUID,_certID,_granteeUID);
        } else {
            INFO_3("Admin %1 granting persistent cert %2 to %3",_granterUID,_certID,_granteeUID);
        };

        // Optimistic local event — only fires for online grantees because
        // processGrant needs the unit to run grant_event and update the
        // setVariable. Offline grantees pick the cert up on next connect via
        // push_player_certs.
        [
            QEV_CERTIFICATION_GRANTED,
            [createHashMapFromArray [["player_id", _granteeUID], ["cert_id", _certID]]]
        ] call CBA_fnc_localEvent;
    } else {
        INFO_3("Admin %1 granting persistent cert %2 to offline player %3",_granterUID,_certID,_granteeUID);
    };

    "skua" callExtension ["certification:grant", [_granteeUID, _certID, _granterUID]];
};

// Temp grant path. Requires a live unit.
if (!_isOnline) exitWith {
    WARNING_2("Rejecting temp grant for offline player %1 (cert %2)",_granteeUID,_certID);
};

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
