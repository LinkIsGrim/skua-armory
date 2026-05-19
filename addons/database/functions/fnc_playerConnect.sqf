#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Sends the player_connect callExtension and arms a strand-mitigation timer.
 * If QEV_PLAYER_CONNECTED doesn't arrive within STRAND_TIMEOUT_SECS, resend
 * the call (the upsert is idempotent). Gives up after STRAND_MAX_ATTEMPTS so
 * a hopelessly broken DB doesn't loop forever.
 */

#define STRAND_TIMEOUT_SECS 5
#define STRAND_MAX_ATTEMPTS 3

params ["_uid", "_hasInterface", "_player"];

if (!_hasInterface) exitWith {};

"skua" callExtension ["database:player_connect", [_uid, name _player]];

// Strand mitigation: if Event::PlayerConnected doesn't land in time, resend.
// fnc_onPlayerConnected sets QGVAR(info); the timer checks that as the
// success marker and self-removes.
[{
    params ["_args", "_handle"];
    _args params ["_uid", "_player", "_attempt"];

    if (isNull _player) exitWith {
        // Player left or unit was deleted before connect resolved; abandon.
        [_handle] call CBA_fnc_removePerFrameHandler;
    };

    if (!isNil {_player getVariable QGVAR(info)}) exitWith {
        // Event landed; we're done.
        [_handle] call CBA_fnc_removePerFrameHandler;
    };

    if (_attempt >= STRAND_MAX_ATTEMPTS) exitWith {
        ERROR_2("player_connect stranded for UID %1 after %2 attempts; giving up.",_uid,_attempt);
        [_handle] call CBA_fnc_removePerFrameHandler;
    };

    WARNING_2("player_connect timeout for UID %1; retrying (attempt %2)",_uid,_attempt + 1);
    "skua" callExtension ["database:player_connect", [_uid, name _player]];
    _args set [2, _attempt + 1];
}, STRAND_TIMEOUT_SECS, [_uid, _player, 1]] call CBA_fnc_addPerFrameHandler;
