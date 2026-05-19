#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * CBA event handler for QEV_RANK_LIST_CHANGED. Fired after the post-bootstrap
 * push. Populates GVAR(list)/GVAR(map) and broadcasts QGVAR(loaded) on the
 * first call.
 *
 * Arguments:
 * 0: Ranks list payload <ARRAY> of <HASHMAP>
 *   Each hashmap mirrors the `Rank` struct: id, display_name.
 *
 * Return Value:
 * None.
 *
 * Public: No
 */
params ["_ranks"];
TRACE_1("fnc_onRanksListChanged",count _ranks);

GVAR(list) = _ranks;
GVAR(map) = createHashMap;
{
    private _id = _x get "id";
    private _displayName = _x get "display_name";
    GVAR(map) set [_id, _displayName];
} forEach GVAR(list);

INFO_1("Ranks list loaded: %1 entries.",count GVAR(list));

if (!GVAR(loaded)) then {
    GVAR(loaded) = true;
    [QGVAR(loaded)] call CBA_fnc_globalEventJIP;
};
