#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Receives the decoded `certification:get_player` payload — the queried
 * grantee UID + their persistent cert id list. Stores in the
 * uiNamespace QGVAR(offlineCerts) hashmap so the menu can read cert state
 * for offline players without re-querying the extension on every refresh.
 * Fires a local QGVAR(offlineCertsLoaded) event with the grantee UID so the
 * refresh-on-arrival hook can no-op when a different player is selected.
 *
 * Arguments:
 * 0: Grantee Steam UID <STRING>
 * 1: Cert ids <ARRAY of STRING>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params [["_granteeUID", "", [""]], ["_certIds", [], [[]]]];

private _cache = uiNamespace getVariable QGVAR(offlineCerts);
if (isNil "_cache") then {
    _cache = createHashMap;
    uiNamespace setVariable [QGVAR(offlineCerts), _cache];
};

_cache set [_granteeUID, _certIds];
TRACE_2("offline certs cached",_granteeUID,count _certIds);

[QGVAR(offlineCertsLoaded), [_granteeUID]] call CBA_fnc_localEvent;
