#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

#include "initSettings.inc.sqf"

if (isServer) then {
    GVAR(state) = DATABASESTATE_AWAITCONNECT;
    GVAR(postLoadCode) = [];

    addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback)];

    [QEGVAR(common,clientConnected), LINKFUNC(onClientConnected)] call CBA_fnc_addEventHandler;

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
