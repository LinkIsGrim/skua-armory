#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

#include "events.hpp"

if (isServer) then {
    GVAR(postLoadCode) = []; // Array of code+args to run after certs are loaded from the database
    [QEGVAR(common,clientConnected), LINKFUNC(onClientConnected)] call CBA_fnc_addEventHandler;
};

ADDON = true;
