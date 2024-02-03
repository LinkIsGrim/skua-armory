class CfgWeapons {
    class ItemCore;
    class VestItem;
    class Uniform_Base: ItemCore {
        class ItemInfo;
    };
    class Vest_Camo_Base: ItemCore {
        class ItemInfo;
    };

    // Uniforms
    #include "uniforms.hpp"

    // Vests
    #include "vests.hpp"

    // Headgear (done manually)
    #include "headgear.hpp"
};
