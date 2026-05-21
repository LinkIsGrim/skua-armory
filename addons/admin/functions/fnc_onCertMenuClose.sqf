#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Dialog onUnload handler. Removes the addonMapLoaded listener wired in
 * fnc_onAdminMenuOpen. Refresh handlers are idempotent and short-circuit
 * when findDisplay returns null.
 *
 * Arguments:
 * 0: Display <DISPLAY>
 * 1: Exit code <NUMBER>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_display"];

private _addonMapListener = _display getVariable [QGVAR(addonMapListener), -1];
if (_addonMapListener >= 0) then {
    [QGVAR(addonMapLoaded), _addonMapListener] call CBA_fnc_removeEventHandler;
};
