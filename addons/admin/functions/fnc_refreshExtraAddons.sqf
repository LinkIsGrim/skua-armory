#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Repopulate the Extra Addons listbox with the addons belonging to the
 * currently-selected Extra Mods entry. Driven by the Extra Mods list's
 * LBSelChanged handler.
 *
 * Lookup comes from the display variable QGVAR(extrasByMod) (HashMap<modName,
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

private _extraModsCtrl = _display displayCtrl IDC_ADMINMENU_EXTRA_MODS_LIST;
private _extraAddCtrl  = _display displayCtrl IDC_ADMINMENU_EXTRA_ADDONS_LIST;

lbClear _extraAddCtrl;

private _selIdx = lbCurSel _extraModsCtrl;
if (_selIdx < 0) exitWith {};

private _key = _extraModsCtrl lbText _selIdx;
private _extrasByMod = _display getVariable [QGVAR(extrasByMod), createHashMap];
private _addons = _extrasByMod getOrDefault [_key, []];

{_extraAddCtrl lbAdd _x} forEach _addons;
