#include "..\script_component.hpp"

params ["_unit"];

private _roles = [_unit] call FUNC(findRoles);

private _missing = [];

private _items = items _unit;

if (GVAR(requireShovel) && {!([ACE_player] call ACEFUNC(trenches,hasEntrenchingTool))}) then {
    _missing pushBack "Why don't you have a shovel?";
};

if (uniform _unit isEqualTo "") then {
    _missing pushBack "A Uniform";
};

if (GVAR(requireVest)) then {
    if (vest _unit isEqualTo "") then {
        _missing pushBack "A Vest";
    };
};

// Check for a weapon
if (GVAR(requirePrimary)) then {
    private _weapon = primaryWeapon _unit;
    if (_weapon isEqualTo "") then {
        _missing pushBack "A Rifle";
    };
    private _rounds = 0;
    {
        if (_weapon canAdd (_x#0)) then {
            _rounds = _rounds + _x#1;
        };
    } forEach magazinesAmmoFull _unit;
    if (_rounds < GVAR(requirePrimaryAmmo)) then {
        _missing pushBack format ["%1 Primary Rounds", GVAR(requirePrimaryAmmo) - _rounds];
    };
};

if (GVAR(requireHandgun)) then {
    private _weapon = handgunWeapon _unit;
    if (_weapon isEqualTo "") then {
        _missing pushBack "A Handgun";
    };
    private _rounds = 0;
    {
        if (_weapon canAdd (_x#0)) then {
            _rounds = _rounds + _x#1;
        };
    } forEach magazinesAmmoFull _unit;
    if (_rounds < GVAR(requireHandgunAmmo)) then {
        _missing pushBack format ["%1 Handgun Rounds", GVAR(requireHandgunAmmo) - _rounds];
    };
};

/*if (GVAR(requireRadio) && {!("sl" in _roles)}) then {
    [["ACRE_BF888S"], 1, "Baofeng 888S", _missing] call FUNC(countItem);
};*/

if (GVAR(requireRadio) && {!("TFAR_anprc152" in _new#9#2)}) then {
    _missing pushBack "1 AN/PRC-152";
};

[["ACE_fieldDressing", "ACE_elasticBandage", "ACE_packingBandage", "ACE_quikclot"],
                            15, "Bandages", _missing] call FUNC(countItem);
[["ACE_splint"],            2, "Splints", _missing] call FUNC(countItem);
[["ACE_morphine"],          2, "Morphine Autoinjector", _missing] call FUNC(countItem);
[["ACE_epinephrine"],       1, "Epinephrine Autoinjector", _missing] call FUNC(countItem);
[["ACE_tourniquet"],        4, "Tourniquets", _missing] call FUNC(countItem);
[["SmokeShell"],            2, "Smoke Grenade (White)", _missing] call FUNC(countItem);
[["ACE_CableTie"],          2, "Cable Tie", _missing] call FUNC(countItem);

if (_new#9#0 isEqualTo "") then {
    _missing pushBack "A Map";
};

if (_new#9#1 isEqualTo "") then {
    _missing pushBack "A GPS";
};

if (_new#9#3 isEqualTo "") then {
    _missing pushBack "A Compass";
};

if (_new#9#4 isEqualTo "") then {
    _missing pushBack "A Watch";
};

_missing
