#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Server-only mission ExtensionCallback router for the admin cert menu.
 * Decodes the two callback channels this addon owns and fans the results
 * out to admin clients only (via fnc_adminEvent) — these payloads are
 * admin-UI-only and would otherwise flood every connected client:
 *   - "skua:player_info" "list"        → QGVAR(rosterPushed) [roster]
 *   - "skua:certification" "get_player" → QGVAR(offlineCertsPushed) [uid, certIds]
 *
 * Wire format: `_data` is the SQF rendering of `[QueryState, payload]`. Both
 * payloads are JSON strings (see Roster / PlayerCerts IntoArma in the
 * extension). Non-Done states log and drop — the extension already logged
 * the underlying failure.
 *
 * Arguments:
 * 0: Extension name <STRING>
 * 1: Function name <STRING>
 * 2: Raw data <STRING>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_name", "_function", "_data"];

switch (_name) do {
    case "skua:player_info": {
        if (_function != "list") exitWith {};
        (parseSimpleArray _data) params ["_state", "_payload"];
        if (_state != QUERYSTATE_DONE) exitWith {
            ERROR_1("player_info:list callback failed with state %1",_state);
        };
        private _roster = fromJSON _payload;
        [QGVAR(rosterPushed), [_roster]] call FUNC(adminEvent);
    };

    case "skua:certification": {
        if (_function != "get_player") exitWith {};
        (parseSimpleArray _data) params ["_state", "_payload"];
        if (_state != QUERYSTATE_DONE) exitWith {
            ERROR_1("certification:get_player callback failed with state %1",_state);
        };
        private _decoded = fromJSON _payload;
        [
            QGVAR(offlineCertsPushed),
            [_decoded get "player_id", _decoded get "cert_ids"]
        ] call FUNC(adminEvent);
    };

    default {};
};
