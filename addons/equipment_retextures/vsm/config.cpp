#include "script_component.hpp"

class CfgPatches {
    class SUBADDON {
        addonRootClass = QUOTE(COMPONENT);
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {
            "skua_main",
            "VSM_M81_Config",
            "VSM_AOR1_Config",
            "VSM_OGA_Config",
            "VSM_Multicam_Config",
            "VSM_MulticamTropic_Config",
            "VSM_Scorpion_Config",
            "AOR2_Config",
            "MLO_Ghost"
        };
        skipWhenMissingDependencies = 1;
        VERSION_CONFIG;
    };
};

#include "CfgWeapons.hpp"
#include "CfgVehicles.hpp"
