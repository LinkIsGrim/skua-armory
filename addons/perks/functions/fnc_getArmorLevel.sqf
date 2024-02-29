#include "..\script_component.hpp"

params ["_unit", "_hitpoint"];

private _return = switch (_hitpoint) do {
    case "HitChest": {
        ([vest _unit, _hitpoint] call ACEFUNC(medical_engine,getItemArmor)) params ["_armor", "_armorScaled"];
        if (_armor > 24) then {
            _armor = _armorScaled;
        };
        round (_armor / 5)
    };
    case "HitHead": {
        ([headgear _unit, _hitpoint] call ACEFUNC(medical_engine,getItemArmor)) params ["_armor"];
        round (_armor / 4)
    };
    default {
        ([uniform _unit, _hitpoint] call ACEFUNC(medical_engine,getItemArmor)) params ["_armor"];
        round (_armor / 4)
    };
};

_return