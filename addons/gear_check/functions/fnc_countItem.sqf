#include "..\script_component.hpp"

params ["_items", "_itemMap", "_count", "_pretty", "_missing", "_missingClasses"];

private _have = 0;
{
    _have = _have + (_itemMap getOrDefault [_x, 0]);
} forEach _items;

if (_have < _count) exitWith {
    private _qnt = _count - _have;
    _missing pushBackUnique format ["%1 %2", _qnt, _pretty select (_qnt > 1)];
    _missingClasses pushBack ["#item", _items, _qnt];
};
