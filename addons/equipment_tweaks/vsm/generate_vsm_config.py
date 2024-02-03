import re

# yes this updates base classes, no, I don't care. VSM's configs are a mess anyway

cfgWeapons = open('addons\\equipment_tweaks\\vsm\\CfgWeapons.hpp','w')
cfgUniforms = open('addons\\equipment_tweaks\\vsm\\uniforms.hpp','w')
cfgVests = open('addons\\equipment_tweaks\\vsm\\vests.hpp','w')

cfgWeapons.write("""class CfgWeapons {
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
""")
cfgWeapons.close()

# export ingame using dev scripts
uniformMap = [
    ["VSM_Multicam_Crye_SS_tan_pants_Camo","[VSM] Crye G3 Rolled Sleeves (Multicam + Tan Pants)"],
    ["VSM_ProjectHonor_tan_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (ProjectHonor + Tan Pants)"],
    ["VSM_OCP_od_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (OCP + OD Pants)"],
    ["VSM_MulticamTropic_tan_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamTropic + Tan Pants)"],
    ["VSM_ProjectHonor_od_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (ProjectHonor + OD Shirt)"],
    ["VSM_Scorpion_casual_Camo","[VSM] Crye G3 Button Up (Scorpion)"],
    ["VSM_MulticamTropic_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamTropic)"],
    ["VSM_OCP_BDU_od_pants_Camo","[VSM] Battle Dress Uniform (OCP + OD Pants)"],
    ["Multicam_Arid_Black_casual_Camo","[VSM] Crye G3 Black Button Up (Arid)"],
    ["MLO_Multicam_Crye_Black_Shirt_Camo","[VSM] Crye G3 (Multicam + Black Shirt)"],
    ["MLO_ProjectHonor_Crye_Black_Pants_Camo","[VSM] Crye G3 (ProjectHonor + Black Pants)"],
    ["MLO_AOR1_Crye_SS_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (AOR1 + Black Shirt)"],
    ["MLO_MulticamWoodland_Crye_Camo","[VSM] Crye G3 (MulticamWoodland)"],
    ["MLO_MulticamWoodland_Crye_OD_Pants_Camo","[VSM] Crye G3 (MulticamWoodland + OD Pants)"],
    ["VSM_OGA_Crye_SS_OD_tan_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OGA OD + Tan Pants)"],
    ["VSM_Scorpion_Crye_Tee_Camo","[VSM] Crye G3 Tee (Scorpion)"],
    ["VSM_M81_tan_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (M81 + Tan Pants)"],
    ["VSM_AOR1_Crye_tan_pants_Camo","[VSM] Crye G3 (AOR1 + Tan Pants)"],
    ["VSM_CSAT_MulticamTropic_Camo","[VSM] CSAT (MulticamTropic)"],
    ["VSM_OCP_Camo","[VSM] Massif Combat Uniform (OCP)"],
    ["VSM_Scorpion_Camo","[VSM] Massif Combat Uniform (Scorpion)"],
    ["MLO_OGA_Crye_SS_Grey_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (OGA Grey + Black Pants)"],
    ["MLO_OGA_OD_Crye_OD_Black_Shirt_Camo","[VSM] Crye G3 (OGA Black + OD Pants)"],
    ["MLO_OCP_Crye_SS_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (OCP + Black Pants)"],
    ["MLO_MulticamWoodland_Crye_SS_OD_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamWoodland + OD Shirt)"],
    ["MLO_MulticamWoodland_Crye_Grey_Shirt_Camo","[VSM] Crye G3 (MulticamWoodland + Grey Shirt)"],
    ["MLO_MulticamWoodland_OD_Shirt_Camo","[VSM] Massif Combat Uniform (MulticamWoodland + OD Shirt)"],
    ["AOR_2_Crye_Camo","[VSM] Crye G3 (AOR2)"],
    ["VSM_ProjectHonor_BDU_od_pants_Camo","[VSM] Battle Dress Uniform (ProjectHonor + OD Pants)"],
    ["Arid_Arid_Camo","[VSM] Crye G3 (Arid + Tan)"],
    ["VSM_Scorpion_od_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Scorpion + OD Shirt)"],
    ["MLO_OGA_Crye_Black_Camo","[VSM] Crye G3 (OGA Black)"],
    ["MLO_Multicam_Crye_Black_Pants_Camo","[VSM] Crye G3 (Multicam + Black Pants)"],
    ["MLO_MulticamWoodland_Crye_OD_Shirt_Camo","[VSM] Crye G3 (MulticamWoodland + OD Shirt)"],
    ["MLO_MulticamWoodland_Crye_SS_Grey_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamWoodland + Grey Shirt)"],
    ["VSM_ProjectHonor_Crye_Tee_Camo","[VSM] Crye G3 Tee (ProjectHonor)"],
    ["VSM_M81_tan_shirt_Camo","[VSM] Massif Combat Uniform (M81 + Tan Shirt)"],
    ["VSM_MulticamTropic_Crye_grey_shirt_Camo","[VSM] Crye G3 (MulticamTropic + Grey Shirt)"],
    ["VSM_MulticamTropic_BDU_tan_pants_Camo","[VSM] Battle Dress Uniform (MulticamTropic + Tan Pants)"],
    ["VSM_Scorpion_Crye_SS_grey_pants_Camo","[VSM] Crye G3 Rolled Sleeves (Scorpion + Grey Pants)"],
    ["VSM_Multicam_Crye_SS_grey_pants_Camo","[VSM] Crye G3 Rolled Sleeves (Multicam + Grey Pants)"],
    ["VSM_OCP_od_pants_Camo","[VSM] Massif Combat Uniform (OCP + OD Pants)"],
    ["VSM_AOR1_Crye_grey_pants_Camo","[VSM] Crye G3 (AOR1 + Grey Pants)"],
    ["VSM_AOR1_Crye_SS_tan_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (AOR1 + Tan Shirt)"],
    ["VSM_M81_Crye_SS_od_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (M81 + OD Shirt)"],
    ["VSM_Multicam_Crye_SS_od_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (Multicam + OD Shirt)"],
    ["VSM_OGA_od_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (OGA + OD Pants)"],
    ["AOR_2_Crye_Tee_Camo","[VSM] Crye G3 Tee (AOR2)"],
    ["MLO_M81_Crye_Black_Pants_Camo","[VSM] Crye G3 (M81 + Black Pants)"],
    ["MLO_MulticamWoodland_OD_Pants_Camo","[VSM] Massif Combat Uniform (MulticamWoodland + OD Pants)"],
    ["MLO_MulticamWoodland_SS_Tan_Shirt_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamWoodland + Tan Shirt)"],
    ["VSM_AOR1_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (AOR1)"],
    ["VSM_M81_Crye_SS_tan_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (M81 + Tan Shirt)"],
    ["VSM_Multicam_BDU_tan_pants_Camo","[VSM] Battle Dress Uniform (Multicam + Tan Pants)"],
    ["VSM_OGA_Crye_SS_grey_tan_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OGA Grey + Tan Pants)"],
    ["VSM_Scorpion_Crye_tan_pants_Camo","[VSM] Crye G3 (Scorpion/Tan)"],
    ["VSM_ProjectHonor_Crye_SS_grey_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (ProjectHonor + Grey Shirt)"],
    ["VSM_MulticamTropic_od_shirt_Camo","[VSM] Massif Combat Uniform (MulticamTropic + OD Shirt)"],
    ["VSM_OGA_Crye_od_grey_pants_Camo","[VSM] Crye G3 (OD/Grey)"],
    ["Alpine_Crye_Camo","[VSM] Crye G3 (Alpine)"],
    ["VSM_Scorpion_od_shirt_Camo","[VSM] Massif Combat Uniform (Scorpion + OD Shirt)"],
    ["MLO_MulticamWoodland_Tan_Shirt_Camo","[VSM] Massif Combat Uniform (MulticamWoodland + Tan Shirt)"],
    ["Multicam_Alpine_casual_Camo","[VSM] Crye G3 Black Button Up (Alpine)"],
    ["VSM_ProjectHonor_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (ProjectHonor)"],
    ["VSM_OGA_Crye_grey_od_pants_Camo","[VSM] Crye G3 (Grey/OD)"],
    ["VSM_OCP_tan_pants_Camo","[VSM] Massif Combat Uniform (OCP + Tan Pants)"],
    ["VSM_AOR1_Crye_SS_grey_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (AOR1 + Grey Shirt)"],
    ["VSM_Scorpion_Crye_SS_od_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (Scorpion + OD Shirt)"],
    ["AOR2_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (AOR2)"],
    ["MLO_OGA_Crye_Black_Shirt_Camo","[VSM] Crye G3 (OGA Black + Tan Pants)"],
    ["MLO_OGA_Black_Pants_ACU_Camo","[VSM] Massif Combat Uniform (OGA + Black Pants)"],
    ["VSM_M81_Crye_SS_od_pants_Camo","[VSM] Crye G3 Rolled Sleeves (M81 + OD Pants)"],
    ["VSM_AOR1_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (AOR1)"],
    ["VSM_Multicam_BDU_od_pants_Camo","[VSM] Battle Dress Uniform (Multicam + OD Pants)"],
    ["VSM_OGA_od_pants_Camo","[VSM] Massif Combat Uniform (OGA + OD Pants)"],
    ["VSM_AOR1_Crye_SS_grey_pants_Camo","[VSM] Crye G3 Rolled Sleeves (AOR1 + Grey Pants)"],
    ["VSM_M81_casual_Camo","[VSM] Crye G3 Button Up (M81)"],
    ["VSM_OCP_Crye_tan_pants_Camo","[VSM] Crye G3 (OCP + Tan Pants)"],
    ["VSM_ProjectHonor_od_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (ProjectHonor + OD Pants)"],
    ["VSM_OCP_Crye_SS_tan_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (OCP + Tan Shirt)"],
    ["VSM_OGA_Crye_od_pants_Camo","[VSM] Crye G3 (OD)"],
    ["VSM_MulticamTropic_od_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamTropic + OD Pants)"],
    ["VSM_OGA_OD_casual_Camo","[VSM] Crye G3 Button Up (OGA OD)"],
    ["AOR2_SS_camo","[VSM] Massif Combat Uniform Rolled Sleeves (AOR2)"],
    ["Black_Black_SS_Camo","[VSM] Crye G3 Rolled Sleeves (MC Black + Black)"],
    ["MLO_Black_OD_Pants_ACU_SS_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (OGA Black + OD Pants)"],
    ["MLO_MulticamWoodland_Black_Shirt_Camo","[VSM] Massif Combat Uniform (MulticamWoodland + Black Shirt)"],
    ["MLO_MulticamWoodland_Tan_Pants_Camo","[VSM] Massif Combat Uniform (MulticamWoodland + Tan Pants)"],
    ["VSM_M81_Crye_SS_grey_pants_Camo","[VSM] Crye G3 Rolled Sleeves (M81 + Grey Pants)"],
    ["VSM_Multicam_Crye_tan_pants_Camo","[VSM] Crye G3 (Multicam + Tan Pants)"],
    ["VSM_Scorpion_Crye_Camo","[VSM] Crye G3 (Scorpion)"],
    ["VSM_Multicam_od_pants_Camo","[VSM] Massif Combat Uniform (Multicam + OD Pants)"],
    ["VSM_OGA_Crye_Grey_pants_Camo","[VSM] Crye G3 (Tan/Grey)"],
    ["VSM_M81_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (M81)"],
    ["VSM_Multicam_casual_Camo","[VSM] Crye G3 Button Up (Multicam)"],
    ["VSM_MulticamTropic_tan_shirt_Camo","[VSM] Massif Combat Uniform (MulticamTropic + Tan Shirt)"],
    ["VSM_OCP_Crye_od_shirt_Camo","[VSM] Crye G3 (OCP + OD Shirt)"],
    ["VSM_AOR1_Crye_SS_od_pants_Camo","[VSM] Crye G3 Rolled Sleeves (AOR1 + OD Pants)"],
    ["VSM_MulticamTropic_Crye_SS_tan_pants_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamTropic + Tan Pants)"],
    ["Alpine_white_Crye_camo","[VSM] Crye G3 (White + Alpine)"],
    ["VSM_ProjectHonor_od_shirt_Camo","[VSM] Massif Combat Uniform (ProjectHonor + OD Shirt)"],
    ["VSM_MulticamTropic_tan_pants_Camo","[VSM] Massif Combat Uniform (MulticamTropic + Tan Pants)"],
    ["VSM_M81_od_pants_Camo","[VSM] Massif Combat Uniform (M81 + OD Pants)"],
    ["Multicam_black_casual_Camo","[VSM] Crye G3 Black Button Up (MC Black)"],
    ["VSM_OGA_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (OGA)"],
    ["MLO_OGA_Crye_Grey_Black_Shirt_Camo","[VSM] Crye G3 (OGA Black + Grey Pants)"],
    ["MLO_Scorpion_Crye_SS_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (Scorpion + Black Pants)"],
    ["MLO_M81_Crye_SS_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (M81 + Black Pants)"],
    ["MLO_Black_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (Black)"],
    ["VSM_OGA_grey_casual_Camo","[VSM] Crye G3 Button Up (OGA Grey)"],
    ["VSM_ProjectHonor_Crye_SS_tan_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (ProjectHonor + Tan Shirt)"],
    ["VSM_OGA_od_tan_pants_Camo","[VSM] Massif Combat Uniform (OGA OD + Tan Pants)"],
    ["VSM_M81_od_shirt_Camo","[VSM] Massif Combat Uniform (M81 + OD Shirt)"],
    ["VSM_OGA_Crye_SS_od_Camo","[VSM] Crye G3 Rolled Sleeves (OGA OD)"],
    ["VSM_ProjectHonor_Crye_grey_shirt_Camo","[VSM] Crye G3 (ProjectHonor + Grey Shirt)"],
    ["MLO_OGA_Crye_SS_Grey_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (OGA Black + Grey Pants)"],
    ["MLO_OCP_Crye_Black_Pants_Camo","[VSM] Crye G3 (OCP + Black Pants)"],
    ["MLO_MulticamWoodland_Crye_SS_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamWoodland + Black Shirt)"],
    ["MLO_MulticamWoodland_SS_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamWoodland)"],
    ["VSM_OCP_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (OCP)"],
    ["VSM_MulticamTropic_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (MulticamTropic)"],
    ["VSM_M81_od_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (M81 + OD Pants)"],
    ["VSM_OCP_Crye_SS_grey_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OCP + Grey Pants)"],
    ["Arid_Arid_SS_Camo","[VSM] Crye G3 Rolled Sleeves (Arid + Tan)"],
    ["MLO_OGA_OD_Crye_SS_OD_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (OGA Black + OD Pants)"],
    ["VSM_OCP_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (OCP)"],
    ["VSM_M81_Crye_SS_tan_pants_Camo","[VSM] Crye G3 Rolled Sleeves (M81 + Tan Pants)"],
    ["VSM_OCP_Crye_SS_tan_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OCP + Tan Pants)"],
    ["VSM_AOR1_Crye_tan_shirt_Camo","[VSM] Crye G3 (AOR1 + Tan Shirt)"],
    ["VSM_Scorpion_tan_shirt_Camo","[VSM] Massif Combat Uniform (Scorpion + Tan Shirt)"],
    ["MLO_OGA_OD_Crye_SS_OD_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (OGA OD + Black Pants)"],
    ["MLO_Scorpion_Crye_Black_Shirt_Camo","[VSM] Crye G3 (Black/Scorpion)"],
    ["MLO_OGA_OD_Black_Pants_ACU_Camo","[VSM] Massif Combat Uniform (OGA OD + Black Pants)"],
    ["MLO_MulticamWoodland_SS_OD_Shirt_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamWoodland + OD Shirt)"],
    ["MLO_MulticamWoodland_SS_OD_Pants_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamWoodland + OD Pants)"],
    ["VSM_Multicam_Crye_Tee_Camo","[VSM] Crye G3 Tee (Multicam)"],
    ["VSM_CSAT_AOR2_Camo","[VSM] CSAT (AOR2)"],
    ["VSM_MulticamTropic_Crye_SS_od_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamTropic + OD Shirt)"],
    ["VSM_ProjectHonor_Camo","[VSM] Massif Combat Uniform (ProjectHonor)"],
    ["AOR_2_Grey_Crye_Camo","[VSM] Crye G3 (AOR2 + Green Shirt)"],
    ["VSM_Multicam_Crye_grey_pants_Camo","[VSM] Crye G3 (Multicam + Grey Pants)"],
    ["VSM_ProjectHonor_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (ProjectHonor)"],
    ["VSM_AOR1_od_pants_Camo","[VSM] Massif Combat Uniform (AOR1 + OD Pants)"],
    ["VSM_M81_Crye_SS_grey_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (M81 + Grey Shirt)"],
    ["VSM_MulticamTropic_tan_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamTropic + Tan Shirt)"],
    ["VSM_ProjectHonor_tan_pants_Camo","[VSM] Massif Combat Uniform (ProjectHonor + Tan Pants)"],
    ["VSM_Scorpion_Crye_grey_shirt_Camo","[VSM] Crye G3 (Grey/Scorpion)"],
    ["VSM_Scorpion_tan_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Scorpion + Tan Pants)"],
    ["VSM_OGA_OD_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (OGA OD)"],
    ["MLO_OGA_Crye_SS_Black_Camo","[VSM] Crye G3 Rolled Sleeves (OGA Black)"],
    ["MLO_OGA_OD_Crye_OD_Black_Pants_Camo","[VSM] Crye G3 (OGA OD + Black Pants)"],
    ["MLO_OGA_Crye_SS_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (OGA Black + Tan Pants)"],
    ["MLO_OGA_Crye_Black_pants_Camo","[VSM] Crye G3 (OGA + Black Pants)"],
    ["MLO_MulticamWoodland_Crye_Grey_Pants_Camo","[VSM] Crye G3 (MulticamWoodland + Grey Pants)"],
    ["VSM_AOR1_Camo","[VSM] Massif Combat Uniform (AOR1)"],
    ["VSM_MulticamTropic_Crye_tan_pants_Camo","[VSM] Crye G3 (MulticamTropic + Tan Pants)"],
    ["VSM_AOR1_casual_Camo","[VSM] Crye G3 Button Up (AOR1)"],
    ["AOR_2_BlkCasual_camo","[VSM] Crye G3 Black Button Up (AOR2)"],
    ["VSM_OCP_Crye_SS_od_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (OCP + OD Shirt)"],
    ["Multicam_Arid_casual_Camo","[VSM] Crye G3 Tan Button Up (Arid)"],
    ["DTS_BDU_Camo","[VSM] Battle Dress Uniform (Desert Tiger Stripe)"],
    ["VSM_Scorpion_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (Scorpion)"],
    ["MLO_OGA_Crye_SS_Black_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OGA + Black Pants)"],
    ["MLO_Black_ACU_Camo","[VSM] Massif Combat Uniform (Black)"],
    ["VSM_AOR1_Crye_od_shirt_Camo","[VSM] Crye G3 (AOR1 + OD Shirt)"],
    ["VSM_M81_Crye_tan_shirt_Camo","[VSM] Crye G3 (M81 + Tan Shirt)"],
    ["VSM_M81_Crye_od_shirt_Camo","[VSM] Crye G3 (M81 + OD Shirt)"],
    ["VSM_OCP_tan_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (OCP + Tan Shirt)"],
    ["VSM_AOR1_od_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (AOR1 + OD Shirt)"],
    ["VSM_AOR1_Crye_Camo","[VSM] Crye G3 (AOR1)"],
    ["VSM_AOR1_BDU_Camo","[VSM] Battle Dress Uniform (AOR1)"],
    ["VSM_OCP_Crye_SS_od_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OCP + OD Pants)"],
    ["VSM_Scorpion_Crye_SS_od_pants_Camo","[VSM] Crye G3 Rolled Sleeves (Scorpion + OD Pants)"],
    ["VSM_MulticamTropic_Crye_grey_pants_Camo","[VSM] Crye G3 (MulticamTropic + Grey Pants)"],
    ["black_Crye_Camo","[VSM] Crye G3 (MC Black)"],
    ["VSM_M81_Crye_tan_pants_Camo","[VSM] Crye G3 (M81 + Tan Pants)"],
    ["AOR_2_TCasual_camo","[VSM] Crye G3 Tan Button Up (AOR2)"],
    ["VSM_AOR1_Crye_grey_shirt_Camo","[VSM] Crye G3 (AOR1 + Grey Shirt)"],
    ["VSM_OGA_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (OGA)"],
    ["VSM_M81_BDU_Camo","[VSM] Battle Dress Uniform (M81)"],
    ["VSM_OCP_tan_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (OCP + Tan Pants)"],
    ["VSM_OGA_Crye_grey_tan_pants_Camo","[VSM] Crye G3 (Grey/Tan)"],
    ["VSM_ProjectHonor_Crye_od_pants_Camo","[VSM] Crye G3 (ProjectHonor + OD Pants)"],
    ["MLO_Scorpion_Crye_Black_Pants_Camo","[VSM] Crye G3 (Scorpion/Black)"],
    ["MLO_M81_Crye_SS_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (M81 + Black Shirt)"],
    ["MLO_Black_Tan_Pants_ACU_Camo","[VSM] Massif Combat Uniform (OGA Black + Tan Pants)"],
    ["MLO_Black_OD_Pants_ACU_Camo","[VSM] Massif Combat Uniform (OGA Black + OD Pants)"],
    ["MLO_MulticamWoodland_Crye_SS_OD_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamWoodland + OD Pants)"],
    ["MLO_MulticamWoodland_Crye_SS_Grey_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamWoodland + Grey Pants)"],
    ["MLO_MulticamWoodland_SS_Black_Shirt_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamWoodland + Black Shirt)"],
    ["VSM_OCP_BDU_Camo","[VSM] Battle Dress Uniform (OCP)"],
    ["VSM_Multicam_BDU_Camo","[VSM] Battle Dress Uniform (Multicam)"],
    ["VSM_Multicam_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (Multicam)"],
    ["VSM_AOR1_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (AOR1)"],
    ["VSM_Scorpion_Crye_grey_pants_Camo","[VSM] Crye G3 (Scorpion/Grey)"],
    ["VSM_Multicam_od_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Multicam + OD Shirt)"],
    ["VSM_Multicam_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Multicam)"],
    ["MLO_MulticamTropic_Crye_SS_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamTropic + Black Pants)"],
    ["MLO_ProjectHonor_Crye_Black_Shirt_Camo","[VSM] Crye G3 (ProjectHonor + Black Shirt)"],
    ["VSM_Scorpion_Crye_SS_tan_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (Scorpion + Tan Shirt)"],
    ["VSM_OGA_Crye_od_Camo","[VSM] Crye G3 (OD)"],
    ["VSM_ProjectHonor_od_pants_Camo","[VSM] Massif Combat Uniform (ProjectHonor + OD Pants)"],
    ["VSM_AOR1_BDU_tan_pants_Camo","[VSM] Battle Dress Uniform (AOR1 + Tan Pants)"],
    ["VSM_OCP_tan_shirt_Camo","[VSM] Massif Combat Uniform (OCP + Tan Shirt)"],
    ["VSM_ProjectHonor_casual_Camo","[VSM] Crye G3 Button Up (ProjectHonor)"],
    ["VSM_Multicam_Crye_od_shirt_Camo","[VSM] Crye G3 (Multicam + OD Shirt)"],
    ["VSM_OGA_Crye_SS_grey_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OGA + Grey Pants)"],
    ["VSM_MulticamTropic_Crye_SS_grey_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamTropic + Grey Shirt)"],
    ["VSM_OCP_Crye_grey_shirt_Camo","[VSM] Crye G3 (OCP + Grey Shirt)"],
    ["VSM_M81_od_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (M81 + OD Shirt)"],
    ["DTS_Crye_SS_Camo","[VSM] Crey G3 Rolled Sleeves (Desert Tiger Stripe)"],
    ["VSM_Scorpion_od_pants_Camo","[VSM] Massif Combat Uniform (Scorpion + OD Pants)"],
    ["VSM_AOR1_tan_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (AOR1 + Tan Shirt)"],
    ["VSM_Multicam_tan_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Multicam + Tan Pants)"],
    ["VSM_OCP_Crye_od_pants_Camo","[VSM] Crye G3 (OCP + OD Pants)"],
    ["VSM_OGA_tan_casual_Camo","[VSM] Crye G3 Button Up (OGA)"],
    ["VSM_Scorpion_Crye_od_pants_Camo","[VSM] Crye G3 (Scorpion/OD)"],
    ["VSM_Multicam_Crye_SS_grey_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (Multicam + Grey Shirt)"],
    ["VSM_Scorpion_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (Scorpion)"],
    ["AOR_2_BCasual_camo","[VSM] Crye G3 Brown Button Up (AOR2)"],
    ["VSM_Scorpion_tan_pants_Camo","[VSM] Massif Combat Uniform (Scorpion + Tan Pants)"],
    ["MLO_ProjectHonor_Crye_SS_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (ProjectHonor + Black Pants)"],
    ["VSM_Multicam_Crye_SS_od_pants_Camo","[VSM] Crye G3 Rolled Sleeves (Multicam + OD Pants)"],
    ["VSM_M81_Crye_grey_shirt_Camo","[VSM] Crye G3 (M81 + Grey Shirt)"],
    ["VSM_Scorpion_Crye_SS_grey_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (Scorpion + Grey Shirt)"],
    ["VSM_M81_Camo","[VSM] Massif Combat Uniform (M81)"],
    ["VSM_MulticamTropic_Crye_Tee_Camo","[VSM] Crye G3 Tee (MulticamTropic)"],
    ["AOR_2_CryeSS_Camo","[VSM] Crye G3 Rolled Sleeves (AOR2)"],
    ["DTS_Massif_Camo","[VSM] Massif Combat Uniform (Desert Tiger Stripe)"],
    ["VSM_Scorpion_od_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Scorpion + OD Pants)"],
    ["MLO_Black_Tan_Pants_ACU_SS_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (OGA Black + Tan Pants)"],
    ["MLO_MulticamWoodland_SS_Black_Pants_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamWoodland + Black Pants)"],
    ["Alpine_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (Alpine)"],
    ["VSM_MulticamTropic_BDU_Camo","[VSM] Battle Dress Uniform (MulticamTropic)"],
    ["VSM_OGA_od_Camo","[VSM] Massif Combat Uniform (OGA OD)"],
    ["VSM_Scorpion_Crye_od_shirt_Camo","[VSM] Crye G3 (OD/Scorpion)"],
    ["VSM_OGA_Crye_od_tan_pants_Camo","[VSM] Crye G3 (OD/Tan)"],
    ["VSM_MulticamTropic_BDU_od_pants_Camo","[VSM] Battle Dress Uniform (MulticamTropic + OD Pants)"],
    ["MLO_MulticamWoodland_Crye_Black_Shirt_Camo","[VSM] Crye G3 (MulticamWoodland + Black Shirt)"],
    ["VSM_Multicam_od_shirt_Camo","[VSM] Massif Combat Uniform (Multicam + OD Shirt)"],
    ["VSM_MulticamTropic_Crye_Camo","[VSM] Crye G3 (MulticamTropic)"],
    ["VSM_AOR1_Crye_SS_od_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (AOR1 + OD Shirt)"],
    ["VSM_Multicam_Crye_grey_shirt_Camo","[VSM] Crye G3 (Multicam + Grey Shirt)"],
    ["VSM_Scorpion_Crye_SS_tan_pants_Camo","[VSM] Crye G3 Rolled Sleeves (Scorpion + Tan Pants)"],
    ["VSM_M81_BDU_tan_pants_Camo","[VSM] Battle Dress Uniform (M81 + Tan Pants)"],
    ["VSM_ProjectHonor_Crye_SS_od_pants_Camo","[VSM] Crye G3 Rolled Sleeves (ProjectHonor + OD Pants)"],
    ["VSM_MulticamTropic_Crye_SS_od_pants_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamTropic + OD Pants)"],
    ["VSM_OGA_Crye_Camo","[VSM] Crye G3 (Tan)"],
    ["Black_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (MC Black)"],
    ["VSM_Scorpion_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Scorpion)"],
    ["VSM_MulticamTropic_Crye_od_pants_Camo","[VSM] Crye G3 (MulticamTropic + OD Pants)"],
    ["VSM_Multicam_Camo","[VSM] Massif Combat Uniform (Multicam)"],
    ["VSM_Multicam_od_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Multicam + OD Pants)"],
    ["VSM_M81_Crye_Camo","[VSM] Crye G3 (M81)"],
    ["VSM_MulticamTropic_Crye_tan_shirt_Camo","[VSM] Crye G3 (MulticamTropic + Tan Shirt)"],
    ["VSM_AOR1_Crye_Tee_Camo","[VSM] Crye G3 Tee (AOR1)"],
    ["VSM_MulticamTropic_od_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamTropic + OD Shirt)"],
    ["VSM_OGA_Crye_SS_od_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OGA + OD Pants)"],
    ["VSM_ProjectHonor_Crye_od_shirt_Camo","[VSM] Crye G3 (ProjectHonor + OD Shirt)"],
    ["VSM_MulticamTropic_casual_Camo","[VSM] Crye G3 Button Up (MulticamTropic)"],
    ["AOR2_camo","[VSM] Massif Combat Uniform (AOR2)"],
    ["Arid_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (Arid)"],
    ["MLO_MulticamTropic_Crye_Black_Pants_Camo","[VSM] Crye G3 (MulticamTropic + Black Pants)"],
    ["MLO_AOR1_Crye_Black_Pants_Camo","[VSM] Crye G3 (AOR1 + Black Pants)"],
    ["MLO_MulticamWoodland_Crye_Black_Pants_Camo","[VSM] Crye G3 (MulticamWoodland + Black Pants)"],
    ["VSM_Multicam_Crye_Camo","[VSM] Crye G3 (Multicam)"],
    ["VSM_MulticamTropic_od_pants_Camo","[VSM] Massif Combat Uniform (MulticamTropic + OD Pants)"],
    ["VSM_OCP_Crye_Camo","[VSM] Crye G3 (OCP)"],
    ["AOR_2_GreySS_Crye_Camo","[VSM] Crye G3 Rolled Sleeves (AOR2 + Green Shirt)"],
    ["VSM_OCP_Crye_grey_pants_Camo","[VSM] Crye G3 (OCP + Grey Pants)"],
    ["VSM_OGA_od_tan_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (OD + Tan Pants)"],
    ["VSM_MulticamTropic_Crye_SS_grey_pants_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamTropic + Grey Pants)"],
    ["VSM_MulticamTropic_Crye_od_shirt_Camo","[VSM] Crye G3 (MulticamTropic + OD Shirt)"],
    ["Arid_Crye_Camo","[VSM] Crye G3 (Arid)"],
    ["Multicam_Arid_Blue_casual_Camo","[VSM] Crye G3 Blue Button Up (Arid)"],
    ["MLO_M81_Crye_Black_Shirt_Camo","[VSM] Crye G3 (M81 + Black Shirt)"],
    ["VSM_Multicam_Crye_SS_tan_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (Multicam + Tan Shirt)"],
    ["VSM_OGA_Crye_SS_grey_Camo","[VSM] Crye G3 Rolled Sleeves (OGA Grey)"],
    ["VSM_ProjectHonor_Crye_Camo","[VSM] Crye G3 (ProjectHonor)"],
    ["VSM_M81_Crye_grey_pants_Camo","[VSM] Crye G3 (M81 + Grey Pants)"],
    ["VSM_OCP_Crye_Tee_Camo","[VSM] Crye G3 Tee (OCP)"],
    ["VSM_Scorpion_Crye_tan_shirt_Camo","[VSM] Crye G3 (Tan/Scorpion)"],
    ["VSM_AOR1_tan_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (AOR1 + Tan Pants)"],
    ["VSM_OCP_BDU_tan_pants_Camo","[VSM] Battle Dress Uniform (OCP + Tan Pants)"],
    ["VSM_ProjectHonor_Crye_tan_shirt_Camo","[VSM] Crye G3 (ProjectHonor + Tan Shirt)"],
    ["VSM_AOR1_tan_pants_Camo","[VSM] Massif Combat Uniform (AOR1 + Tan Pants)"],
    ["MLO_MulticamWoodland_Camo","[VSM] Massif Combat Uniform (MulticamWoodland)"],
    ["Alpine_Massif_Camo","[VSM] Massif Combat Uniform (Alpine)"],
    ["VSM_M81_tan_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (M81 + Tan Shirt)"],
    ["VSM_M81_Crye_Tee_Camo","[VSM] Crye G3 Tee (M81)"],
    ["VSM_OCP_Crye_tan_shirt_Camo","[VSM] Crye G3 (OCP + Tan Shirt)"],
    ["VSM_OGA_Crye_SS_OD_grey_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OGA OD + Grey Pants)"],
    ["DTS_Massif_SS_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (Desert Tiger Stripe)"],
    ["MLO_MulticamTropic_Crye_Black_Shirt_Camo","[VSM] Crye G3 (MulticamTropic + Black Shirt)"],
    ["VSM_ProjectHonor_tan_shirt_Camo","[VSM] Massif Combat Uniform (ProjectHonor + Tan Shirt)"],
    ["VSM_Multicam_Crye_od_pants_Camo","[VSM] Crye G3 (Multicam + OD Pants)"],
    ["VSM_OCP_Crye_SS_grey_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (OCP + Grey Shirt)"],
    ["VSM_MulticamTropic_Camo","[VSM] Massif Combat Uniform (MulticamTropic)"],
    ["VSM_ProjectHonor_Crye_SS_od_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (ProjectHonor + OD Shirt)"],
    ["VSM_ProjectHonor_BDU_tan_pants_Camo","[VSM] Battle Dress Uniform (ProjectHonor + Tan Pants)"],
    ["VSM_OCP_od_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (OCP + OD Shirt)"],
    ["MLO_Multicam_Crye_SS_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (Multicam + Black Shirt)"],
    ["MLO_OGA_OD_Black_Pants_ACU_SS_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (OGA OD + Black Pants)"],
    ["VSM_OGA_od_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (OGA OD)"],
    ["VSM_ProjectHonor_Crye_SS_grey_pants_Camo","[VSM] Crye G3 Rolled Sleeves (ProjectHonor + Grey Pants)"],
    ["VSM_OCP_casual_Camo","[VSM] Crye G3 Button Up (OCP)"],
    ["VSM_M81_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (M81)"],
    ["Alpine_Massif_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Alpine)"],
    ["VSM_M81_Crye_od_pants_Camo","[VSM] Crye G3 (M81 + OD Pants)"],
    ["VSM_ProjectHonor_Crye_tan_pants_Camo","[VSM] Crye G3 (ProjectHonor + Tan Pants)"],
    ["MLO_Black_ACU_SS_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (Black)"],
    ["MLO_MulticamWoodland_Crye_Tan_Shirt_Camo","[VSM] Crye G3 (MulticamWoodland + Tan Shirt)"],
    ["VSM_Multicam_tan_shirt_Camo","[VSM] Massif Combat Uniform (Multicam + Tan Shirt)"],
    ["VSM_Multicam_Camo_TShirt","[VSM] Massif Combat Uniform T-Shirt (Multicam)"],
    ["VSM_M81_BDU_od_pants_Camo","[VSM] Battle Dress Uniform (M81 + OD Pants)"],
    ["VSM_OGA_Camo","[VSM] Massif Combat Uniform (OGA)"],
    ["DTS_Crye_Camo","[VSM] Crey G3 (Desert Tiger Stripe)"],
    ["Black_Black_Camo","[VSM] Crye G3 (MC Black + Black)"],
    ["MLO_Scorpion_Crye_SS_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (Scorpion + Black Shirt)"],
    ["MLO_ProjectHonor_Crye_SS_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (ProjectHonor + Black Shirt)"],
    ["MLO_AOR1_Crye_SS_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (AOR1 + Black Pants)"],
    ["MLO_MulticamWoodland_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamWoodland)"],
    ["MLO_MulticamWoodland_Crye_Tan_Pants_Camo","[VSM] Crye G3 (MulticamWoodland + Tan Pants)"],
    ["VSM_AOR1_Crye_SS_tan_pants_Camo","[VSM] Crye G3 Rolled Sleeves (AOR1 + Tan Pants)"],
    ["VSM_OCP_od_shirt_Camo","[VSM] Massif Combat Uniform (OCP + OD Shirt)"],
    ["VSM_Multicam_tan_pants_Camo","[VSM] Massif Combat Uniform (Multicam + Tan Pants)"],
    ["VSM_OGA_Crye_grey_Camo","[VSM] Crye G3 (Grey)"],
    ["VSM_ProjectHonor_Crye_SS_tan_pants_Camo","[VSM] Crye G3 Rolled Sleeves (ProjectHonor + Tan Pants)"],
    ["VSM_M81_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (M81)"],
    ["VSM_AOR1_tan_shirt_Camo","[VSM] Massif Combat Uniform (AOR1 + Tan Shirt)"],
    ["VSM_OGA_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (OGA)"],
    ["MLO_MulticamTropic_Crye_SS_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamTropic + Black Shirt)"],
    ["MLO_MulticamWoodland_Crye_SS_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamWoodland + Black Pants)"],
    ["MLO_MulticamWoodland_Crye_SS_Tan_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamWoodland + Tan Pants)"],
    ["MLO_MulticamWoodland_Black_Pants_Camo","[VSM] Massif Combat Uniform (MulticamWoodland + Black Pants)"],
    ["Alpine_white_Crye_SS_camo","[VSM] Crye G3 Rolled Sleeves (White + Alpine)"],
    ["VSM_AOR1_od_pants_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (AOR1 + OD Pants)"],
    ["VSM_ProjectHonor_tan_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (ProjectHonor + Tan Shirt)"],
    ["VSM_Scorpion_tan_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Scorpion + Tan Shirt)"],
    ["MLO_OGA_Crye_Grey_Black_Pants_Camo","[VSM] Crye G3 (OGA Grey + Black Pants)"],
    ["MLO_OCP_Crye_Black_Shirt_Camo","[VSM] Crye G3 (OCP + Black Shirt)"],
    ["MLO_OCP_Crye_SS_Black_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (OCP + Black Shirt)"],
    ["MLO_OGA_Black_Pants_ACU_SS_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (OGA + Black Pants)"],
    ["VSM_AOR1_od_shirt_Camo","[VSM] Massif Combat Uniform (AOR1 + OD Shirt)"],
    ["VSM_Multicam_Crye_tan_shirt_Camo","[VSM] Crye G3 (Multicam + Tan Shirt)"],
    ["VSM_OCP_Crye_SS_Camo","[VSM] Crye G3 Rolled Sleeves (OCP)"],
    ["VSM_M81_tan_pants_Camo","[VSM] Massif Combat Uniform (M81 + Tan Pants)"],
    ["VSM_MulticamTropic_Crye_SS_tan_shirt_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamTropic + Tan Shirt)"],
    ["VSM_ProjectHonor_BDU_Camo","[VSM] Battle Dress Uniform (ProjectHonor)"],
    ["VSM_AOR1_BDU_od_pants_Camo","[VSM] Battle Dress Uniform (AOR1 + OD Pants)"],
    ["VSM_MulticamTropic_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamTropic)"],
    ["VSM_Multicam_tan_shirt_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (Multicam + Tan Shirt)"],
    ["VSM_ProjectHonor_Crye_grey_pants_Camo","[VSM] Crye G3 (ProjectHonor + Grey Pants)"],
    ["VSM_ProjectHonor_Camo_SS","[VSM] Massif Combat Uniform Rolled Sleeves (ProjectHonor)"],
    ["VSM_AOR1_Crye_od_pants_Camo","[VSM] Crye G3 (AOR1 + OD Pants)"],
    ["VSM_OGA_Crye_SS_grey_od_pants_Camo","[VSM] Crye G3 Rolled Sleeves (OGA Grey + OD Pants)"],
    ["MLO_Multicam_Crye_SS_Black_Pants_Camo","[VSM] Crye G3 Rolled Sleeves (Multicam + Black Pants)"],
    ["MLO_AOR1_Crye_Black_Shirt_Camo","[VSM] Crye G3 (AOR1 + Black Shirt)"],
    ["MLO_black_casual_camo","[VSM] Crye G3 Button Up (Black)"],
    ["MLO_MulticamWoodland_Crye_SS_Tan_Shirt_Camo","[VSM] Crye G3 Rolled Sleeves (MulticamWoodland + Tan Shirt)"],
    ["MLO_MulticamWoodland_SS_Tan_Pants_Camo","[VSM] Massif Combat Uniform Rolled Sleeves (MulticamWoodland + Tan Pants)"]
]

