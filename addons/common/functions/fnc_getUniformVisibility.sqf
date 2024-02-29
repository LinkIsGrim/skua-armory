#include "..\script_component.hpp"

#define BASELINE_CAMO_COEF 1.4

params ["_uniform"];

if (_uniform == "") exitWith {1};

GVAR(camoCoefMap) getOrDefaultCall [_uniform, {
    (getNumber (configFile >> "CfgVehicles" >> getText (configFile >> "CfgWeapons" >> _uniform >> "ItemInfo" >> "uniformClass") >> "camouflage")) / BASELINE_CAMO_COEF
}, true];