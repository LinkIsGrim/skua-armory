#include "script_component.hpp"

if (isServer) then {
    // Don't create base arsenal if it already exists
    // This lets mission makers create the base arsenal with a limited selection themselves if they want to
    {
        if (!isNull _y) then {
            continue;
        };

        GVAR(baseArsenals) set [_x, createVehicle ["Land_HelipadEmpty_F", [0, 0, 0], [], 0, "NONE"]];

        [GVAR(baseArsenals) get _x, true, true] call ACEFUNC(arsenal,initBox);
    } forEach GVAR(baseArsenals);

    publicVariable QGVAR(baseArsenals); // make sure to sync
};

if (hasInterface) then {
    private _fieldLogiAction = [
        QGVAR(fieldLogisticsMenu),
        "Field Logistics",
        "\A3\ui_f\data\igui\cfg\simpletasks\types\rearm_ca.paa",
        {true},
        {
            alive _target && {
                [_target] call ACEFUNC(repair,isRepairVehicle)
                || {!(_target isKindOf "AllVehicles")
                    && {_target getVariable ["ACE_isRepairFacility",
                        getNumber (configOf _target >> QGVAR(canRepair)) max
                        getNumber (configOf _target >> "transportRepair")] in [1, true]}}
            }
        },
        {_target call FUNC(makeFieldLogisticsActions)}
    ] call ACEFUNC(interact_menu,createAction);

    {
        [_x, 0, ["ACE_MainActions"], _fieldLogiAction, true] call ACEFUNC(interact_menu,addActionToClass);
    } forEach ["Car", "Tank", "Helicopter", "Plane", "Ship", "Static", "Building", "ThingX"];
};

if (isServer && isMultiplayer) then {
    [{
        [{time > 0}, {
            [QGVAR(missionStarted), []] call CBA_fnc_globalEventJIP;
        }] call CBA_fnc_waitUntilAndExecute; 
    }] call CBA_fnc_execNextFrame;
};
