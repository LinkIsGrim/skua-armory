#include "..\script_component.hpp"

/*
 * Author: Bridge, Crowmedic, LinkIsParking
 * Determines whether target can be punched
 *
 * Arguments:
 * 0: Player <OBJECT>
 * 1: Target <OBJECT>
 *
 * Return Value:
 * None
 *
 * Example:
 * [_player, _target] call skua_knockout_fnc_canPunch
 *
 * Public: No
*/
params ["_player", ["_target", objNull]];

private _distance = 0;
if (isNull _target) then {
    private _cursorObjectParams = getCursorObjectParams;
    _target = _cursorObjectParams select 0;
    _distance = _cursorObjectParams select 2;
} else {
    _distance = _player distance _target;
};

if (_distance > 4) exitWith {false};
if !(_target isKindOf "CAManBase") exitWith {false};

if (!isNull objectParent _target && {!([_player, objectParent _target] call ACEFUNC(interaction,canInteractWithVehicleCrew))}) exitWith {false};

private _permLevel = [GVAR(knockOutAI), GVAR(knockOutPlayers)] select isPlayer _target;

if (_permLevel == 0) exitWith {true}; // Everyone
if (_permLevel == 2) exitWith {false}; // Disabled

// Admins Only
isServer || {call BIS_fnc_isDebugConsoleAllowed} || {IS_ADMIN_LOGGED};
