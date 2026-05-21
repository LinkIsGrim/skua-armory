#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Answer an admin's request for the full client addon map. The map is
 * server-only state; we only ship it back to verified admins.
 *
 * Arguments:
 * 0: Requesting admin UID <STRING>
 * 1: Requesting admin player object <OBJECT>
 *
 * Return Value:
 * None — response goes back via QGVAR(clientAddonMapPushed) targetEvent.
 *
 * Public: No
 */

params ["_requestingAdminUID", "_requestingAdminPlayer"];

if (!isServer) exitWith {};

if !(_requestingAdminUID in GVAR(admins)) exitWith {
    ERROR_1("Received client addon map request from non-admin UID %1, ignoring",_requestingAdminUID);
};

[QGVAR(clientAddonMapPushed), [GVAR(clientAddonMap)], _requestingAdminPlayer] call CBA_fnc_targetEvent;
