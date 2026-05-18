#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

#include "initSettings.inc.sqf"

// Registered on every machine: the timeout event is a globalEventJIP and each
// client surfaces it in their own systemChat.
[QGVAR(bootstrapTimeout), {
    params ["_state"];
    ERROR_1("Database bootstrap timeout: state stuck at %1 after 60s.",_state);
    if (hasInterface) then {
        systemChat format ["[Skua] Database bootstrap timed out (state: %1). Persistence is unavailable this session.", _state];
    };
}] call CBA_fnc_addEventHandler;

if (isServer) then {
    GVAR(state) = DATABASESTATE_AWAITCONNECT;
    GVAR(postLoadCode) = [];

    addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback)];

    [QEGVAR(common,clientConnected), LINKFUNC(onClientConnected)] call CBA_fnc_addEventHandler;

    addMissionEventHandler ["HandleDisconnect", {
        params ["", "", "_uid", "_name"];
        [_uid, _name] call FUNC(playerDisconnect);
    }];

    // Flush queued code when bootstrap finishes. fnc_runAfterDatabaseInit
    // pushes [code, args] pairs into postLoadCode until state is CONNECTEDINIT.
    [QGVAR(initialized), {
        INFO_1("Database initialized, flushing %1 deferred callbacks.",count GVAR(postLoadCode));
        {
            private _code = _x select 0;
            private _args = _x select 1;
            _args call _code;
        } forEach GVAR(postLoadCode);
        GVAR(postLoadCode) = [];
    }] call CBA_fnc_addEventHandler;
};

ADDON = true;
