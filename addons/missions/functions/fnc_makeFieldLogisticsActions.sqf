#include "..\script_component.hpp"
/*
 * Authors: LinkIsGrim
 * Build the Field Logistics Menu action tree for a repair vehicle/facility.
 *
 * Arguments:
 * 0: Target object <OBJECT>
 *
 * Return Value:
 * Actions <ARRAY>
 *
 * Example:
 * [_target] call skua_missions_fnc_makeFieldLogisticsActions;
 *
 * Public: No
 */
params ["_target"];
TRACE_1("fnc_makeFieldLogisticsActions",_this);

private _actions = [];
{
    _actions pushBack [_x, [], _target];
} forEach GVAR(fieldLogisticsMenuActions);

_actions
