#include "..\script_component.hpp"
/*
 * Authors: LinkIsGrim
 * Compiles Logistics Menu Actions.
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * List of actions <ARRAY>
 *
 * Example:
 * call skua_missions_fnc_compileLogisticsMenuActions;
 *
 * Public: No
 */

TRACE_1("fnc_compileLogisticsMenuActions",_this);

GVAR(logisticsMenuActions) = GVAR(logisticsObjects) apply { _x call FUNC(logistics_makeLogiObjectAction) };
GVAR(fieldLogisticsMenuActions) = (GVAR(logisticsObjects) select { _x select 5 == 1 }) apply { _x call FUNC(logistics_makeLogiObjectAction) };
