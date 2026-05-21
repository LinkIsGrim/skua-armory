#include "\a3\ui_f\hpp\defineCommonGrids.inc"
#include "\a3\ui_f\hpp\defineResincl.inc"
#include "\a3\ui_f\hpp\defineResinclDesign.inc"
#include "defines.hpp"

class RscText;
class RscTitle;
class RscListBox;
class RscButtonMenu;
class RscCheckBox;
class RscActivePicture;

// POS_X/POS_Y are anchored on the 40-column, 25-row GUI_GRID_CENTER zone.
// Stay inside [0, 40] horizontally and [0, 25] vertically or controls fall
// off-screen.
#define ACM_X       1
#define ACM_Y       2
#define ACM_W       38
#define ACM_H       20

#define ACM_TITLE_H 1
#define ACM_TAB_H   1.0
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

// Addons tab uses the same right-side region as Held + Avail combined.
// Top half: Missing Mods (spans both columns).
// Bottom half: Extra Mods (left sub-column) | Extra Addons (right sub-column).
#define ACM_ADDON_AREA_X    ACM_COL_HELD_X
#define ACM_ADDON_AREA_W    (ACM_W - ACM_ADDON_AREA_X)
#define ACM_ADDON_LEFT_W    (ACM_COL_HELD_W)
#define ACM_ADDON_RIGHT_W   (ACM_COL_AVAIL_W)

// Y bands. Tab row sits between title and the column headers.
#define ACM_TAB_Y           (ACM_TITLE_H + 0.1)
#define ACM_BODY_Y          (ACM_TAB_Y + ACM_TAB_H + 0.1)
#define ACM_LIST_HEADER_H   1.0
#define ACM_LIST_BODY_Y     (ACM_BODY_Y + ACM_LIST_HEADER_H)
#define ACM_LIST_H          (ACM_H - ACM_LIST_BODY_Y - ACM_BTN_H - 0.4)
#define ACM_BTN_ROW_Y       (ACM_H - ACM_BTN_H)

// Addon-area split: top half = Missing, bottom half = Extra Mods | Extra Addons.
#define ACM_ADDON_TOP_H     (ACM_LIST_H / 2 - 0.1)
#define ACM_ADDON_BOT_Y     (ACM_LIST_BODY_Y + ACM_ADDON_TOP_H + 0.2)
#define ACM_ADDON_BOT_HDR_Y (ACM_ADDON_BOT_Y - ACM_LIST_HEADER_H)
#define ACM_ADDON_BOT_H     (ACM_LIST_H - ACM_ADDON_TOP_H - 0.2)

// Header-row controls in the Player column.
//   [ ☐ ] Online only ........... [↻]
// Checkbox is a 1x1 square (RscCheckBox has no built-in text); the label is a
// separate RscText with onMouseButtonClick wired so clicking it toggles the
// box too. Refresh icon sits flush-right.
#define ACM_REFRESH_ICON_W  1
#define ACM_CHK_BOX_W       1
#define ACM_CHK_LABEL_W     (ACM_COL_PLAYER_W - ACM_CHK_BOX_W - ACM_REFRESH_ICON_W - 0.4)

// Tab bar takes the right-side body columns' horizontal span, split in two.
#define ACM_TAB_W           ((ACM_W - ACM_COL_HELD_X) / 2 - 0.1)

// Top-level config class — createDialog finds root-level classes from any
// loaded addon. ACE3's cargo/menu and similar dialogs use the same pattern.
class skua_admin_AdminMenu {
    idd = IDD_ADMIN_MENU;
    movingEnable = 0;
    enableSimulation = 1;

