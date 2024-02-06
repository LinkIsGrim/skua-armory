#include "..\script_component.hpp"

params ["_items", "_itemMap", "_count", "_pretty", "_missing", "_missingClasses"];

private _have = 0;
{
    _have = _have + (_itemMap getOrDefault [_x, 0]);
} forEach _items;

if (_have < _count) exitWith {
    private _qnt = _count - _have;
    if (_pretty isEqualType "") then {
        _pretty = [_pretty, _pretty];
    } else {
        _pretty params ["_singular", ["_plural", ""]];
        if (_plural == "") then {
            _pretty set [1, _singular];
        };
    };
    _missing pushBackUnique format ["%1 %2", _qnt, _pretty select (_qnt > 1)];
    _missingClasses pushBack ["#item", _items, _qnt];
};
