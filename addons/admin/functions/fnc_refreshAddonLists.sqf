#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Repopulate the Addons-tab listboxes for the currently selected player.
 * No-op if the dialog isn't open or the cached client addon map isn't
 * available yet.
 *
 * Layout:
 *  - Missing Mods : deduped mod names, alphabetical.
 *  - Extra Mods   : sorted mod names; "<unresolved>" prepended when orphans
 *                   exist. Selection drives Extra Addons via
 *                   fnc_refreshExtraAddons.
 *  - Extra Addons : addons belonging to the Extra-Mods selection (orphans
 *                   when "<unresolved>" is selected). Populated indirectly
 *                   via the Extra Mods LBSelChanged handler.
 *
 * Data flow:
 *  - Cache lives in uiNamespace QGVAR(clientAddonMap):
 *    UID -> [extras, missing, extrasModMap]
 *  - extrasModMap (HASHMAP<addon, [dir, name]>) is what the client shipped at
 *    connect; values' name is "" when CBA couldn't resolve the source mod →
 *    that addon goes into the "<unresolved>" bucket.
 *  - We bucket addons by mod name into a display variable (QGVAR(extrasByMod))
 *    so the Extra-Mods LBSelChanged handler can look up the addon list
 *    without walking the cache again.
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

#define UNRESOLVED_LABEL "<unresolved>"

private _display = findDisplay IDD_ADMIN_MENU;
if (isNull _display) exitWith {};

private _missingCtrl   = _display displayCtrl IDC_ADMINMENU_MISSING_LIST;
private _extraModsCtrl = _display displayCtrl IDC_ADMINMENU_EXTRA_MODS_LIST;
private _extraAddCtrl  = _display displayCtrl IDC_ADMINMENU_EXTRA_ADDONS_LIST;

lbClear _missingCtrl;
lbClear _extraModsCtrl;
lbClear _extraAddCtrl;

_display setVariable [QGVAR(extrasByMod), createHashMap];

private _map = uiNamespace getVariable [QGVAR(clientAddonMap), nil];
if (isNil "_map") exitWith {}; // Not fetched yet.

private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;
private _selIdx = lbCurSel _playerList;
if (_selIdx < 0) exitWith {};
private _uid = _playerList lbData _selIdx;
if (_uid isEqualTo "" || !(_uid in _map)) exitWith {};

(_map get _uid) params ["_extras", "_missing", ["_extrasModMap", createHashMap], ["_missingModMap", createHashMap]];

// Missing mods — resolved server-side since the admin client doesn't have
// them loaded and can't do configSourceMod on them.
private _missingModNames = createHashMap;
{
    _missingModNames set [(_missingModMap getOrDefault [_x, ["", ""]]) select 1, nil];
} forEach _missing;
_missingModNames deleteAt "";
private _sortedMissing = keys _missingModNames;
_sortedMissing sort true;
{_missingCtrl lbAdd _x} forEach _sortedMissing;

// Bucket extras by resolved mod name. Empty name → orphan bucket keyed by
// UNRESOLVED_LABEL so the lookup path is uniform.
private _extrasByMod = createHashMap;
{
    private _entry = _extrasModMap getOrDefault [_x, ["", ""]];
    _entry params ["", "_name"];
    private _bucketKey = [_name, UNRESOLVED_LABEL] select (_name isEqualTo "");
    private _bucket = _extrasByMod getOrDefault [_bucketKey, []];
    _bucket pushBack _x;
    _extrasByMod set [_bucketKey, _bucket];
} forEach _extras;

// Sort each bucket alphabetically for stable display.
{
    (_extrasByMod get _x) sort true;
} forEach (keys _extrasByMod);

_display setVariable [QGVAR(extrasByMod), _extrasByMod];

// Extra Mods list: unresolved bucket pinned at the top (if any), then mod
// names alphabetical.
private _modNames = (keys _extrasByMod) - [UNRESOLVED_LABEL];
_modNames sort true;

if (UNRESOLVED_LABEL in _extrasByMod) then {
    _extraModsCtrl lbAdd UNRESOLVED_LABEL;
};
{_extraModsCtrl lbAdd _x} forEach _modNames;

// Drive the Extra Addons list by selecting the first row; the LBSelChanged
// handler wired in fnc_onAdminMenuOpen does the actual population.
if (lbSize _extraModsCtrl > 0) then {
    _extraModsCtrl lbSetCurSel 0;
};
