#include "..\script_component.hpp"

params ["_newUnit", "_oldUnit"];

if (!isNull _oldUnit) then {
    [QGVAR(setUnitTrait), [_oldUnit, "camouflageCoef", 1], _oldUnit] call CBA_fnc_targetEvent;
    _oldUnit removeEventHandler ["SlotItemChanged", _oldUnit getVariable [QGVAR(SlotItemChangedID), -1]];
    _oldUnit setVariable [QGVAR(SlotItemChangedID), nil];
};

[QEGVAR(common,setCamoCoef), [_newUnit, (uniform _newUnit) call FUNC(getUniformVisibility)]] call CBA_fnc_localEvent;

_newUnit setVariable [QGVAR(SlotItemChangedID), _newUnit addEventHandler ["SlotItemChanged", LINKFUNC(handleSlotItemChanged)]];