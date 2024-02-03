#include "..\script_component.hpp"

params ["_display"];

{
    private _ctrl = _display displayCtrl _x;
    _ctrl ctrlShow false;
    _ctrl ctrlCommit 0.15;
} forEach CUSTOM_BOXES;
