// IDC/IDD constants for the admin menu dialog.
// IDD chosen high enough to avoid collision with engine displays.

#define IDD_ADMIN_MENU                 14710
// Backwards-compat alias — older references read the cert menu by this name.
// Both IDDs resolve to the same dialog; do not duplicate-define elsewhere.
#define IDD_ADMIN_CERT_MENU            IDD_ADMIN_MENU

#define IDC_ADMINCERT_PLAYER_LIST      14711
#define IDC_ADMINCERT_HELD_LIST        14712
#define IDC_ADMINCERT_AVAILABLE_LIST   14713

// One pair of action buttons; their label + behavior changes based on which
// cert listbox (Held vs Available) was clicked last. See fnc_updateActionButtons.
#define IDC_ADMINCERT_BTN_ACTION       14720
#define IDC_ADMINCERT_BTN_ACTION_TEMP  14721
#define IDC_ADMINCERT_BTN_CLOSE        14724
#define IDC_ADMINCERT_BTN_COMMIT       14725
#define IDC_ADMINCERT_BTN_REFRESH      14726

#define IDC_ADMINCERT_CHK_ONLINE_ONLY        14740
#define IDC_ADMINCERT_CHK_ONLINE_ONLY_LABEL  14741

#define IDC_ADMINCERT_TITLE            14730
#define IDC_ADMINCERT_HELD_TITLE       14731
#define IDC_ADMINCERT_AVAILABLE_TITLE  14732

// Tab bar + Addons panel.
#define IDC_ADMINMENU_BTN_TAB_CERTS    14750
#define IDC_ADMINMENU_BTN_TAB_ADDONS   14751

#define IDC_ADMINMENU_MISSING_TITLE       14752
#define IDC_ADMINMENU_MISSING_LIST        14753
#define IDC_ADMINMENU_EXTRA_MODS_TITLE    14754
#define IDC_ADMINMENU_EXTRA_MODS_LIST     14755
#define IDC_ADMINMENU_EXTRA_ADDONS_TITLE  14756
#define IDC_ADMINMENU_EXTRA_ADDONS_LIST   14757
#define IDC_ADMINMENU_MISSING_ADDONS_TITLE  14758
#define IDC_ADMINMENU_MISSING_ADDONS_LIST   14759

// Tab IDs used in QGVAR(activeTab).
#define ADMIN_TAB_CERTS  0
#define ADMIN_TAB_ADDONS 1

#define COLOR_BCG { \
    "(profilenamespace getVariable ['GUI_BCG_RGB_R',0.13])", \
    "(profilenamespace getVariable ['GUI_BCG_RGB_G',0.54])", \
    "(profilenamespace getVariable ['GUI_BCG_RGB_B',0.21])", \
    "(profilenamespace getVariable ['GUI_BCG_RGB_A',0.8])" \
}

#define POS_X(N) ((N) * GUI_GRID_W + GUI_GRID_CENTER_X)
#define POS_Y(N) ((N) * GUI_GRID_H + GUI_GRID_CENTER_Y)
#define POS_W(N) ((N) * GUI_GRID_W)
#define POS_H(N) ((N) * GUI_GRID_H)
