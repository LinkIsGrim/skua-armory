#include "..\script_component.hpp"

params ["_uid", "_hasInterface", "_player"];

if (!_hasInterface) exitWith {};

"skua" callExtension ["database:upsert_player", [_uid, name _player]];
