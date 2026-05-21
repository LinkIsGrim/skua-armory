#include "script_component.hpp"
#include "ui\defines.hpp"

if (isServer) then {
    GVAR(admins) = getArray (configFile >> "enableDebugConsole");
    GVAR(adminPlayers) = [];
    if (!isMultiplayer) then {
        // In singleplayer, the only player is an admin. clientConnected isn't
        // fired in SP (see addons/common/functions/fnc_notifyClientConnected.sqf),
        // so we seed the admin player roster directly.
        GVAR(admins) pushBack getPlayerUID player;
        GVAR(adminPlayers) pushBackUnique player;
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

    // Roster of currently-connected admin player units drives fnc_adminEvent
    // so admin-only payloads (addon mismatch map, roster, offline certs)
    // never land on regular clients. Initialized above; populated here as
    // admins join.
    [QEGVAR(common,clientConnected), {
        params ["_uid", "_hasInterface", "_player"];
        if (!_hasInterface) exitWith {}; // Headless: no UI, no admin.
        if (_uid in GVAR(admins)) then {
            GVAR(adminPlayers) pushBackUnique _player;
        };
    }] call CBA_fnc_addEventHandler;

    // Forward handler: lets non-server callers raise admin events; the server
    // re-fans out via targetEvent.
    [QGVAR(adminEventForward), {
        params ["_eventName", "_args"];
        [_eventName, _args] call FUNC(adminEvent);
    }] call CBA_fnc_addEventHandler;

    GVAR(serverAddons) = cba_common_addons;

    // Map structure: UID (STRING) -> [extraAddons, missingAddons, extrasModMap]
    // where extrasModMap is HASHMAP<addonClass, [modDir, modName]> (client-resolved
    // because the server doesn't have the extras loaded).
    GVAR(clientAddonMap) = createHashMap;

    // Addon monitoring
    [QGVAR(addons), LINKFUNC(onClientAddons)] call CBA_fnc_addEventHandler;
    [QGVAR(requestClientAddons), LINKFUNC(handleRequestClientAddons)] call CBA_fnc_addEventHandler;
    [QGVAR(requestClientAddonMap), LINKFUNC(handleFetchClientAddonMapRequest)] call CBA_fnc_addEventHandler;

    addMissionEventHandler ["PlayerDisconnected", {
        params ["", "_uid"];
        GVAR(clientAddonMap) deleteAt _uid;
        // Prune the leaver from the admin-fan-out roster (the object may
        // already be null at this point — filter both conditions).
        GVAR(adminPlayers) = GVAR(adminPlayers) select {
            !isNull _x && {getPlayerUID _x != _uid}
        };
        // Fan out to admin clients only — we don't use QEV_PLAYER_DISCONNECTED
        // (DB-gated) and we don't want non-admins to receive this either.
        [QGVAR(clientDisconnected), [_uid]] call FUNC(adminEvent);
    }];

    [QGVAR(grantRequest), LINKFUNC(handleGrantRequest)] call CBA_fnc_addEventHandler;
    [QGVAR(revokeRequest), LINKFUNC(handleRevokeRequest)] call CBA_fnc_addEventHandler;

    // Roster/offline-cert fetch routing: client serverEvent → server
    // callExtension → ExtensionCallback adapter globalEvents the parsed
    // payload so any admin client can cache it.
    [QGVAR(fetchRosterRequest), LINKFUNC(handleFetchRosterRequest)] call CBA_fnc_addEventHandler;
    [QGVAR(fetchOfflineCertsRequest), LINKFUNC(handleFetchOfflineCertsRequest)] call CBA_fnc_addEventHandler;
    addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback)];
};

call FUNC(sendClientAddons);

// Disconnect fan-out lands on every machine; admin clients prune their
// uiNamespace cache. No-op on machines that never opened the Addons tab.
[QGVAR(clientDisconnected), LINKFUNC(onClientDisconnected)] call CBA_fnc_addEventHandler;

