#include "..\script_component.hpp"

/*
* Author: Bridge, Crowmedic, LinkIsParking
* Knock cursor target out.
*
* Arguments:
* 0: Player <OBJECT>
*
* Return Value:
* None.
* Example:
*
* [_player] call skua_knockout_fnc_punchTarget
*
* Public: No
*/

params [["_player", player, [player]]];

private _target = cursorObject;
[_target, "BRIDGE_PunchSound"] remoteExec ["say3D", 0];
_player playActionNow "PutDown";
[_target, true, [GVAR(timeAI), GVAR(timePlayers)] select (isPlayer _target), true] call ACEFUNC(medical,setUnconscious);
