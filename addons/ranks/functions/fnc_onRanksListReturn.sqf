#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Receives the unsolicited `skua:ranks/list` push fired by the extension after
 * a successful bootstrap. Populates the rank list + lookup map and broadcasts
 * `QGVAR(loaded)` (JIP) once.
 *
 * The extension serializes `Vec<Rank>` via IntoArma, so the wire payload is an
 * SQF array literal — `parseSimpleArray` is sufficient (no `fromJSON` needed).
 *
 * Arguments:
 * 0: Data returned from extension callback <STRING>
 *
 * Return Value:
 * None.
 *
 * Example:
 * "[1, [[0, ""Unranked""], [1, ""Private""]]]" call skua_ranks_fnc_onRanksListReturn;
 *
 * Public: No
 */

params ["_data"];
TRACE_1("fnc_onRanksListReturn",_this);

(parseSimpleArray _data) params ["_status", "_return"];

if (_status != 1) exitWith {
    ERROR_2("Failed to fetch ranks from database: %1: %2",_status,_return);
};

GVAR(list) = _return;

GVAR(map) = createHashMap;
{
    _x params ["_id", "_displayName"];
    GVAR(map) set [_id, _displayName];
} forEach GVAR(list);

INFO_1("Ranks list loaded: %1 entries.",count GVAR(list));

if (!GVAR(loaded)) then {
    GVAR(loaded) = true;
    [QGVAR(loaded)] call CBA_fnc_globalEventJIP;
};
