#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {
            "skua_main"
        };
        author = "LinkIsGrim";
        name = COMPONENT_NAME;
        VERSION_CONFIG;
    };
};

enableTargetDebug = 1;

#include "CfgEventHandlers.hpp"
#include "CfgFunctions.hpp"
#include "ui\RscModal.hpp"