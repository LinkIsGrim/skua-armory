#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

#include "initSettings.inc.sqf"

addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback_log)];
addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback_event)];

ADDON = true;
