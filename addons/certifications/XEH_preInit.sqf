#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

#include "events.hpp"

if (isServer) then {
    GVAR(loaded) = false;
    GVAR(list) = []; // Array of Hashmaps containing cert data
    GVAR(map) = createHashMap; // Map of id to cert data for easy lookup
    GVAR(postLoadCode) = []; // Array of code+args to run after certs are loaded from the database

    addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback)];

    // Per-player cert load chains off the database addon's playerReady event:
    // by then the player_info row exists and the DB is connected. Then we
    // defer through runAfterCertsLoaded so the per-player extension call also
    // waits for the global cert list to be loaded.
    [QEGVAR(database,playerReady), {
        params ["_player", "_info"];
        private _uid = getPlayerUID _player;
        [FUNC(loadPlayerCerts), [_uid, true, _player]] call FUNC(runAfterCertsLoaded);
    }] call CBA_fnc_addEventHandler;

    [QGVAR(loaded), {
        GVAR(loaded) = true;
        INFO_1("Certifications loaded, running post-load code. %1 callbacks to run.",count GVAR(postLoadCode));
        {
            private _code = _x select 0;
            private _args = _x select 1;
            _args call _code;
        } forEach GVAR(postLoadCode);
    }] call CBA_fnc_addEventHandler;
};

ADDON = true;
