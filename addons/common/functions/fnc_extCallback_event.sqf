#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Fired from the Mission EventHandler ExtensionCallback added at preInit.
 * Receives events on the unified "skua:event" channel and re-emits them as
 * CBA events. The arma-rs _function slot carries the CBA event name verbatim
 * (e.g. "skua_certification_granted"); subscribers register via
 * `[QEV_*, LINKFUNC(handler)] call CBA_fnc_addEventHandler` and read the
 * fromJSON-decoded payload as the sole arg.
 *
 * Wire format: _data is the SQF rendering of `[QueryState, json_payload]`.
 * parseSimpleArray unwraps the outer array; fromJSON parses the payload. If
 * the state is not QUERYSTATE_DONE, the event is dropped with an error log
 * (the emitter is expected to skip serialization failures).
 *
 * Arguments:
 * 0: Extension name <STRING>
 * 1: Event name (CBA event name verbatim) <STRING>
 * 2: Raw data: `[QueryState, json_payload]` <STRING>
 *
 * Return Value:
 * None.
 *
 * Example:
 * ["skua:event", "skua_certification_granted", "[1,""{""player_id"":""1"",""cert_id"":""medic""}""]"]
 *   call skua_common_fnc_extCallback_event;
 *
 * Public: No
 */
params ["_name", "_event", "_data"];

if (_name != "skua:event") exitWith {};

(parseSimpleArray _data) params ["_state", "_payload"];

if (_state != QUERYSTATE_DONE) exitWith {
    ERROR_2("skua:event %1 callback failed with state %2",_event,_state);
};

[_event, [fromJSON _payload]] call CBA_fnc_localEvent;
