#include "..\script_component.hpp"

params ["_unit"];

if (_unit getVariable QGVAR(equipped) == "counterweight") exitWith {};

GVAR(counterweightPenalty) = COUNTERWEIGHT_PRIMARY_WEIGHT_INCREASE * ((primaryWeapon _unit) call FUNC(getWeaponMass));
[_unit, _unit, GVAR(counterweightPenalty)] call ACEFUNC(movement,addLoadToUnitContainer);
_unit setUnitRecoilCoefficient COUNTERWEIGHT_RECOIL_COEF

GVAR(counterweightPlayerEH) = ["loadout", {
    params ["_unit"];
    [_unit, _unit, -GVAR(counterweightPenalty)] call ACEFUNC(movement,addLoadToUnitContainer);
    GVAR(counterweightPenalty) = COUNTERWEIGHT_PRIMARY_WEIGHT_INCREASE * ((primaryWeapon _unit) call FUNC(getWeaponMass));
    [_unit, _unit, GVAR(counterweightPenalty)] call ACEFUNC(movement,addLoadToUnitContainer);
}] call CBA_fnc_addPlayerEventHandler;
