#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Fired from Mission EventHandler ExtensionCallback added at PreInit.
 * Handles extension callbacks for "skua:ranks" and routes to appropriate functions.
 *
 * Arguments:
 * 0: Extension name (in theory, extension may report something else) <STRING>
 * 1: Function name (arbitrary, defined by extension) <STRING>
 * 2: Data: returned data from the extension and error codes. <STRING>
 *
 * Return Value:
 * None.
 *
 * Example:
 * ["skua:ranks", "list", "[1, [[0, ""Unranked""]]]"] call skua_ranks_fnc_extCallback;
 *
 * Public: No
 */
params ["_name", "_function", "_data"];

if (_name != "skua:ranks") exitWith {};

switch (_function) do {
    case "list": {_data call FUNC(onRanksListReturn)};
    default {ERROR_1("Unhandled extension callback function: %1",_function)};
};
