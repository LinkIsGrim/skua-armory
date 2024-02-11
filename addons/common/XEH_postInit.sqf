#include "script_component.hpp"

#define BASELINE_CAMO_COEF 1.4

if (isServer) then {
    GVAR(zeusChannel) = radioChannelCreate [[248/256,148/256,6/256,1], "Zeus Chat", "Zeus (%UNIT_NAME)", [], false];
};

["CAManBase", "SlotItemChanged", {
    params ["_unit", "_name", "_slot", "_assigned"];
    if (!local _unit || {_slot != TYPE_UNIFORM}) exitWith {};
    if (!_assigned) exitWith {
        _unit setUnitTrait ["camouflageCoef", 1];
    };
    private _camoCoef = getNumber (configFile >> "CfgVehicles" >> getText (configFile >> "CfgWeapons" >> _uniform >> "ItemInfo" >> "uniformClass") >> "camouflage");
    _camoCoef = _camoCoef/BASELINE_CAMO_COEF;
    _unit setUnitTrait ["camouflageCoef", sqrt(_camoCoef)];
}] call CBA_fnc_addEventHandler;

[QACEGVAR(zeus,zeusUnitAssigned), {
    params ["_logic", "_unit"];

    systemChat format ["Zeus Module assigned to %1", name _unit];

    if (isServer) then {
        GVAR(zeusChannel) radioChannelAdd [_unit];
    };
}] call CBA_fnc_addEventHandler;