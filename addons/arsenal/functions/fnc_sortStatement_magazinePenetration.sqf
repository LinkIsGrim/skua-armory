#include "..\script_component.hpp"

params ["_magazineConfig", "_magazineClass"];

private _cfgAmmo = configFile >> "CfgAmmo";
private _ammoConfig = _cfgAmmo >> (getText (_magazineConfig >> "ammo"));
private _simulation = getText (_ammoConfig >> "simulation");

if (_simulation != "shotBullet") exitWith {
    private _submunition = getText (_ammoConfig >> "submunitionAmmo");
    if (_submunition != "") then {
        (getNumber (_cfgAmmo >> _submunition >> "caliber")) * 15 // return
    } else {
        0 // return
    };
};

private _caliber = getNumber (_ammoConfig >> "caliber");
private _muzzleVelocity = ["", _magazineConfig] call FUNC(getMagazineMuzzleVelocity);

_caliber * _muzzleVelocity * 0.015 // return
