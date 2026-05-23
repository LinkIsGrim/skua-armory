#include "..\script_component.hpp"
/*
 * Authors: LinkIsGrim
 * Handles pulling Rope Crate.
 *
 * Arguments:
 * 0: Crate <OBJECT>
 *
 * Return Value:
 * None.
 *
 * Example:
 * cursorTarget call skua_missions_fnc_logistics_onPullRopeCrate;
 *
 * Public: No
 */

params ["_object"];
TRACE_1("fnc_logistics_onPullRopeCrate",_this);

[{
    clearMagazineCargoGlobal _this;
    clearWeaponCargoGlobal _this;
    clearItemCargoGlobal _this;
    clearBackpackCargoGlobal _this;

    _this addItemCargoGlobal ["ACE_rope12", 2];
    _this addItemCargoGlobal ["ACE_rope18", 2];
    _this addItemCargoGlobal ["ACE_rope27", 1];
}, _object] call CBA_fnc_execNextFrame;
