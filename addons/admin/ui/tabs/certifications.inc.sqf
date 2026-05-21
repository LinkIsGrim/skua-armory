// Tab descriptor + panel-internal event wiring: Certifications.
// Sourced from fnc_onAdminMenuOpen with `_display` and `_tabs` in scope.

_tabs set [ADMIN_TAB_CERTS, createHashMapFromArray [
    ["label",            "Certifications"],
    ["tabBtnIdc",        IDC_ADMINMENU_BTN_TAB_CERTS],
    ["panelIdcs", [
        IDC_ADMINCERT_HELD_TITLE,
        IDC_ADMINCERT_HELD_LIST,
        IDC_ADMINCERT_AVAILABLE_TITLE,
        IDC_ADMINCERT_AVAILABLE_LIST,
        IDC_ADMINCERT_BTN_ACTION,
        IDC_ADMINCERT_BTN_ACTION_TEMP,
        IDC_ADMINCERT_BTN_COMMIT
    ]],
    ["onRefresh",        {call FUNC(refreshCertLists)}],
    ["onActivate",       {call FUNC(refreshPlayerList)}],
    ["onDeactivate",     {}],
    ["forcesOnlineOnly", false],
    ["showsRefreshIcon", true]
]];

// Focus tracking on Held/Avail drives the shared action buttons (Grant /
// Revoke + their _TEMP variants). updateActionButtons reads the focused
// list IDC to decide which op to queue.
private _heldList = _display displayCtrl IDC_ADMINCERT_HELD_LIST;
_heldList ctrlAddEventHandler ["LBSelChanged", {
    private _display = findDisplay IDD_ADMIN_MENU;
    _display setVariable [QGVAR(focusedCertList), IDC_ADMINCERT_HELD_LIST];
    call FUNC(updateActionButtons);
}];
private _availList = _display displayCtrl IDC_ADMINCERT_AVAILABLE_LIST;
_availList ctrlAddEventHandler ["LBSelChanged", {
    private _display = findDisplay IDD_ADMIN_MENU;
    _display setVariable [QGVAR(focusedCertList), IDC_ADMINCERT_AVAILABLE_LIST];
    call FUNC(updateActionButtons);
}];
