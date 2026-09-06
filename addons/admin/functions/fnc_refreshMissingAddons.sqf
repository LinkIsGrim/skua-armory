#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Repopulate the Missing Addons listbox with the addons belonging to the
 * currently-selected Missing Mods entry. Driven by the Missing Mods list's
 * LBSelChanged handler.
 *
 * Lookup comes from the display variable QGVAR(missingByMod) (HashMap<modName,
 * sorted addons>) populated by fnc_refreshAddonLists. Selection is read by
 * label text — UNRESOLVED_LABEL ("<unresolved>") falls into the same bucket
 * as any resolved mod name.
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

private _display = findDisplay IDD_ADMIN_MENU;
if (isNull _display) exitWith {};

private _missingCtrl    = _display displayCtrl IDC_ADMINMENU_MISSING_LIST;
private _missingAddCtrl = _display displayCtrl IDC_ADMINMENU_MISSING_ADDONS_LIST;

lbClear _missingAddCtrl;

private _selIdx = lbCurSel _missingCtrl;
if (_selIdx < 0) exitWith {};

private _key = _missingCtrl lbText _selIdx;
private _missingByMod = _display getVariable [QGVAR(missingByMod), createHashMap];
private _addons = _missingByMod getOrDefault [_key, []];

{_missingAddCtrl lbAdd _x} forEach _addons;
