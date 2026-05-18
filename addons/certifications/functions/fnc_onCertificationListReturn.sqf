#include "..\script_component.hpp"
/*
 * Authors: LinkIsGrim
 * Description.
 *
 * Arguments:
 * 0: Data returned from extension callback <STRING>
 *
 * Return Value:
 * None.
 *
 * Example:
 * '[1, "[{""id"": ""medic"", ""displayName"": ""Medic"", ""grant_event"": ""skua_cert_medic""}]"]' call skua_certifications_fnc_onCertificationListReturn;
 *
 * Public: No
 */

params ["_data"];
TRACE_1("fnc_onCertificationListReturn",_this);

INFO_1("Received certification list from server: %1",_data);
(parseSimpleArray _data) params ["_status", "_return"];

if (_status != 1) exitWith {
    ERROR_2("Failed to fetch certifications from database: %1: %2",_status,_return);
};

GVAR(list) = fromJSON _return; // Store certs in global variable

// Populate id to cert data map for easy lookup
{
    private _id = _x get "id";
    GVAR(map) set [_id, _x];
} forEach GVAR(list);
INFO("Certification list updated successfully.");

// Bootstrap-retry re-pushes hit this path; GVAR(map) is refreshed above, but
// the loaded event + pending-event flush are first-time-only.
if (GVAR(loaded)) exitWith {};

GVAR(loaded) = true;

INFO_1("Flushing %1 pending cert event(s) queued before cert list arrived.",count GVAR(pendingCertEvents));
{
    _x params ["_type", "_playerID", "_certID"];
    switch (_type) do {
        case "grant": { [_playerID, _certID] call FUNC(processGrant); };
        case "revoke": { [_playerID, _certID] call FUNC(processRevoke); };
        default { ERROR_1("Unknown pending cert event type: %1",_type); };
    };
} forEach GVAR(pendingCertEvents);
GVAR(pendingCertEvents) = [];

[QGVAR(loaded)] call CBA_fnc_globalEventJIP;