uniformStrSingle = """
    class {0}: Uniform_Base {{
        CRYE_G3_DISPLAY_NAME_SINGLE({1});
        UNIFORM_ITEM_INFO;
    }};
"""

uniformStrDual = """
    class {0}: Uniform_Base {{
        CRYE_G3_DISPLAY_NAME({1},{2});
        UNIFORM_ITEM_INFO;
    }};
"""

for uniformArr in uniformMap:
    uniformClass = uniformArr[0]
    uniformText = uniformArr[1]

    isG3 = "Crye G3" in uniformText
    isPolo = isG3 and "Button Up" in uniformText
    isTShirt = "T-Shirt" in uniformText or "Tee" in uniformText
    isBDU = "Battle Dress" in uniformText
    isCSAT = "CSAT" in uniformText
    isRolledSleeve = "Rolled" in uniformText
    hasOffColor = "+" in uniformText
    if hasOffColor:
        uniformStr = uniformStrDual
        offcolor = re.findall("\+ (.*?)[ |\)]",uniformText)[0]
        camocolor = re.findall(" \((.*?) \+",uniformText)[0]
        match camocolor.casefold():
            case "multicam":
                camocolor = "MC"
            case "oga grey":
                camocolor = "Grey"
            case "oga od":
                camocolor = "OD"
            case "oga":
                camocolor = "Tan"
            case "oga black":
                camocolor = "Black"
            case "multicamwoodland":
                camocolor = "MC-WDL"
            case "alpine":
                camocolor = "MC-ALP"
            case "arid":
                camocolor = "MC-ARD"
            case "mc black":
                camocolor = "MC-BLK"
            case "multicamtropic":
                camocolor = "MC-TRP"
            case "desert tiger stripe":
                camocolor = "DTS"
            case "projecthonor":
                camocolor = "PH"
        topcolor = camocolor
        bottomcolor = camocolor
        if "Pants" in uniformText:
            bottomcolor = offcolor
        else:
            topcolor = offcolor
    else:
        uniformStr = uniformStrSingle
        camocolor = re.findall(" \((.*?)\)",uniformText)[0]
        match camocolor.casefold():
            case "multicam":
                camocolor = "MC"
            case "oga grey":
                camocolor = "Grey"
            case "oga od":
                camocolor = "OD"
            case "oga":
                camocolor = "Tan"
            case "oga black":
                camocolor = "Black"
            case "multicamwoodland":
                camocolor = "MC-WDL"
            case "alpine":
                camocolor = "MC-ALP"
            case "arid":
                camocolor = "MC-ARD"
            case "mc black":
                camocolor = "MC-BLK"
            case "multicamtropic":
                camocolor = "MC-TRP"
            case "desert tiger stripe":
                camocolor = "DTS"
            case "projecthonor":
                camocolor = "PH"
    if isRolledSleeve:
        uniformStr = re.sub("CRYE_G3_","CRYE_G3_ROLLED_",uniformStr)
    if isPolo:
        uniformStr = re.sub("CRYE_G3_","CRYE_G3_POLO_",uniformStr)
    if isTShirt:
        uniformStr = re.sub("CRYE_G3_","CRYE_G3_TEE_",uniformStr)
    if not isG3:
        if isBDU:
            uniformStr = re.sub("CRYE_G3_","BDU_",uniformStr)
        elif isCSAT:
            uniformStr = re.sub("CRYE_G3_","CSAT_",uniformStr)
        else:
            uniformStr = re.sub("CRYE_G3_","ACU_",uniformStr)
    if hasOffColor:
        cfgUniforms.write(uniformStr.format(uniformClass,topcolor,bottomcolor))
    else:
        cfgUniforms.write(uniformStr.format(uniformClass,camocolor))

