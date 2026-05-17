#include "..\script_component.hpp"

params ["_code", "_args"];

if (GVAR(loaded)) exitWith {
    _args call _code;
};

GVAR(postLoadCode) pushBack [_code, _args];