if (hasInterface) then {
    // Pause menu entry — admins only. CBA_fnc_addPauseMenuOption has no
    // condition param so we gate the registration itself. Same trick as
    // ACE's headbug fix entry in addons/optionsmenu.
    if (player call FUNC(isAdmin)) then {
        [
            ["Admin Menu", "Open the Skua admin menu."],
            QGVAR(PauseMenuHelperAdminMenu)
        ] call CBA_fnc_addPauseMenuOption;
    };

    // Refresh the open admin cert menu (no-op if closed) whenever cert state
    // changes anywhere. Server rebroadcasts the canonical events as `_GLOBAL`
    // so clients catch live grants/revokes triggered by any admin.
    //
    // The grant/revoke handlers also keep the offline-cert cache in sync —
    // online players already update through their unit setVariable broadcast,
    // but offline players' state lives only in the uiNamespace cache. Without
    // this mutation, a grant on an offline player would land in the DB and
    // fire the event, but the Held listbox would still show stale rows until
    // the next force-refresh.
    [QEV_CERTIFICATION_GRANTED_GLOBAL, {
        params ["_data"];
        private _uid = _data get "player_id";
        private _certId = _data get "cert_id";
        private _cache = uiNamespace getVariable [QGVAR(offlineCerts), createHashMap];
        if (_uid in _cache) then {
            (_cache get _uid) pushBackUnique _certId;
        };
        call FUNC(refreshPlayerList);
    }] call CBA_fnc_addEventHandler;
    [QEV_CERTIFICATION_REVOKED_GLOBAL, {
        params ["_data"];
        private _uid = _data get "player_id";
        private _certId = _data get "cert_id";
        private _cache = uiNamespace getVariable [QGVAR(offlineCerts), createHashMap];
        if (_uid in _cache) then {
            _cache set [_uid, (_cache get _uid) - [_certId]];
        };
        call FUNC(refreshPlayerList);
    }] call CBA_fnc_addEventHandler;
    [QEV_CERTIFICATION_LIST_CHANGED_GLOBAL, {call FUNC(refreshPlayerList)}] call CBA_fnc_addEventHandler;

    // Roster + offline-cert fetches: server callExtension → server
    // ExtensionCallback adapter globalEvents the parsed payload here.
    [QGVAR(rosterPushed), LINKFUNC(onPlayerInfoListCallback)] call CBA_fnc_addEventHandler;
    [QGVAR(offlineCertsPushed), LINKFUNC(onCertificationGetPlayerCallback)] call CBA_fnc_addEventHandler;
    // Bulk addon-map response (admin → server → admin) for the Addon List tab.
    [QGVAR(clientAddonMapPushed), LINKFUNC(onClientAddonMapCallback)] call CBA_fnc_addEventHandler;
    // Addon-map cache changed — refresh the open Admin Menu's Addons tab.
    // refreshAddonLists no-ops when the dialog isn't open.
    [QGVAR(addonMapLoaded), {call FUNC(refreshAddonLists)}] call CBA_fnc_addEventHandler;

    // Cache-arrival hooks: when a roster or offline-cert fetch lands, refresh
    // the open menu (no-op when closed). Cert refresh only runs if the newly
    // arrived UID matches the currently selected player — avoids gratuitous
    // listbox rebuilds when other admins' fetches land on this client.
    [QGVAR(rosterLoaded), {call FUNC(refreshPlayerList)}] call CBA_fnc_addEventHandler;
    [QGVAR(offlineCertsLoaded), {
        params ["_granteeUID"];
        private _display = findDisplay IDD_ADMIN_MENU;
        if (isNull _display) exitWith {};
        private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;
        private _sel = lbCurSel _playerList;
        if (_sel >= 0 && {(_playerList lbData _sel) isEqualTo _granteeUID}) then {
            call FUNC(refreshCertLists);
        };
    }] call CBA_fnc_addEventHandler;

    // Catch player joins/leaves so the player listbox stays current without
    // forcing the admin to reopen the menu. `PlayerConnected` is the engine
    // mission event — it fires on every machine, unlike skua_common's
    // clientConnected which is a server-only serverEvent. Defer the refresh
    // a few seconds so allPlayers / the player's unit object are populated
    // by the time we rebuild the listbox.
    addMissionEventHandler ["PlayerConnected", {
        [{call FUNC(refreshPlayerList)}, [], 3] call CBA_fnc_waitAndExecute;
    }];
    addMissionEventHandler ["PlayerDisconnected", {call FUNC(refreshPlayerList)}];
};