vestMap = [ #this was manually treated. sorry not sorry, too much of a pain to script it all
    ['VSM_CarrierRig_Breacher_OCP', '[VSM] LBX Armatus II (OCP/Breacher)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Gunner_OCP', '[VSM] LBX Armatus II (OCP/Gunner)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Operator_OCP', '[VSM] LBX Armatus II (OCP/Operator)', 'Vest_Camo_Base'],
    ['VSM_FAPC_Breacher_OCP', '[VSM] Flyye FAPC (OCP/Breacher)', 'ItemCore'],
    ['VSM_FAPC_MG_OCP', '[VSM] Flyye FAPC (OCP/Gunner)', 'ItemCore'],
    ['VSM_FAPC_Operator_OCP', '[VSM] Flyye FAPC (OCP/Operator)', 'ItemCore'],
    ['VSM_LBT6094_breacher_OCP', '[VSM] LBT 6094A (OCP/Breacher)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_MG_OCP', '[VSM] LBT 6094A (OCP/Gunner)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_operator_OCP', '[VSM] LBT 6094A (OCP/Operator)', 'Vest_Camo_Base'],
    ['VSM_RAV_Breacher_OCP', '[VSM] MSA RAV (OCP/Breacher)', 'Vest_Camo_Base'],
    ['VSM_RAV_MG_OCP', '[VSM] MSA RAV (OCP/Gunner)', 'Vest_Camo_Base'],
    ['VSM_RAV_operator_OCP', '[VSM] MSA RAV (OCP/Operator)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Breacher_multicamTropic', '[VSM] LBX Armatus II (MC-TRP/Breacher)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Gunner_multicamTropic', '[VSM] LBX Armatus II (MC-TRP/Gunner)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Operator_multicamTropic', '[VSM] LBX Armatus II (MC-TRP/Operator)', 'Vest_Camo_Base'],
    ['VSM_FAPC_Breacher_MulticamTropic', '[VSM] Flyye FAPC (MC-TRP/Breacher)', 'ItemCore'],
    ['VSM_FAPC_MG_MulticamTropic', '[VSM] Flyye FAPC (MC-TRP/Gunner)', 'ItemCore'],
    ['VSM_FAPC_Operator_MulticamTropic', '[VSM] Flyye FAPC (MC-TRP/Operator)', 'ItemCore'],
    ['VSM_LBT6094_breacher_multicamTropic', '[VSM] LBT 6094A (MC-TRP/Breacher)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_MG_multicamTropic', '[VSM] LBT 6094A (MC-TRP/Gunner)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_operator_multicamTropic', '[VSM] LBT 6094A (MC-TRP/Operator)', 'Vest_Camo_Base'],
    ['VSM_RAV_Breacher_MulticamTropic', '[VSM] MSA RAV (MC-TRP/Breacher)', 'Vest_Camo_Base'],
    ['VSM_RAV_MG_MulticamTropic', '[VSM] MSA RAV (MC-TRP/Gunner)', 'Vest_Camo_Base'],
    ['VSM_RAV_operator_MulticamTropic', '[VSM] MSA RAV (MC-TRP/Operator)', 'Vest_Camo_Base'],
    ['VSM_LBT1961_Black', '[VSM] LBT 1961a (Black)', 'Vest_Camo_Base'],
    ['VSM_MBSS_Green', '[VSM] MBSS (Green)', 'Vest_Camo_Base'],
    ['VSM_MBSS_PACA', '[VSM] MBSS + PACA (Green)', 'Vest_Camo_Base'],
    ['VSM_OGA_OD_Vest_1', '[VSM] Carrier Rig (OD)', 'Vest_Camo_Base'],
    ['VSM_OGA_OD_Vest_2', '[VSM] Carrier Lite (OD)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Breacher_OGA_OD', '[VSM] LBX Armatus II (OD/Breacher)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Gunner_OGA_OD', '[VSM] LBX Armatus II (OD/Gunner)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Operator_OGA_OD', '[VSM] LBX Armatus II (OD/Operator)', 'Vest_Camo_Base'],
    ['VSM_FAPC_Breacher_OGA_OD', '[VSM] Flyye FAPC (OD/Breacher)', 'ItemCore'],
    ['VSM_FAPC_MG_OGA_OD', '[VSM] Flyye FAPC (OD/Gunner)', 'ItemCore'],
    ['VSM_FAPC_Operator_OGA_OD', '[VSM] Flyye FAPC (OD/Operator)', 'ItemCore'],
    ['VSM_LBT1961_OGA_OD', '[VSM] LBT 1961a (OD)', 'VSM_LBT1961_Black'],
    ['VSM_LBT6094_breacher_OGA_OD', '[VSM] LBT 6094A (OD/Breacher)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_MG_OGA_OD', '[VSM] LBT 6094A (OD/Gunner)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_operator_OGA_OD', '[VSM] LBT 6094A (OD/Operator)', 'Vest_Camo_Base'],
    ['VSM_RAV_Breacher_OGA_OD', '[VSM] MSA RAV (OD/Breacher)', 'Vest_Camo_Base'],
    ['VSM_RAV_MG_OGA_OD', '[VSM] MSA RAV (OD/Gunner)', 'Vest_Camo_Base'],
    ['VSM_RAV_operator_OGA_OD', '[VSM] MSA RAV (OD/Operator)', 'Vest_Camo_Base'],
    ['VSM_OGA_OD_Vest_3', '[VSM] Carrier Lite Special (OD)', 'Vest_Camo_Base'], #these NEED to be first for inheritance
    ['Alpine_MBSS_Green', '[VSM] MBSS (MC-ALP)', 'VSM_MBSS_Green'],
    ['Alpine_MBSS_PACA', '[VSM] MBSS + PACA (MC-ALP)', 'VSM_MBSS_PACA'],
    ['AOR2_MBSS_Green', '[VSM] MBSS (AOR2)', 'VSM_MBSS_Green'],
    ['AOR2_MBSS_PACA', '[VSM] MBSS + PACA (AOR2)', 'VSM_MBSS_PACA'],
    ['ARD_MBSS_Green', '[VSM] MBSS (MC-ARD)', 'VSM_MBSS_Green'],
    ['ARD_MBSS_PACA', '[VSM] MBSS + PACA (MC-ARD)', 'VSM_MBSS_PACA'],
    ['BLK_MBSS_Green', '[VSM] MBSS (MC-BLK)', 'VSM_MBSS_Green'],
    ['BLK_MBSS_PACA', '[VSM] MBSS + PACA (MC-BLK)', 'VSM_MBSS_PACA'],
    ['CarrierRig_Breacher_Alpine', '[VSM] LBX Armatus II (MC-ALP/Breacher)', 'VSM_CarrierRig_Breacher_multicamTropic'],
    ['CarrierRig_Breacher_AOR2', '[VSM] LBX Armatus II (AOR2/Breacher)', 'VSM_CarrierRig_Breacher_multicamTropic'],
    ['CarrierRig_Breacher_Arid', '[VSM] LBX Armatus II (MC-ARD/Breacher)', 'VSM_CarrierRig_Breacher_multicamTropic'],
    ['CarrierRig_Breacher_DTS', '[VSM] LBX Armatus II (DTS/Breacher)', 'VSM_CarrierRig_Breacher_multicamTropic'],
    ['CarrierRig_Gunner_Alpine', '[VSM] LBX Armatus II (MC-ALP/Gunner)', 'VSM_CarrierRig_Gunner_multicamTropic'],
    ['CarrierRig_Gunner_AOR2', '[VSM] LBX Armatus II (AOR2/Gunner)', 'VSM_CarrierRig_Gunner_multicamTropic'],
    ['CarrierRig_Gunner_Arid', '[VSM] LBX Armatus II (MC-ARD/Gunner)', 'VSM_CarrierRig_Gunner_multicamTropic'],
    ['CarrierRig_Gunner_DTS', '[VSM] LBX Armatus II (DTS/Gunner)', 'VSM_CarrierRig_Gunner_multicamTropic'],
    ['CarrierRig_Operator_Alpine', '[VSM] LBX Armatus II (MC-ALP/Operator)', 'VSM_CarrierRig_Operator_multicamTropic'],
    ['CarrierRig_Operator_AOR2', '[VSM] LBX Armatus II (AOR2/Operator)', 'VSM_CarrierRig_Operator_multicamTropic'],
    ['CarrierRig_Operator_Arid', '[VSM] LBX Armatus II (MC-ARD/Operator)', 'VSM_CarrierRig_Operator_multicamTropic'],
    ['CarrierRig_Operator_DTS', '[VSM] LBX Armatus II (DTS/Operator)', 'VSM_CarrierRig_Operator_multicamTropic'],
    ['dr_Alpinefacp_br', '[VSM] Flyye FAPC (MC-ALP/Breacher)', 'VSM_FAPC_Breacher_OCP'],
    ['dr_Alpinefacp_mg', '[VSM] Flyye FAPC (MC-ALP/Gunner)', 'VSM_FAPC_MG_OCP'],
    ['dr_Alpinefacp_op', '[VSM] Flyye FAPC (MC-ALP/Operator)', 'VSM_FAPC_Operator_OCP'],
    ['dr_Alpinelbt_br', '[VSM] LBT 6094A (MC-ALP/Breacher)', 'VSM_LBT6094_breacher_OCP'],
    ['dr_Alpinelbt_mg', '[VSM] LBT 6094A (MC-ALP/Gunner)', 'VSM_LBT6094_MG_OCP'],
    ['dr_Alpinelbt_op', '[VSM] LBT 6094A (MC-ALP/Operator)', 'VSM_LBT6094_operator_OCP'],
    ['dr_Alpinepar_br', '[VSM] MSA RAV (MC-ALP/Breacher)', 'VSM_RAV_Breacher_OCP'],
    ['dr_Alpinepar_mg', '[VSM] MSA RAV (MC-ALP/Gunner)', 'VSM_RAV_MG_OCP'],
    ['dr_Alpinepar_op', '[VSM] MSA RAV (MC-ALP/Operator)', 'VSM_RAV_operator_OCP'],
    ['dr_AOR2facp_br', '[VSM] Flyye FAPC (AOR2/Breacher)', 'VSM_FAPC_Breacher_OCP'],
    ['dr_AOR2facp_mg', '[VSM] Flyye FAPC (AOR2/Gunner)', 'VSM_FAPC_MG_OCP'],
    ['dr_AOR2facp_op', '[VSM] Flyye FAPC (AOR2/Operator)', 'VSM_FAPC_Operator_OCP'],
    ['dr_AOR2lbt_br', '[VSM] LBT 6094A (AOR2/Breacher)', 'VSM_LBT6094_breacher_OCP'],
    ['dr_AOR2lbt_mg', '[VSM] LBT 6094A (AOR2/Gunner)', 'VSM_LBT6094_MG_OCP'],
    ['dr_AOR2lbt_op', '[VSM] LBT 6094A (AOR2/Operator)', 'VSM_LBT6094_operator_OCP'],
    ['dr_AOR2par_br', '[VSM] MSA RAV (AOR2/Breacher)', 'VSM_RAV_Breacher_OCP'],
    ['dr_AOR2par_mg', '[VSM] MSA RAV (AOR2/Gunner)', 'VSM_RAV_MG_OCP'],
    ['dr_AOR2par_op', '[VSM] MSA RAV (AOR2/Operator)', 'VSM_RAV_operator_OCP'],
    ['dr_ARDfacp_br', '[VSM] Flyye FAPC (MC-ARD/Breacher)', 'VSM_FAPC_Breacher_OCP'],
    ['dr_ARDfacp_mg', '[VSM] Flyye FAPC (MC-ARD/Gunner)', 'VSM_FAPC_MG_OCP'],
    ['dr_ARDfacp_op', '[VSM] Flyye FAPC (MC-ARD/Operator)', 'VSM_FAPC_Operator_OCP'],
    ['dr_ARDlbt_br', '[VSM] LBT 6094A (MC-ARD/Breacher)', 'VSM_LBT6094_breacher_OCP'],
    ['dr_ARDlbt_mg', '[VSM] LBT 6094A (MC-ARD/Gunner)', 'VSM_LBT6094_MG_OCP'],
    ['dr_ARDlbt_op', '[VSM] LBT 6094A (MC-ARD/Operator)', 'VSM_LBT6094_operator_OCP'],
    ['dr_ARDpar_br', '[VSM] MSA RAV (MC-ARD/Breacher)', 'VSM_RAV_Breacher_OCP'],
    ['dr_ARDpar_mg', '[VSM] MSA RAV (MC-ARD/Gunner)', 'VSM_RAV_MG_OCP'],
    ['dr_ARDpar_op', '[VSM] MSA RAV (MC-ARD/Operator)', 'VSM_RAV_operator_OCP'],
    ['dr_BLKfacp_br', '[VSM] Flyye FAPC (MC-BLK/Breacher)', 'VSM_FAPC_Breacher_OCP'],
    ['dr_BLKfacp_mg', '[VSM] Flyye FAPC (MC-BLK/Gunner)', 'VSM_FAPC_MG_OCP'],
    ['dr_BLKfacp_op', '[VSM] Flyye FAPC (MC-BLK/Operator)', 'VSM_FAPC_Operator_OCP'],
    ['dr_BLKlbt_br', '[VSM] LBT 6094A (MC-BLK/Breacher)', 'VSM_LBT6094_breacher_OCP'],
    ['dr_BLKlbt_mg', '[VSM] LBT 6094A (MC-BLK/Gunner)', 'VSM_LBT6094_MG_OCP'],
    ['dr_BLKlbt_op', '[VSM] LBT 6094A (MC-BLK/Operator)', 'VSM_LBT6094_operator_OCP'],
    ['dr_BLKpar_br', '[VSM] MSA RAV (MC-BLK/Breacher)', 'VSM_RAV_Breacher_OCP'],
    ['dr_BLKpar_mg', '[VSM] MSA RAV (MC-BLK/Gunner)', 'VSM_RAV_MG_OCP'],
    ['dr_BLKpar_op', '[VSM] MSA RAV (MC-BLK/Operator)', 'VSM_RAV_operator_OCP'],
    ['DTS_MBSS_Green', '[VSM] MBSS (DTS)', 'VSM_MBSS_Green'],
    ['DTS_MBSS_PACA', '[VSM] MBSS + PACA (DTS)', 'VSM_MBSS_PACA'],
    ['DTSfacp_br', '[VSM] Flyye FAPC (DTS/Breacher)', 'VSM_FAPC_Breacher_OCP'],
    ['DTSfacp_mg', '[VSM] Flyye FAPC (DTS/Gunner)', 'VSM_FAPC_MG_OCP'],
    ['DTSfacp_op', '[VSM] Flyye FAPC (DTS/Operator)', 'VSM_FAPC_Operator_OCP'],
    ['DTSlbt_br', '[VSM] LBT 6094A (DTS/Breacher)', 'VSM_LBT6094_breacher_OCP'],
    ['DTSlbt_mg', '[VSM] LBT 6094A (DTS/Gunner)', 'VSM_LBT6094_MG_OCP'],
    ['DTSlbt_op', '[VSM] LBT 6094A (DTS/Operator)', 'VSM_LBT6094_operator_OCP'],
    ['DTSpar_br', '[VSM] MSA RAV (DTS/Breacher)', 'VSM_RAV_Breacher_OCP'],
    ['DTSpar_mg', '[VSM] MSA RAV (DTS/Gunner)', 'VSM_RAV_MG_OCP'],
    ['DTSpar_op', '[VSM] MSA RAV (DTS/Operator)', 'VSM_RAV_operator_OCP'],
    ['LBT1961_Alpine', '[VSM] LBT 1961a (MC-ALP)', 'VSM_LBT1961_Black'],
    ['LBT1961_AOR2', '[VSM] LBT 1961a (AOR2)', 'VSM_LBT1961_Black'],
    ['LBT1961_Arid', '[VSM] LBT 1961a (MC-ARD)', 'VSM_LBT1961_Black'],
    ['LBT1961_DTS', '[VSM] LBT 1961a (DTS)', 'VSM_LBT1961_Black'],
    ['MLO_Black_Vest_1', '[VSM] Carrier Rig (Black)', 'VSM_OGA_OD_Vest_1'],
    ['MLO_Black_Vest_2', '[VSM] Carrier Lite (Black)', 'VSM_OGA_OD_Vest_2'],
    ['MLO_Black_Vest_3', '[VSM] Carrier Lite Special (Black)', 'VSM_OGA_OD_Vest_3'],
    ['MLO_CarrierRig_Breacher_Black', '[VSM] LBX Armatus II (Black/Breacher)', 'VSM_CarrierRig_Breacher_OGA_OD'],
    ['MLO_CarrierRig_Gunner_Black', '[VSM] LBX Armatus II (Black/Gunner)', 'VSM_CarrierRig_Gunner_OGA_OD'],
    ['MLO_CarrierRig_Operator_Black', '[VSM] LBX Armatus II (Black/Operator)', 'VSM_CarrierRig_Operator_OGA_OD'],
    ['MLO_FAPC_Breacher_Black', '[VSM] Flyye FAPC (Black/Breacher)', 'VSM_FAPC_Breacher_OGA_OD'],
    ['MLO_FAPC_MG_Black', '[VSM] Flyye FAPC (Black/Gunner)', 'VSM_FAPC_MG_OGA_OD'],
    ['MLO_FAPC_Operator_Black', '[VSM] Flyye FAPC (Black/Operator)', 'VSM_FAPC_Operator_OGA_OD'],
    ['MLO_LBT6094_breacher_Black', '[VSM] LBT 6094A (Black/Breacher)', 'VSM_LBT6094_breacher_OGA_OD'],
    ['MLO_LBT6094_MG_Black', '[VSM] LBT 6094A (Black/Gunner)', 'VSM_LBT6094_MG_OGA_OD'],
    ['MLO_LBT6094_operator_Black', '[VSM] LBT 6094A (Black/Operator)', 'VSM_LBT6094_operator_OGA_OD'],
    ['MLO_RAV_Breacher_Black', '[VSM] MSA RAV (Black/Breacher)', 'VSM_RAV_Breacher_OGA_OD'],
    ['MLO_RAV_MG_Black', '[VSM] MSA RAV (Black/Gunner)', 'VSM_RAV_MG_OGA_OD'],
    ['MLO_RAV_Operator_Black', '[VSM] MSA RAV (Black/Operator)', 'VSM_RAV_operator_OGA_OD'],
    ['VSM_CarrierRig_Breacher_AOR1', '[VSM] LBX Armatus II (AOR1/Breacher)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Breacher_M81', '[VSM] LBX Armatus II (M81/Breacher)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Breacher_Multicam', '[VSM] LBX Armatus II (MC/Breacher)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Breacher_OGA', '[VSM] LBX Armatus II (Tan/Breacher)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Breacher_ProjectHonor', '[VSM] LBX Armatus II (PH/Breacher)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Gunner_AOR1', '[VSM] LBX Armatus II (AOR1/Gunner)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Gunner_M81', '[VSM] LBX Armatus II (M81/Gunner)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Gunner_Multicam', '[VSM] LBX Armatus II (MC/Gunner)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Gunner_OGA', '[VSM] LBX Armatus II (Tan/Gunner)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Gunner_ProjectHonor', '[VSM] LBX Armatus II (PH/Gunner)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Operator_AOR1', '[VSM] LBX Armatus II (AOR1/Operator)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Operator_M81', '[VSM] LBX Armatus II (M81/Operator)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Operator_Multicam', '[VSM] LBX Armatus II (MC/Operator)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Operator_OGA', '[VSM] LBX Armatus II (Tan/Operator)', 'Vest_Camo_Base'],
    ['VSM_CarrierRig_Operator_ProjectHonor', '[VSM] LBX Armatus II (PH/Operator)', 'Vest_Camo_Base'],
    ['VSM_FAPC_Breacher_AOR1', '[VSM] Flyye FAPC (AOR1/Breacher)', 'ItemCore'],
    ['VSM_FAPC_Breacher_M81', '[VSM] Flyye FAPC (M81/Breacher)', 'ItemCore'],
    ['VSM_FAPC_Breacher_Multicam', '[VSM] Flyye FAPC (MC/Breacher)', 'ItemCore'],
    ['VSM_FAPC_Breacher_OGA', '[VSM] Flyye FAPC (Tan/Breacher)', 'ItemCore'],
    ['VSM_FAPC_Breacher_ProjectHonor', '[VSM] Flyye FAPC (PH/Breacher)', 'ItemCore'],
    ['VSM_FAPC_MG_AOR1', '[VSM] Flyye FAPC (AOR1/Gunner)', 'ItemCore'],
    ['VSM_FAPC_MG_M81', '[VSM] Flyye FAPC (M81/Gunner)', 'ItemCore'],
    ['VSM_FAPC_MG_Multicam', '[VSM] Flyye FAPC (MC/Gunner)', 'ItemCore'],
    ['VSM_FAPC_MG_OGA', '[VSM] Flyye FAPC (Tan/Gunner)', 'ItemCore'],
    ['VSM_FAPC_MG_ProjectHonor', '[VSM] Flyye FAPC (PH/Gunner)', 'ItemCore'],
    ['VSM_FAPC_Operator_AOR1', '[VSM] Flyye FAPC (AOR1/Operator)', 'ItemCore'],
    ['VSM_FAPC_Operator_M81', '[VSM] Flyye FAPC (M81/Operator)', 'ItemCore'],
    ['VSM_FAPC_Operator_Multicam', '[VSM] Flyye FAPC (MC/Operator)', 'ItemCore'],
    ['VSM_FAPC_Operator_OGA', '[VSM] Flyye FAPC (Tan/Operator)', 'ItemCore'],
    ['VSM_FAPC_Operator_ProjectHonor', '[VSM] Flyye FAPC (PH/Operator)', 'ItemCore'],
    ['VSM_LBT1961_CB', '[VSM] LBT 1961a (CB)', 'VSM_LBT1961_Black'],
    ['VSM_LBT1961_GRN', '[VSM] LBT 1961a (Green)', 'VSM_LBT1961_Black'],
    ['VSM_LBT6094_breacher_AOR1', '[VSM] LBT 6094A (AOR1/Breacher)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_breacher_M81', '[VSM] LBT 6094A (M81/Breacher)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_breacher_Multicam', '[VSM] LBT 6094A (MC/Breacher)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_breacher_OGA', '[VSM] LBT 6094A (Tan/Breacher)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_breacher_ProjectHonor', '[VSM] LBT 6094A (PH/Breacher)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_MG_AOR1', '[VSM] LBT 6094A (AOR1/Gunner)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_MG_M81', '[VSM] LBT 6094A (M81/Gunner)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_MG_Multicam', '[VSM] LBT 6094A (MC/Gunner)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_MG_OGA', '[VSM] LBT 6094A (Tan/Gunner)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_MG_ProjectHonor', '[VSM] LBT 6094A (PH/Gunner)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_operator_AOR1', '[VSM] LBT 6094A (AOR1/Operator)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_operator_M81', '[VSM] LBT 6094A (M81/Operator)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_operator_Multicam', '[VSM] LBT 6094A (MC/Operator)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_operator_OGA', '[VSM] LBT 6094A (Tan/Operator)', 'Vest_Camo_Base'],
    ['VSM_LBT6094_operator_ProjectHonor', '[VSM] LBT 6094A (PH/Operator)', 'Vest_Camo_Base'],
    ['VSM_MBSS_BLK', '[VSM] MBSS (Black)', 'VSM_MBSS_Green'],
    ['VSM_MBSS_CB', '[VSM] MBSS (CB)', 'VSM_MBSS_Green'],
    ['VSM_MBSS_PACA_BLK', '[VSM] MBSS + PACA (Black)', 'VSM_MBSS_PACA'],
    ['VSM_MBSS_PACA_CB', '[VSM] MBSS + PACA (CB)', 'VSM_MBSS_PACA'],
    ['VSM_MBSS_PACA_TAN', '[VSM] MBSS + PACA (Tan)', 'VSM_MBSS_PACA'],
    ['VSM_MBSS_PACA_WTF', '[VSM] MBSS + PACA (Pink)', 'VSM_MBSS_PACA'],
    ['VSM_MBSS_TAN', '[VSM] MBSS (Tan)', 'VSM_MBSS_Green'],
    ['VSM_MBSS_WTF', '[VSM] MBSS (Pink)', 'VSM_MBSS_Green'],
    ['VSM_OGA_IOTV_1', '[VSM] IOTV Lite (Tan)', 'Vest_Camo_Base'],
    ['VSM_OGA_IOTV_2', '[VSM] IOTV (Tan)', 'Vest_Camo_Base'],
    ['VSM_OGA_OD_IOTV_1', '[VSM] IOTV Lite (OD)', 'Vest_Camo_Base'],
    ['VSM_OGA_OD_IOTV_2', '[VSM] IOTV (OD)', 'Vest_Camo_Base'],
    ['VSM_OGA_Vest_1', '[VSM] Carrier Rig (Tan)', 'Vest_Camo_Base'],
    ['VSM_OGA_Vest_2', '[VSM] Carrier Lite (Tan)', 'Vest_Camo_Base'],
    ['VSM_OGA_Vest_3', '[VSM] Carrier Lite Special (Tan)', 'Vest_Camo_Base'],
    ['VSM_RAV_Breacher_AOR1', '[VSM] MSA RAV (AOR1/Breacher)', 'Vest_Camo_Base'],
    ['VSM_RAV_Breacher_M81', '[VSM] MSA RAV (M81/Breacher)', 'Vest_Camo_Base'],
    ['VSM_RAV_Breacher_Multicam', '[VSM] MSA RAV (MC/Breacher)', 'Vest_Camo_Base'],
    ['VSM_RAV_Breacher_OGA', '[VSM] MSA RAV (Tan/Breacher)', 'Vest_Camo_Base'],
    ['VSM_RAV_Breacher_ProjectHonor', '[VSM] MSA RAV (PH/Breacher)', 'Vest_Camo_Base'],
    ['VSM_RAV_MG_AOR1', '[VSM] MSA RAV (AOR1/Gunner)', 'Vest_Camo_Base'],
    ['VSM_RAV_MG_M81', '[VSM] MSA RAV (M81/Gunner)', 'Vest_Camo_Base'],
    ['VSM_RAV_MG_Multicam', '[VSM] MSA RAV (MC/Gunner)', 'Vest_Camo_Base'],
    ['VSM_RAV_MG_OGA', '[VSM] MSA RAV (Tan/Gunner)', 'Vest_Camo_Base'],
    ['VSM_RAV_MG_ProjectHonor', '[VSM] MSA RAV (PH/Gunner)', 'Vest_Camo_Base'],
    ['VSM_RAV_operator_AOR1', '[VSM] MSA RAV (AOR1/Operator)', 'Vest_Camo_Base'],
    ['VSM_RAV_operator_M81', '[VSM] MSA RAV (M81/Operator)', 'Vest_Camo_Base'],
    ['VSM_RAV_operator_Multicam', '[VSM] MSA RAV (MC/Operator)', 'Vest_Camo_Base'],
    ['VSM_RAV_operator_OGA', '[VSM] MSA RAV (Tan/Operator)', 'Vest_Camo_Base'],
    ['VSM_RAV_operator_ProjectHonor', '[VSM] MSA RAV (PH/Operator)', 'Vest_Camo_Base']
]

