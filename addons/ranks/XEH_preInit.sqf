#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

if (isServer) then {
    GVAR(loaded) = false;
    GVAR(list) = []; // Array of hashmaps with id, display_name (ordered by id)
    GVAR(map) = createHashMap; // id (number) -> display_name (string)

    [QEV_RANK_CHANGED, LINKFUNC(onRankChanged)] call CBA_fnc_addEventHandler;
    [QEV_RANK_LIST_CHANGED, LINKFUNC(onRanksListChanged)] call CBA_fnc_addEventHandler;
};

ADDON = true;
