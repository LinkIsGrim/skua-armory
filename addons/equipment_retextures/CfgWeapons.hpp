class CfgWeapons {
    class ItemCore;
    class UniformItem;
    class Uniform_Base: ItemCore {
        class ItemInfo;
    };
    class U_B_W_FullGhillie_wdl_F: Uniform_Base {
        author = "AveryTheKitty";
        scope = 2;
        displayName = "Full Ghillie (Woodland) [NATO]";
        picture = QPATHTOF(data\ghillie\icon_U_B_W_FullGhillie_wdl_F_CA.paa);
        model = "\A3\Characters_F\Common\Suitpacks\suitpack_universal_F.p3d";
        hiddenSelections[] = {"camo"};
        hiddenSelectionsTextures[] = {QPATHTOF(data\ghillie\ghillie_coverall_NATO_CO.paa)};
        class ItemInfo: UniformItem {
            uniformModel = "-";
            uniformClass = "B_W_ghillie_wdl_F";
            containerClass = "Supply60";
            mass = 80;
        };
    };
    class U_O_R_FullGhillie_wdl_F: Uniform_Base {
        author = "AveryTheKitty";
        scope = 2;
        displayName = "Full Ghillie (Woodland) [CSAT]";
        picture = QPATHTOF(data\ghillie\icon_U_O_R_FullGhillie_wdl_F_CA.paa);
        model = "\A3\Characters_F\Common\Suitpacks\suitpack_universal_F.p3d";
        hiddenSelections[] = {"camo"};
        hiddenSelectionsTextures[] = {"\A3\Characters_F\Common\Suitpacks\Data\suitpack_soldier_OPFOR_CO.paa"};
        class ItemInfo: UniformItem {
            uniformModel = "-";
            uniformClass = "O_R_ghillie_wdl_F";
            containerClass = "Supply60";
            mass = 120;
        };
    };
    class U_B_FullGhillie_arc_F: Uniform_Base {
        author = "AveryTheKitty";
        scope = 2;
        displayName = "Full Ghillie (Snow) [NATO]";
        picture = QPATHTOF(data\ghillie\icon_U_B_FullGhillie_arc_F_CA.paa);
        model = "\A3\Characters_F\Common\Suitpacks\suitpack_universal_F.p3d";
        hiddenSelections[] = {"camo"};
        hiddenSelectionsTextures[] = {QPATHTOF(data\ghillie\ghillie_coverall_NATO_arc_CO.paa)};
        class ItemInfo: UniformItem {
            uniformModel = "-";
            uniformClass = "B_ghillie_arc_F";
            containerClass = "Supply60";
            mass = 80;
        };
    };
    class U_O_FullGhillie_arc_F: Uniform_Base {
        author = "AveryTheKitty";
        scope = 2;
        displayName = "Full Ghillie (Snow) [CSAT]";
        picture = QPATHTOF(data\ghillie\icon_U_B_FullGhillie_arc_F_ca.paa);
        model = "\A3\Characters_F\Common\Suitpacks\suitpack_universal_F.p3d";
        hiddenSelections[] = {"camo"};
        hiddenSelectionsTextures[] = {"\A3\Characters_F\OPFOR\Data\clothing_oucamo_co.paa"};
        class ItemInfo: UniformItem {
            uniformModel = "-";
            uniformClass = "O_ghillie_arc_F";
            containerClass = "Supply60";
            mass = 120;
        };
    };
};
