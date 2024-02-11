#include "script_component.hpp"

class CfgPatches {
    class SUBADDON {
        addonRootClass = QUOTE(COMPONENT);
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {
            "skua_weapon_tweaks",
            "Weapons_F_JCA_gen4_Rifles_M4A1",
            "Weapons_F_JCA_gen4_Rifles_M4A4",
            "Weapons_F_JCA_gen4_Rifles_SR10",
            "Weapons_F_JCA_gen4_Rifles_SR25"
        };
        skipWhenMissingDependencies = 1;
        VERSION_CONFIG;
    };
};

#include "CfgMagazineWells.hpp"
#include "CfgWeapons.hpp"
