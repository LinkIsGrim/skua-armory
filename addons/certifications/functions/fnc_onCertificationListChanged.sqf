#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * CBA event handler for QEV_CERTIFICATION_LIST_CHANGED. Fired after the
 * post-bootstrap push and whenever the live-load watchdog detects definition
 * drift. Populates GVAR(list)/GVAR(map), flushes any cert events that landed
 * before the list arrived, and broadcasts QGVAR(loaded) on the first call.
 *
 * Arguments:
 * 0: Cert list payload <ARRAY> of <HASHMAP>
 *   Each hashmap mirrors the `Certification` struct: id, display_name,
 *   document, description, perk, pay_bonus, grant_event, revoke_event, requires.
 *
 * Return Value:
 * None.
 *
 * Public: No
 */
params ["_certs"];
TRACE_1("fnc_onCertificationListChanged",count _certs);

GVAR(list) = _certs;
GVAR(map) = createHashMap;
{
    private _id = _x get "id";
    GVAR(map) set [_id, _x];
} forEach GVAR(list);
INFO_1("Certification list updated: %1 entries.",count GVAR(list));

// Subsequent pushes (watchdog re-emissions, manual re-fetches) refresh
// GVAR(map) above but skip the loaded/flush logic.
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
