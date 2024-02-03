#include "..\script_component.hpp"

params ["_display"];

private _loadout = getUnitLoadout ACEGVAR(arsenal,center);
[ACEGVAR(arsenal,center), _loadout, _loadout] call FUNC(handleLoadout);
