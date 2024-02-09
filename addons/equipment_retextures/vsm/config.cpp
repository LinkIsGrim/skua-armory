#include "script_component.hpp"

class CfgPatches {
    class SUBADDON {
        addonRootClass = QUOTE(COMPONENT);
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {
            "skua_equipment_retextures",
            "VSM_M81_Config",
            "VSM_AOR1_Config",
            "VSM_OGA_Config",
            "VSM_Multicam_Config",
            "VSM_MulticamTropic_Config",
            "VSM_Scorpion_Config",
            "AOR2_Config",
            "VSM_Vests_Config",
            "Black_Vests_Config",
            "Arid_Vests_Config",
            "Alpine_Config",
            "Alpine_Vests_Config",
            "AOR2_Vests_Config",
            "MLO_Ghost"
        };
        skipWhenMissingDependencies = 1;
        VERSION_CONFIG;
    };
};

#include "CfgWeapons.hpp"
#include "CfgVehicles.hpp"
