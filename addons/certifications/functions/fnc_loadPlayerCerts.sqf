#include "..\script_component.hpp"

params ["_uid", "_hasInterface", "_player"];

if (!_hasInterface) exitWith {};

"skua" callExtension ["certification:load_player", [_uid]];
