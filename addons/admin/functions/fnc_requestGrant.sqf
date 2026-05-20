#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Client-side: ask the server to grant a cert to a player. Server validates
 * the caller is an admin before doing anything. The granter UID is taken from
 * the local player (do not trust client-supplied granter).
 *
 * Arguments:
 * 0: Grantee Steam UID <STRING>
 * 1: Cert ID <STRING>
 * 2: Persistent (writes to DB) <BOOL>
 *
 * Return Value:
 * None.
 *
 * Example:
 * ["76561198000000000", "pilot", true] call skua_admin_fnc_requestGrant;
 *
 * Public: No
 */

params [["_granteeUID", "", [""]], ["_certID", "", [""]], ["_persistent", false, [false]]];

if (_granteeUID isEqualTo "" || {_certID isEqualTo ""}) exitWith {
    ERROR_2("requestGrant called with bad args: grantee=%1 cert=%2",_granteeUID,_certID);
};

[QGVAR(grantRequest), [_granteeUID, _certID, getPlayerUID player, _persistent]] call CBA_fnc_serverEvent;
