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

    // Grant/revoke callbacks that arrive before the static cert list has
    // loaded are queued here as `[event_type, _playerID, _certID]` triples;
    // fnc_onCertificationListReturn flushes them once GVAR(map) is populated.
    // The extension fires per-player grants as part of database:player_connect
    // and the watchdog may emit grants/revokes between bootstrap and the
    // cert-list push landing.
    GVAR(pendingCertEvents) = [];

    addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback)];
};

ADDON = true;
