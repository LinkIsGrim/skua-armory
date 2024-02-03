#include "..\script_component.hpp"

params ["_items", "_count", "_pretty", "_missing"];

private _have = ({ _x in _items } count GVAR(itemCache));
if (_have < _count) exitWith {
    _missing pushBackUnique format ["%1 %2", _count - _have, _pretty];
};
