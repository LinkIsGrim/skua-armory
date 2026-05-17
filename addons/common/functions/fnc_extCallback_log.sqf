#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Fired from Mission EventHandler ExtensionCallback added at PreInit.
 * Handles "skua_ext_log" callbacks emitted by the extension's tracing layer
 * and forwards them to the .rpt via diag_log.
 *
 * Arguments:
 * 0: Extension name <STRING>
 * 1: Tracing target (e.g. "skua::database") <STRING>
 * 2: [Level, Message] <ARRAY>
 *   0: Level — one of "ERROR", "WARN", "INFO", "DEBUG", "TRACE" <STRING>
 *   1: Formatted message <STRING>
 *
 * Return Value:
 * None.
 *
 * Example:
 * ["skua_ext_log", "skua::database", ["INFO", "Connection established"]] call skua_common_fnc_extCallback_log;
 *
 * Public: No
 */
params ["_name", "_target", "_data"];

if (_name != "skua_ext_log") exitWith {};

(parseSimpleArray _data) params [["_level", "?", [""]], ["_message", "", [""]]];

INFO_4("skua_ext_log: %1 | Target: %2 | Level: %3 | Message: %4",_name,_target,_level,_message);
