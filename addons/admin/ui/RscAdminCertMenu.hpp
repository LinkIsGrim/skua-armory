#include "\a3\ui_f\hpp\defineCommonGrids.inc"
#include "\a3\ui_f\hpp\defineResincl.inc"
#include "\a3\ui_f\hpp\defineResinclDesign.inc"
#include "defines.hpp"

class RscText;
class RscTitle;
class RscListBox;
class RscButtonMenu;

// POS_X/POS_Y are anchored on the 40-column, 25-row GUI_GRID_CENTER zone.
// Stay inside [0, 40] horizontally and [0, 25] vertically or controls fall
// off-screen.
#define ACM_X       1
#define ACM_Y       2
#define ACM_W       38
#define ACM_H       20

#define ACM_TITLE_H 1
#define ACM_BTN_H   1.1
#define ACM_BTN_W   6

// Column geometry (relative to ACM_X). Three equal-ish panes separated by
// 0.2 gutters. Sums to ACM_W exactly so the right edge lines up with MainBg.
#define ACM_COL_PLAYER_X    0
#define ACM_COL_PLAYER_W    12
#define ACM_COL_HELD_X      (ACM_COL_PLAYER_W + 0.2)
#define ACM_COL_HELD_W      12
#define ACM_COL_AVAIL_X     (ACM_COL_HELD_X + ACM_COL_HELD_W + 0.2)
#define ACM_COL_AVAIL_W     (ACM_W - ACM_COL_AVAIL_X)

// Y bands.
#define ACM_BODY_Y          (ACM_TITLE_H + 0.2)
#define ACM_LIST_HEADER_H   0.8
#define ACM_LIST_BODY_Y     (ACM_BODY_Y + ACM_LIST_HEADER_H)
#define ACM_LIST_H          (ACM_H - ACM_LIST_BODY_Y - ACM_BTN_H - 0.4)
#define ACM_BTN_ROW_Y       (ACM_H - ACM_BTN_H)

// Top-level config class — createDialog finds root-level classes from any
// loaded addon. ACE3's cargo/menu and similar dialogs use the same pattern.
class skua_admin_AdminCertMenu {
    idd = IDD_ADMIN_CERT_MENU;
    movingEnable = 0;
    enableSimulation = 1;

    onLoad = QUOTE(_this call FUNC(onCertMenuOpen));
    onUnload = QUOTE(_this call FUNC(onCertMenuClose));

    class ControlsBackground {
        class TitleBg: RscText {
            colorBackground[] = COLOR_BCG;
            x = QUOTE(POS_X(ACM_X));
            y = QUOTE(POS_Y(ACM_Y));
            w = QUOTE(POS_W(ACM_W));
            h = QUOTE(POS_H(ACM_TITLE_H));
        };
        class MainBg: RscText {
            colorBackground[] = {0,0,0,0.7};
            x = QUOTE(POS_X(ACM_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_BODY_Y));
            w = QUOTE(POS_W(ACM_W));
            h = QUOTE(POS_H(ACM_H - ACM_BODY_Y));
        };
    };

    class Controls {
        class Title: RscTitle {
            idc = IDC_ADMINCERT_TITLE;
            style = ST_LEFT;
            text = "Admin: Certification Management";
            x = QUOTE(POS_X(ACM_X));
            y = QUOTE(POS_Y(ACM_Y));
            w = QUOTE(POS_W(ACM_W));
            h = QUOTE(POS_H(ACM_TITLE_H));
        };

        class PlayerHeader: RscText {
            text = "Players";
            x = QUOTE(POS_X(ACM_X + ACM_COL_PLAYER_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_BODY_Y));
            w = QUOTE(POS_W(ACM_COL_PLAYER_W));
            h = QUOTE(POS_H(ACM_LIST_HEADER_H));
        };
        class PlayerList: RscListBox {
            idc = IDC_ADMINCERT_PLAYER_LIST;
            x = QUOTE(POS_X(ACM_X + ACM_COL_PLAYER_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_LIST_BODY_Y));
            w = QUOTE(POS_W(ACM_COL_PLAYER_W));
            h = QUOTE(POS_H(ACM_LIST_H));
        };

        class HeldHeader: PlayerHeader {
            idc = IDC_ADMINCERT_HELD_TITLE;
            text = "Held Certs";
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X));
            w = QUOTE(POS_W(ACM_COL_HELD_W));
        };
        class HeldList: PlayerList {
            idc = IDC_ADMINCERT_HELD_LIST;
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X));
            w = QUOTE(POS_W(ACM_COL_HELD_W));
        };

        class AvailHeader: PlayerHeader {
            idc = IDC_ADMINCERT_AVAILABLE_TITLE;
            text = "Available Certs";
            x = QUOTE(POS_X(ACM_X + ACM_COL_AVAIL_X));
            w = QUOTE(POS_W(ACM_COL_AVAIL_W));
        };
        class AvailList: PlayerList {
            idc = IDC_ADMINCERT_AVAILABLE_LIST;
            x = QUOTE(POS_X(ACM_X + ACM_COL_AVAIL_X));
            w = QUOTE(POS_W(ACM_COL_AVAIL_W));
        };

        class BtnRevoke: RscButtonMenu {
            idc = IDC_ADMINCERT_BTN_REVOKE;
            text = "Revoke";
            onButtonClick = QUOTE([false] call FUNC(onCertMenuRevokeClicked));
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_BTN_ROW_Y));
            w = QUOTE(POS_W(ACM_BTN_W));
            h = QUOTE(POS_H(ACM_BTN_H));
        };
        class BtnRevokeTemp: BtnRevoke {
            idc = IDC_ADMINCERT_BTN_REVOKE_TEMP;
            text = "Revoke (temp)";
            onButtonClick = QUOTE([true] call FUNC(onCertMenuRevokeClicked));
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X + ACM_BTN_W + 0.2));
        };
        class BtnGrant: BtnRevoke {
            idc = IDC_ADMINCERT_BTN_GRANT;
            text = "Grant";
            onButtonClick = QUOTE([false] call FUNC(onCertMenuGrantClicked));
            x = QUOTE(POS_X(ACM_X + ACM_COL_AVAIL_X));
        };
        class BtnGrantTemp: BtnRevoke {
            idc = IDC_ADMINCERT_BTN_GRANT_TEMP;
            text = "Grant (temp)";
            onButtonClick = QUOTE([true] call FUNC(onCertMenuGrantClicked));
            x = QUOTE(POS_X(ACM_X + ACM_COL_AVAIL_X + ACM_BTN_W + 0.2));
        };

        // Close + Commit sit in the Player column's button row. Close runs a
        // custom handler (not RscButtonMenuCancel's auto-close) so it can
        // prompt before discarding pending changes.
        class BtnClose: RscButtonMenu {
            idc = IDC_ADMINCERT_BTN_CLOSE;
            text = "Close";
            onButtonClick = QUOTE(call FUNC(onCertMenuCloseClicked));
            x = QUOTE(POS_X(ACM_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_BTN_ROW_Y));
            w = QUOTE(POS_W(ACM_BTN_W));
            h = QUOTE(POS_H(ACM_BTN_H));
        };
        class BtnCommit: BtnClose {
            idc = IDC_ADMINCERT_BTN_COMMIT;
            text = "Commit";
            onButtonClick = QUOTE(call FUNC(commitCertChanges));
            x = QUOTE(POS_X(ACM_X + ACM_BTN_W + 0.2));
        };
    };
};