vestStrVariant = """
    class {0}: {1} {{
        VEST_DISPLAY_NAME({2},{3},{4});
        VEST_{5}_ITEM_INFO;
    }};
"""

vestStrNoVariant = """
    class {0}: {1} {{
        VEST_DISPLAY_NAME_NOVARIANT({2},{3});
        VEST_OPERATOR_ITEM_INFO;
    }};
"""

for vestArr in vestMap:
    vestClass = vestArr[0]
    vestText = vestArr[1]
    vestParent = vestArr[2]
    itemInfoParent = "VestItem"
    print(vestArr)
    vestName = re.findall("\[VSM\] (.*?) \(",vestText)[0]
    vestCamo = re.findall("\((.*?)[/\)]",vestText)[0]
    vestVariant = re.findall("/(.*?)\)",vestText)
    if (len(vestVariant)) != 0:
        vestVariant = vestVariant[0]
        vestCfg = vestStrVariant.format(vestClass,vestParent,vestName,vestCamo,vestVariant,vestVariant.upper())
        if not vestParent in ["Vest_Camo_Base", "ItemCore"]:
            vestCfg = re.sub(";\n.*;",";",vestCfg)
        cfgVests.write(vestCfg)
    else:
        vestCfg = vestStrNoVariant.format(vestClass,vestParent,vestName,vestCamo)
        if not vestParent in ["Vest_Camo_Base", "ItemCore"]:
            vestCfg = re.sub(";\n.*;",";",vestCfg)
        cfgVests.write(vestCfg)