#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Fan-out helper for events that should only land on admin clients. Wraps
 * CBA_fnc_targetEvent against the server's GVAR(adminPlayers) list — the
 * roster of currently-connected admin player units, maintained server-side.
 *
 * Generally only raised on the server. If called from a client, the call is
 * forwarded to the server via serverEvent and the server re-fans out the same
 * way. This keeps non-admin clients off the receiver list and avoids
 * flooding them with admin-only payloads (addon mismatch maps, rosters,
 * offline cert dumps, etc.).
 *
 * Arguments:
 * 0: Event name <STRING>
 * 1: Event args <ARRAY> (optional, default [])
 *
 * Return Value:
 * None.
 *
 * Example:
 * [QGVAR(clientDisconnected), [_uid]] call skua_admin_fnc_adminEvent;
 *
 * Public: Yes
 */

params ["_eventName", ["_args", []]];

if (!isServer) exitWith {
    [QGVAR(adminEventForward), [_eventName, _args]] call CBA_fnc_serverEvent;
};

// No admins connected → nothing to broadcast. Not an error: server-only
// state (clientAddonMap, roster cache) still gets maintained; admins joining
// later will fetch fresh.
if (GVAR(adminPlayers) isEqualTo []) exitWith {};

[_eventName, _args, GVAR(adminPlayers)] call CBA_fnc_targetEvent;
