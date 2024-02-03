#include "\a3\ui_f\hpp\defineResincl.inc"

class RscControlsGroupNoScrollbars;
class RscText;
class ctrlStaticBackground;

class ACEGVAR(arsenal,display) {
    class controls {
        class requiredGearBox: RscControlsGroupNoScrollbars {
            idc = IDC_requiredGearBox;
            fade = 0;
            x = QUOTE(safezoneX + safezoneW - (94 + 48) * GRID_W);
            y = QUOTE(safezoneY + 1.8 * GRID_H);
            w = QUOTE(47 * GRID_W);
            h = QUOTE(45 * GRID_H);
            class controls {
                class requiredGearStaticBackground1: ctrlStaticBackground {
                    idc = -1;
                    x = QUOTE(0);
                    y = QUOTE(0);
                    w = QUOTE(47 * GRID_W);
                    h = QUOTE(200 * GRID_H);
                    colorBackground[]={0.1,0.1,0.1,0.5};
                };
                class requiredGearStaticBackground2: ctrlStaticBackground {
                    idc = -1;
                    x = QUOTE(0);
                    y = QUOTE(0);
                    w = QUOTE(47 * GRID_W);
                    h = QUOTE(5 * GRID_H);
                    colorBackground[]={0.1,0.1,0.1,0.8};
                };
                class requiredGearText: RscText {
                    idc = IDC_requiredGearText;
                    type = CT_STRUCTURED_TEXT;
                    style = ST_LEFT;
                    fade = 0;
                    x = QUOTE(0 * GRID_W);
                    y = QUOTE(5 * GRID_H);
                    w = QUOTE(45 * GRID_W);
                    h = QUOTE(40 * GRID_H);
                    colorBackground[]={0,0,0,0};
                    colorText[]={0.7,0.7,0.7,1};
                    sizeEx = QUOTE(4 * GRID_H);
                    text = "";
                    size = QUOTE(4 * GRID_H);
                };
            };
        };
        class requiredGearTitle: RscText {
            idc = IDC_requiredGearTitle;
            style = ST_CENTER;
            x = QUOTE(safezoneX + safezoneW - (94 + 47) * GRID_W);
            y = QUOTE(safezoneY + 1.75 * GRID_H);
            w = QUOTE(47 * GRID_W);
            h = QUOTE(5 * GRID_H);
            colorBackground[]={0,0,0,0};
            shadow=2;
            sizeEx = QUOTE(5 * GRID_H);
            text = "Missing Basic Gear";
        };
        class specialGearBox: RscControlsGroupNoScrollbars {
            idc = IDC_specialGearBox;
            fade = 0;
            x = QUOTE(safezoneX + safezoneW - (94 + 48) * GRID_W);
            y = QUOTE(safezoneY + 1.75 * GRID_H);
            w = QUOTE(47 * GRID_W);
            h = QUOTE(45 * GRID_H);
            class controls {
                class specialGearStaticBackground1: ctrlStaticBackground {
                    idc = -1;
                    x = QUOTE(0);
                    y = QUOTE(0);
                    w = QUOTE(47 * GRID_W);
                    h = QUOTE(200 * GRID_H);
                    colorBackground[]={0.1,0.1,0.1,0.5};
                };
                class specialGearStaticBackground2: ctrlStaticBackground {
                    idc = -1;
                    x = QUOTE(0);
                    y = QUOTE(0);
                    w = QUOTE(47 * GRID_W);
                    h = QUOTE(5 * GRID_H);
                    colorBackground[]={0.1,0.1,0.1,0.8};
                };
                class specialGearText: RscText {
                    idc = IDC_specialGearText;
                    type = CT_STRUCTURED_TEXT;
                    style = ST_LEFT;
                    fade = 0;
                    x = QUOTE(0 * GRID_W);
                    y = QUOTE(5 * GRID_H);
                    w = QUOTE(45 * GRID_W);
                    h = QUOTE(40 * GRID_H);
                    colorBackground[]={0,0,0,0};
                    colorText[]={0.7,0.7,0.7,1};
                    sizeEx = QUOTE(4 * GRID_H);
                    text = "";
                    size = QUOTE(4 * GRID_H);
                };
            };
        };
        class specialGearTitle: RscText {
            idc = IDC_specialGearTitle;
            style = ST_CENTER;
            x = QUOTE(safezoneX + safezoneW - (94 + 47) * GRID_W);
            y = QUOTE(safezoneY + 1.8 * GRID_H);
            w = QUOTE(47 * GRID_W);
            h = QUOTE(5 * GRID_H);
            colorBackground[]={0,0,0,0};
            shadow=2;
            sizeEx = QUOTE(5 * GRID_H);
            text = "Missing Special Gear";
        };
    };
};
