class CfgWeapons {
    class NVGoggles;
    class ACE_NVG_Wide: NVGoggles {
        class ItemInfo;
    };
    class CLASS(Shemagh_Black_NVG): ACE_NVG_Wide {
        author = "GhostIsSpooky";
        scope = 2;
        displayName = "[SIA] Scarf + NVG (Black)";
        picture = QPATHTOF(data\logo.paa);
        hiddenSelections[] = {"camo"};
        hiddenSelectionsTextures[] = {QPATHTOF(data\Shemagh_Black_co.paa)};
        class ItemInfo: ItemInfo {
            modelOff = QPATHTOF(data\G_Shemagh.p3d);
            hiddenSelections[] = {"camo"};
        };
    };
    class CLASS(Shemagh_Olive_NVG): CLASS(Shemagh_Black_NVG) {
        displayName = "[SIA] Scarf + NVG (Olive)";
        hiddenSelectionsTextures[] = {QPATHTOF(data\Shemagh_Olive_co.paa)};
        class ItemInfo: ItemInfo {
        };
    };
    class CLASS(Shemagh_Coyote_NVG): CLASS(Shemagh_Black_NVG) {
        displayName = "[SIA] Scarf + NVG (Coyote)";
        hiddenSelectionsTextures[] = {QPATHTOF(data\do_equip_co.paa)};
        class ItemInfo: ItemInfo {
        };
    };
};