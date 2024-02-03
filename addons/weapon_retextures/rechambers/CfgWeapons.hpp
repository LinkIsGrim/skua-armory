class CfgWeapons {

    // Custom Covert II 9mm retextures
    class hgun_9mmC2_F;
    class hgun_9mmC2_classic_F: hgun_9mmC2_F {
        displayName = CSTRING(9mmC2_classic_Name);
        hiddenSelectionsTextures[] = {QPATHTOF(data\acpc2\hgun_ACPC2_classic_co.paa)};
        picture = QPATHTOF(data\acpc2\icon_hgun_ACPC2_classic_F_X_CA.paa);
        baseWeapon = QUOTE(9mmC2_classic_F);
    };
    class hgun_9mmC2_black_F: hgun_9mmC2_F {
        displayName = CSTRING(9mmC2_black_Name);
        hiddenSelectionsTextures[] = {QPATHTOF(data\acpc2\hgun_ACPC2_black_co.paa)};
        picture = QPATHTOF(data\acpc2\icon_hgun_ACPC2_black_F_X_CA.paa);
        baseWeapon = QUOTE(9mmC2_black_F);
    };

    // Negev NG7 retextures
    class LMG_Negev_F;
    class LMG_Negev_blk_F: LMG_Negev_F {
        displayName = CSTRING(LMG_Negev_blk_Name);
        hiddenSelectionsTextures[] = {
            QPATHTOF(data\lmg_zafir\LMG_Zafir_blk_F_01_CO.paa),
            QPATHTOF(data\lmg_zafir\LMG_Zafir_blk_F_02_CO.paa)
        };
        picture = QPATHTOF(data\lmg_zafir\icon_LMG_Zafir_black_F_X_ca.paa);
        baseWeapon = QUOTE(LMG_Negev_blk_F);
    };
};
