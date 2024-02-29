#include "..\script_component.hpp"

params ["_unit"];

if (_unit getVariable QGVAR(equipped) == "packMule") exitWith {};

GVAR(packMuleIgnoredWeight) = PACKMULE_IGNORED_COEF * loadAbs _unit;
[_unit, _unit, -GVAR(packMuleIgnoredWeight)] call ACEFUNC(movement,addLoadToUnitContainer);

GVAR(packMulePlayerEH) = ["loadout", {
    params ["_unit"];
    [_unit, _unit, GVAR(packMuleIgnoredWeight)] call ACEFUNC(movement,addLoadToUnitContainer);
    GVAR(packMuleIgnoredWeight) = PACKMULE_IGNORED_COEF * loadAbs _unit;
    [_unit, _unit, -GVAR(packMuleIgnoredWeight)] call ACEFUNC(movement,addLoadToUnitContainer);
}] call CBA_fnc_addPlayerEventHandler;