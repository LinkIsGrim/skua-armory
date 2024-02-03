#include "..\script_component.hpp"

params ["_unit", "_new", "_old"];

GVAR(itemCache) = [];
{
    /*if ([_x] call acre_api_fnc_isRadio) then {
        GVAR(itemCache) pushBack ([_x] call acre_api_fnc_getBaseRadio);
    } else {
        GVAR(itemCache) pushBack _x;
    };*/
    if ("TFAR_" in _x) then {
        private _splitString = _x splitString "_";
        if (count _splitString > 2) then {
            _splitString = _splitString select [0, 2];
        };
        GVAR(itemCache) pushBack (_splitString joinString "_");
    } else {
        GVAR(itemCache) pushBack _x;
    };
} forEach (items _unit) + (magazines _unit);

if !(GVAR(enabled)) exitWith {
    {
        private _ctrl = GVAR(arsenalDisplay)#0 displayCtrl _x;
        _ctrl ctrlShow false;
        _ctrl ctrlCommit 0.15;
    } forEach CUSTOM_BOXES;
};

private _missingRequired = [_unit] call FUNC(getMissingGear);
private _missingSpecial = [_unit] call FUNC(getSpecialGear);

INFO_1("%1",_missingRequired);
INFO_1("%1",_missingSpecial);

private _yOffset = 0;

if (_missingRequired isEqualTo []) then {
    {
        private _ctrl = GVAR(arsenalDisplay)#0 displayCtrl _x;
        _ctrl ctrlShow false;
        _ctrl ctrlCommit 0.15;
    } forEach [IDC_requiredGearBox, IDC_requiredGearTitle];
} else {
    private _ctrlBox = GVAR(arsenalDisplay)#0 displayCtrl IDC_requiredGearBox;
    _ctrlBox ctrlSetPosition [
        safezoneX + safezoneW - (94 + 48) * GRID_W,
        safezoneY + 1.8 * GRID_H,
        47 * GRID_W,
        (5 + (4 * count _missingRequired)) * GRID_H
    ];
    _ctrlBox ctrlCommit 0;
    _yOffset = _yOffset + (4 * count _missingRequired) + 6;

    private _ctrlText = GVAR(arsenalDisplay)#0 displayCtrl IDC_requiredGearText;
    _ctrlText ctrlSetStructuredText parseText (_missingRequired joinString "<br/>");
    _ctrlText ctrlSetPosition [
        0 * GRID_W,
        5 * GRID_H,
        45 * GRID_W,
        (4 * count _missingRequired) * GRID_H
    ];
    _ctrlText ctrlCommit 0;

    {
        private _ctrl = GVAR(arsenalDisplay)#0 displayCtrl _x;
        _ctrl ctrlShow true;
        _ctrl ctrlCommit 0.15;
    } forEach [IDC_requiredGearBox, IDC_requiredGearTitle];
};

if (_missingSpecial isEqualTo []) then {
    {
        private _ctrl = GVAR(arsenalDisplay)#0 displayCtrl _x;
        _ctrl ctrlShow false;
        _ctrl ctrlCommit 0.15;
    } forEach [IDC_specialGearBox, IDC_specialGearTitle];
} else {
    private _ctrlTitle = GVAR(arsenalDisplay)#0 displayCtrl IDC_specialGearTitle;
    _ctrlTitle ctrlSetPosition [
        safezoneX + safezoneW - (94 + 48) * GRID_W,
        safezoneY + (1.75 + _yOffset) * GRID_H,
        47 * GRID_W,
        5 * GRID_H
    ];
    _ctrlTitle ctrlCommit 0;

    private _ctrlBox = GVAR(arsenalDisplay)#0 displayCtrl IDC_specialGearBox;
    _ctrlBox ctrlSetPosition [
        safezoneX + safezoneW - (94 + 48) * GRID_W,
        safezoneY + (1.8 + _yOffset) * GRID_H,
        47 * GRID_W,
        (5 + (4 * count _missingSpecial)) * GRID_H
    ];
    _ctrlBox ctrlCommit 0;

    private _ctrlText = GVAR(arsenalDisplay)#0 displayCtrl IDC_specialGearText;
    _ctrlText ctrlSetStructuredText parseText (_missingSpecial joinString "<br/>");
    _ctrlText ctrlSetPosition [
        0 * GRID_W,
        5 * GRID_H,
        45 * GRID_W,
        (4 * count _missingSpecial) * GRID_H
    ];
    _ctrlText ctrlCommit 0;

    {
        private _ctrl = GVAR(arsenalDisplay)#0 displayCtrl _x;
        _ctrl ctrlShow true;
        _ctrl ctrlCommit 0.15;
    } forEach [IDC_specialGearBox, IDC_specialGearTitle];
};
