#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Returns whether the given unit's Steam UID is in the configured admin list
 * (`enableDebugConsole` baked into the admin addon's config). Safe to call
 * client-side — every machine has the same config in its PBO.
 *
 * Arguments:
 * 0: Unit <OBJECT>
 *
 * Return Value:
 * Is admin <BOOL>
 *
 * Example:
 * (_player call skua_admin_fnc_isAdmin)
 *
 * Public: Yes
 */

params [["_unit", objNull, [objNull]]];

getPlayerUID _unit in (getArray (configFile >> "enableDebugConsole"))
