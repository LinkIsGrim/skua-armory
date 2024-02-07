#include "script_component.hpp"

class CfgPatches {
    class SUBADDON {
        addonRootClass = QUOTE(COMPONENT);
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {
            "skua_main",
            "SMA_ELCAN_ALTZOOM_C",
            "sma_vortex_optics",
            "SMA_Eotech_552",
            "ace_attach"
        };
        skipWhenMissingDependencies = 1;
        VERSION_CONFIG;
    };
};

#include "CfgJointRails.hpp"
#include "CfgWeapons.hpp"
