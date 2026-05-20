#include "script_component.hpp"

if (isServer) then {
    GVAR(admins) = getArray (configFile >> "enableDebugConsole");
    if (!isMultiplayer) then {
        // In singleplayer, the only player is an admin
        GVAR(admins) pushBack getPlayerUID player;
    };

    [QACEGVAR(zeus,createZeus), {
        params ["_unit"];
        if (getPlayerUID _unit in GVAR(admins)) exitWith {}; // Admins doing this is fine

        // Others are not; send them to the shadow realm

        endMission "UnauthorizedZeus";
    }] call CBA_fnc_addEventHandler;

    [QEGVAR(common,clientConnected), {
        params ["_uid", "_hasInterface", "_playerObject"];

        INFO_3("Client with ID %1 and Object %2 connected. Headless: %3",_uid,_playerObject,!_hasInterface);
    }] call CBA_fnc_addEventHandler;

    [QEGVAR(common,clientConnected), LINKFUNC(createAdminZeus)] call CBA_fnc_addEventHandler;

    GVAR(serverAddons) = cba_common_addons;

    // Map structure: UID (STRING) -> ARRAY of ARRAY of STRING: [extraAddons, missingAddons]
    GVAR(clientAddonMap) = createHashMap;

    // Addon monitoring
    [QGVAR(addons), LINKFUNC(onClientAddons)] call CBA_fnc_addEventHandler;
    [QGVAR(requestClientAddons), LINKFUNC(handleRequestClientAddons)] call CBA_fnc_addEventHandler;

    addMissionEventHandler ["PlayerDisconnected", {
        params ["", "_uid"];
        GVAR(clientAddonMap) deleteAt _uid;
    }];

    [QGVAR(grantRequest), LINKFUNC(handleGrantRequest)] call CBA_fnc_addEventHandler;
    [QGVAR(revokeRequest), LINKFUNC(handleRevokeRequest)] call CBA_fnc_addEventHandler;
};

call FUNC(sendClientAddons);

if (hasInterface) then {
    // Refresh the open admin cert menu (no-op if closed) whenever cert state
    // changes anywhere. Server rebroadcasts the canonical events as `_GLOBAL`
    // so clients catch live grants/revokes triggered by any admin.
    [QEV_CERTIFICATION_GRANTED_GLOBAL, {call FUNC(refreshCertMenu)}] call CBA_fnc_addEventHandler;
    [QEV_CERTIFICATION_REVOKED_GLOBAL, {call FUNC(refreshCertMenu)}] call CBA_fnc_addEventHandler;
    [QEV_CERTIFICATION_LIST_CHANGED_GLOBAL, {call FUNC(refreshCertMenu)}] call CBA_fnc_addEventHandler;

    // Catch player joins/leaves so the player listbox stays current without
    // forcing the admin to reopen the menu. `PlayerConnected` is the engine
    // mission event — it fires on every machine, unlike skua_common's
    // clientConnected which is a server-only serverEvent. Defer the refresh
    // a few seconds so allPlayers / the player's unit object are populated
    // by the time we rebuild the listbox.
    addMissionEventHandler ["PlayerConnected", {
        [{call FUNC(refreshCertMenu)}, [], 3] call CBA_fnc_waitAndExecute;
    }];
    addMissionEventHandler ["PlayerDisconnected", {call FUNC(refreshCertMenu)}];
};
