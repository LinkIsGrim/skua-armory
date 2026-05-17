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

if (!GVAR(loaded)) then {
    [QGVAR(loaded)] call CBA_fnc_globalEventJIP;
};

if (GVAR(loaded)) exitWith {}; // If certs already loaded, no need to run post-load code again

GVAR(loaded) = true;
INFO_1("Executing post-load code for certifications. %1 callbacks to run.",count GVAR(postLoadCode));
{
    private _code = _x select 0;
    private _args = _x select 1;
    _args call _code;
} forEach GVAR(postLoadCode);
GVAR(postLoadCode) = []; // Clear post-load code after running
