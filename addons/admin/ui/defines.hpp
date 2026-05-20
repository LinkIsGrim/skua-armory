// IDC/IDD constants for the admin cert menu dialog.
// IDD chosen high enough to avoid collision with engine displays.

#define IDD_ADMIN_CERT_MENU            14710

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
