#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        name = QUOTE(COMPONENT);
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {
            "skua_main",
            "ace_arsenal"
        };
        VERSION_CONFIG;
    };
};

#include "ACE_Arsenal_Actions.hpp"
#include "CfgEventHandlers.hpp"
