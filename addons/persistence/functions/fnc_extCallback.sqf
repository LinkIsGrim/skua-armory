#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Server-only ExtensionCallback router for skua:loadout. Decodes the
 * get_player response and applies the loadout to the player unit if one is
 * still connected.
 *
 * Wire format (get_player):
 *   `_data` is the SQF rendering of `[QueryState, payload]` where payload is
 *   `[<uid string>, <loadout array or []>]`. The uid round-trips so we can
 *   route the loadout to the correct unit. Empty inner array means no
 *   stored / default loadout — leave the unit alone. The empty-array
 *   sentinel is used because `null` would break `parseSimpleArray`.
 *
 * `set_player` / `set_default` only fire callbacks on failure (see
 * extension/src/loadout/commands.rs); success is silent.
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

if (_name != "skua:loadout") exitWith {};

switch (_function) do {
    case "get_player": {
        (parseSimpleArray _data) params ["_state", "_payload"];
        if (_state != QUERYSTATE_DONE) exitWith {
            ERROR_2("loadout:get_player callback failed with state %1: %2",_state,_data);
        };
        _payload params ["_uid", "_loadout"];
        if (_loadout isEqualTo []) exitWith {
            TRACE_1("loadout:get_player no stored loadout for player",_uid);
        };

        private _player = _uid call BIS_fnc_getUnitByUID;
        if (isNull _player) exitWith {
            WARNING_1("loadout:get_player landed for unknown player UID %1; dropping.",_uid);
        };

        [_player, _loadout] call CBA_fnc_setLoadout;
        TRACE_1("loadout: applied to player",_uid);
    };

    case "set_player": {
        (parseSimpleArray _data) params ["_state"];
        ERROR_2("loadout:set_player failed with state %1: %2",_state,_data);
    };

    case "set_default": {
        (parseSimpleArray _data) params ["_state"];
        ERROR_2("loadout:set_default failed with state %1: %2",_state,_data);
    };

    default {ERROR_1("Unhandled skua:loadout callback function: %1",_function)};
};
