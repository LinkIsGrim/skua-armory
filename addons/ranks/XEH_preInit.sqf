#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

if (isServer) then {
    GVAR(loaded) = false;
    GVAR(list) = []; // Array of [id, display_name] pairs, ordered by id
    GVAR(map) = createHashMap; // id (number) -> display_name (string)

    addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback)];
};

ADDON = true;
