#include "..\script_component.hpp"
/*
 * Authors: LinkIsGrim
 * Set extension log level from CBA settings.
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * None.
 *
 * Example:
 * call skua_common_fnc_setLogLevelFromSettings;
 *
 * Public: No
 */
TRACE_1("fnc_setLogLevelFromSettings",_this);

INFO_1("Attempting to set extension log level to %1",GVAR(extensionLogLevel));

private _result = "skua" callExtension ["logger:set_level", [GVAR(extensionLogLevel)]];

INFO_1("Set extension log level: %1",_result);
