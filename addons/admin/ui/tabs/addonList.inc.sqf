// Tab descriptor + panel-internal event wiring: Addon List.
// Sourced from fnc_onAdminMenuOpen with `_display` and `_tabs` in scope.
//
// forcesOnlineOnly = true because offline players can't report their addons.

_tabs set [ADMIN_TAB_ADDONS, createHashMapFromArray [
    ["label",            "Addon List"],
    ["tabBtnIdc",        IDC_ADMINMENU_BTN_TAB_ADDONS],
    ["panelIdcs", [
        IDC_ADMINMENU_MISSING_TITLE,
        IDC_ADMINMENU_MISSING_LIST,
        IDC_ADMINMENU_MISSING_ADDONS_TITLE,
        IDC_ADMINMENU_MISSING_ADDONS_LIST,
        IDC_ADMINMENU_EXTRA_MODS_TITLE,
        IDC_ADMINMENU_EXTRA_MODS_LIST,
        IDC_ADMINMENU_EXTRA_ADDONS_TITLE,
        IDC_ADMINMENU_EXTRA_ADDONS_LIST
    ]],
    ["onRefresh",        {call FUNC(refreshAddonLists)}],
    ["onActivate", {
        // Bulk fetch on first switch; otherwise refresh from cache. The
        // shared player list also redraws so offline rows get filtered out
        // now that forcesOnlineOnly has pinned the checkbox.
        if (isNil {uiNamespace getVariable QGVAR(clientAddonMap)}) then {
            call FUNC(fetchClientAddonMap);
        } else {
            call FUNC(refreshAddonLists);
        };
        call FUNC(refreshPlayerList);
    }],
    ["onDeactivate",     {call FUNC(refreshPlayerList)}],
    ["forcesOnlineOnly", true],
    ["showsRefreshIcon", false]
]];

// Selecting an Extra Mods entry repopulates Extra Addons with that mod's
// addons. The "<unresolved>" pseudo-entry routes through the same lookup.
private _extraModsList = _display displayCtrl IDC_ADMINMENU_EXTRA_MODS_LIST;
_extraModsList ctrlAddEventHandler ["LBSelChanged", {call FUNC(refreshExtraAddons)}];

// Selecting a Missing Mods entry repopulates Missing Addons with that mod's
// addons, same pattern as Extra Mods -> Extra Addons.
private _missingList = _display displayCtrl IDC_ADMINMENU_MISSING_LIST;
_missingList ctrlAddEventHandler ["LBSelChanged", {call FUNC(refreshMissingAddons)}];
