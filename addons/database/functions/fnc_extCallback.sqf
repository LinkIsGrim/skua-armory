#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Fired from Mission EventHandler ExtensionCallback added at PreInit.
 * Handles extension callbacks for "skua:database" and routes to appropriate functions.
 *
 * Arguments:
 * 0: Extension name (in theory, extension may report something else) <STRING>
 * 1: Function name (arbitrary, defined by extension) <STRING>
 * 2: Data: returned data from the extension and error codes. <ARRAY>
 *
 * Return Value:
 * None.
 *
 * Example:
 * ["skua:database", "upsert_player", ["0", 0, 0]] call skua_database_fnc_extCallback;
 *
 * Public: No
 */
params ["_name", "_function", "_data"];

if (_name != "skua:database") exitWith {};

switch (_function) do {
    case "upsert_player": {_data call FUNC(onUpsertPlayerReturn)};
    case "bootstrap": {_data call FUNC(onBootstrapReturn)};
    default {ERROR_1("Unhandled extension callback function: %1",_function)};
};
