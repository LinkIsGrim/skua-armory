#include "script_component.hpp"

["CBA_loadoutGet", {
    params ["_unit", "_loadoutArray", "_extendedInfo"];

    if ("grad_slingHelmet_main" call ACEFUNC(common,isModLoaded)) then {
        private _helmetClass = _unit call GRAD_slingHelmet_fnc_getSlungHelmet;
        if (_helmetClass != "") then {
            _extendedInfo set ["grad_slingHelmet_class", _helmetClass];
        };
    };
}] call CBA_fnc_addEventHandler;

["CBA_loadoutSet", {
    params ["_unit", "_loadoutArray", "_extendedInfo"];

    private _helmetClass = _extendedInfo getOrDefault ["grad_slingHelmet_class", ""];
    private _inArsenal = !isNull ACEGVAR(arsenal,currentBox);
    if (_inArsenal) then {
        if (_helmetClass != "" && {!(_helmetClass in ACEGVAR(arsenal,virtualItemsFlat))}) then {
            _helmetClass = "";
        };
    };
    if ("grad_slingHelmet_main" call ACEFUNC(common,isModLoaded)) then {
        if (_helmetClass call GRAD_slingHelmet_fnc_isWhitelisted) then {
            [_unit, _helmetClass] call GRAD_slingHelmet_fnc_addSlungHelmet;
        };
    };
}] call CBA_fnc_addEventHandler;