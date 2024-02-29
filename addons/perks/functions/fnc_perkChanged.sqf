#include "..\script_component.hpp"
params ["_unit", "_oldPerk", "_newPerk"];

if (_oldPerk != "") then {
    private _fnc_perkUnequip = missionNamespace getVariable format [QFUNC(unequip_%1), _oldPerk];
    _unit call _fnc_perkUnequip;
};

if (_newPerk != "") then {
    private _fnc_perkEquip = missionNamespace getVariable format [QFUNC(equip_%1), _newPerk];
    _unit call _fnc_perkEquip;
};

_unit setVariable [QGVAR(equipped), _newPerk, true];