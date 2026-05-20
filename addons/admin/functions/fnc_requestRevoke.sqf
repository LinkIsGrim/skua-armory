#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Client-side: ask the server to revoke a cert from a player. Server validates
 * the caller is an admin.
 *
 * Arguments:
 * 0: Grantee Steam UID <STRING>
 * 1: Cert ID <STRING>
 * 2: Persistent (deletes from DB) <BOOL>
 *
 * Return Value:
 * None.
 *
 * Example:
 * ["76561198000000000", "pilot", false] call skua_admin_fnc_requestRevoke;
 *
 * Public: No
 */

params [["_granteeUID", "", [""]], ["_certID", "", [""]], ["_persistent", false, [false]]];

if (_granteeUID isEqualTo "" || {_certID isEqualTo ""}) exitWith {
    ERROR_2("requestRevoke called with bad args: grantee=%1 cert=%2",_granteeUID,_certID);
};

[QGVAR(revokeRequest), [_granteeUID, _certID, getPlayerUID player, _persistent]] call CBA_fnc_serverEvent;
