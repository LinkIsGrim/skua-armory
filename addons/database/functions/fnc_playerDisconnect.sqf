#include "..\script_component.hpp"

params ["_uid", "_name"];

"skua" callExtension ["database:player_disconnect", [_uid, _name]];