    onLoad = QUOTE(_this call FUNC(onAdminMenuOpen));
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
            text = "Admin Menu";
            x = QUOTE(POS_X(ACM_X));
            y = QUOTE(POS_Y(ACM_Y));
            w = QUOTE(POS_W(ACM_W));
            h = QUOTE(POS_H(ACM_TITLE_H));
        };

        // Tab bar — two buttons sitting under the title, over the right body
        // columns. Selected tab is reflected by fnc_switchAdminTab swapping
        // the button text (prefix "> ").
        class BtnTabCerts: RscButtonMenu {
            idc = IDC_ADMINMENU_BTN_TAB_CERTS;
            text = "Certifications";
            onButtonClick = QUOTE([ARR_2(ctrlParent (_this select 0),ADMIN_TAB_CERTS)] call FUNC(switchAdminTab));
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_TAB_Y));
            w = QUOTE(POS_W(ACM_TAB_W));
            h = QUOTE(POS_H(ACM_TAB_H));
        };
        class BtnTabAddons: BtnTabCerts {
            idc = IDC_ADMINMENU_BTN_TAB_ADDONS;
            text = "Addon List";
            onButtonClick = QUOTE([ARR_2(ctrlParent (_this select 0),ADMIN_TAB_ADDONS)] call FUNC(switchAdminTab));
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X + ACM_TAB_W + 0.2));
        };

        // "Online only" checkbox + label. Default ON via fnc_onAdminMenuOpen.
        // Label click toggles the box (RscCheckBox has no built-in text).
        class PlayerHeader: RscCheckBox {
            idc = IDC_ADMINCERT_CHK_ONLINE_ONLY;
            tooltip = "Show online players only. Uncheck to include everyone who's ever played.";
            onCheckedChanged = QUOTE(call FUNC(refreshCertMenu));
            x = QUOTE(POS_X(ACM_X + ACM_COL_PLAYER_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_BODY_Y));
            w = QUOTE(POS_W(ACM_CHK_BOX_W));
            h = QUOTE(POS_H(ACM_LIST_HEADER_H));
        };
        class PlayerHeaderLabel: RscText {
            idc = IDC_ADMINCERT_CHK_ONLINE_ONLY_LABEL;
            text = "Online only";
            tooltip = "Show online players only. Uncheck to include everyone who's ever played.";
            x = QUOTE(POS_X(ACM_X + ACM_COL_PLAYER_X + ACM_CHK_BOX_W + 0.2));
            y = QUOTE(POS_Y(ACM_Y + ACM_BODY_Y));
            w = QUOTE(POS_W(ACM_CHK_LABEL_W));
            h = QUOTE(POS_H(ACM_LIST_HEADER_H));
        };
        // Square icon-only button — flush-right in the Player column header.
        // Clicking re-fires the roster fetch via the server.
        class RefreshIcon: RscActivePicture {
            idc = IDC_ADMINCERT_BTN_REFRESH;
            text = "DBUG\pictures\reload.paa";
            tooltip = "Refresh roster";
            onButtonClick = QUOTE(call FUNC(fetchPlayerRoster));
            color[] = {1, 1, 1, 1};
            colorActive[] = {1, 1, 1, 0.7};
            x = QUOTE(POS_X(ACM_X + ACM_COL_PLAYER_W - ACM_REFRESH_ICON_W));
            y = QUOTE(POS_Y(ACM_Y + ACM_BODY_Y));
            w = QUOTE(POS_W(ACM_REFRESH_ICON_W));
            h = QUOTE(POS_H(ACM_LIST_HEADER_H));
        };
        class PlayerList: RscListBox {
            idc = IDC_ADMINCERT_PLAYER_LIST;
            x = QUOTE(POS_X(ACM_X + ACM_COL_PLAYER_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_LIST_BODY_Y));
            w = QUOTE(POS_W(ACM_COL_PLAYER_W));
            h = QUOTE(POS_H(ACM_LIST_H));
        };

        // --- Certs panel ------------------------------------------------------
        class HeldHeader: RscText {
            idc = IDC_ADMINCERT_HELD_TITLE;
            text = "Held Certs";
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_BODY_Y));
            w = QUOTE(POS_W(ACM_COL_HELD_W));
            h = QUOTE(POS_H(ACM_LIST_HEADER_H));
        };
        class HeldList: PlayerList {
            idc = IDC_ADMINCERT_HELD_LIST;
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X));
            w = QUOTE(POS_W(ACM_COL_HELD_W));
        };

        class AvailHeader: HeldHeader {
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

        // --- Addons panel -----------------------------------------------------
        // Hidden by default; fnc_switchAdminTab toggles visibility. Top half:
        // Missing Mods, spanning both right columns. Bottom half: Extra Mods
        // (resolved by client) and Extra Addons (orphans the client couldn't
        // attribute to a mod).
        class MissingHeader: HeldHeader {
            idc = IDC_ADMINMENU_MISSING_TITLE;
            text = "Missing Mods";
            x = QUOTE(POS_X(ACM_X + ACM_ADDON_AREA_X));
            w = QUOTE(POS_W(ACM_ADDON_AREA_W));
            show = 0;
        };
        class MissingList: RscListBox {
            idc = IDC_ADMINMENU_MISSING_LIST;
            x = QUOTE(POS_X(ACM_X + ACM_ADDON_AREA_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_LIST_BODY_Y));
            w = QUOTE(POS_W(ACM_ADDON_AREA_W));
            h = QUOTE(POS_H(ACM_ADDON_TOP_H));
            show = 0;
        };
        class ExtraModsHeader: HeldHeader {
            idc = IDC_ADMINMENU_EXTRA_MODS_TITLE;
            text = "Extra Mods";
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_ADDON_BOT_HDR_Y));
            w = QUOTE(POS_W(ACM_ADDON_LEFT_W));
            show = 0;
        };
        class ExtraModsList: RscListBox {
            idc = IDC_ADMINMENU_EXTRA_MODS_LIST;
            x = QUOTE(POS_X(ACM_X + ACM_COL_HELD_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_ADDON_BOT_Y));
            w = QUOTE(POS_W(ACM_ADDON_LEFT_W));
            h = QUOTE(POS_H(ACM_ADDON_BOT_H));
            show = 0;
        };
        class ExtraAddonsHeader: HeldHeader {
            idc = IDC_ADMINMENU_EXTRA_ADDONS_TITLE;
            text = "Extra Addons";
            x = QUOTE(POS_X(ACM_X + ACM_COL_AVAIL_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_ADDON_BOT_HDR_Y));
            w = QUOTE(POS_W(ACM_ADDON_RIGHT_W));
            show = 0;
        };
        class ExtraAddonsList: RscListBox {
            idc = IDC_ADMINMENU_EXTRA_ADDONS_LIST;
            x = QUOTE(POS_X(ACM_X + ACM_COL_AVAIL_X));
            y = QUOTE(POS_Y(ACM_Y + ACM_ADDON_BOT_Y));
            w = QUOTE(POS_W(ACM_ADDON_RIGHT_W));
            h = QUOTE(POS_H(ACM_ADDON_BOT_H));
            show = 0;
        };

        // One pair of action buttons flush-right under the Available column.
        // Their label + behavior changes based on which cert listbox (Held
        // or Available) was clicked last — Held → Revoke / Revoke (temp),
        // Available → Grant / Grant (temp). fnc_updateActionButtons keeps
        // them in sync; fnc_onCertMenuActionClicked routes the queue op.
        // Hidden on the Addons tab.
        class BtnActionTemp: RscButtonMenu {
            idc = IDC_ADMINCERT_BTN_ACTION_TEMP;
            text = "Grant (temp)";
            onButtonClick = QUOTE([true] call FUNC(onCertMenuActionClicked));
            x = QUOTE(POS_X(ACM_X + ACM_W - ACM_BTN_W));
            y = QUOTE(POS_Y(ACM_Y + ACM_BTN_ROW_Y));
            w = QUOTE(POS_W(ACM_BTN_W));
            h = QUOTE(POS_H(ACM_BTN_H));
        };
        class BtnAction: BtnActionTemp {
            idc = IDC_ADMINCERT_BTN_ACTION;
            text = "Grant";
            onButtonClick = QUOTE([false] call FUNC(onCertMenuActionClicked));
            x = QUOTE(POS_X(ACM_X + ACM_W - ACM_BTN_W*2 - 0.2));
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
