#include "..\script_component.hpp"

params ["_weapon"];

getNumber (configFile >> "CfgWeapons" >> _weapon >> "WeaponSlotsInfo" >> "mass")