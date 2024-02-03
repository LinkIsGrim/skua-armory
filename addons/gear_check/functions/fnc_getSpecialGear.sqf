#include "..\script_component.hpp"

params ["_unit"];

private _roles = [_unit] call FUNC(findRoles);

private _missing = [];

{
    switch (tolower _x) do {
        case "medic": {
            [["ACE_bodyBag_blue"],      3, "Bodybags (Blue)", _missing] call FUNC(countItem);
            [["ACE_bodyBag_white"],     2, "Bodybags (White)", _missing] call FUNC(countItem);
            [["ACE_adenosine"],         2, "Adenosine", _missing] call FUNC(countItem);
            [["ACE_epinephrine"],       2, "Epinephrine", _missing] call FUNC(countItem);
            [["ACE_morphine"],          2, "Morphine", _missing] call FUNC(countItem);
            [["ACE_elasticBandage"],    30, "Bandages (Elastic)", _missing] call FUNC(countItem);
            [["ACE_packingBandage"],    20, "Bandages (Packing)", _missing] call FUNC(countItem);
            [["ACE_quikclot"],          20, "Bandages (Quikclot)", _missing] call FUNC(countItem);
            [["ACE_suture"],            40, "Sutures", _missing] call FUNC(countItem);
            [["ACE_salineIV"],          3, "Saline IV (1000ml)", _missing] call FUNC(countItem);
            [["ACE_salineIV_500"],      6, "Saline IV (500ml)", _missing] call FUNC(countItem);
            [["ACE_salineIV_250"],      4, "Saline IV (250ml)", _missing] call FUNC(countItem);
            [["ACE_splint"],            5, "Splints", _missing] call FUNC(countItem);
            [["ACE_tourniquet"],        8, "Tourniquets", _missing] call FUNC(countItem);
            [["ACE_surgicalKit"],       1, "Surgical Kit", _missing] call FUNC(countItem);
            // [["kat_chestSeal"],         2, "Chest Seals", _missing] call FUNC(countItem);
            // [["kat_guedel"],            6, "Guedel Tubes", _missing] call FUNC(countItem);
            // [["kat_larynx"],            2, "KingLT", _missing] call FUNC(countItem);
            // [["kat_Pulseoximeter"],     2, "Pulse Oximeter", _missing] call FUNC(countItem);
            // [["kat_stethoscope"],       1, "Stethoscope", _missing] call FUNC(countItem);
        };
        case "engineer": {
            [["ACE_wirecutter"],        1, "Wirecutter", _missing] call FUNC(countItem);
            [["ACE_EntrenchingTool"],   1, "Entrenching Tool", _missing] call FUNC(countItem);
            [["ToolKit"],               1, "Toolkit", _missing] call FUNC(countItem);
            [["DemoCharge_Remote_Mag"], 2, "M112 Demo Block", _missing] call FUNC(countItem);
            [["ACE_rope3","ACE_rope6"], 2, "Rope", _missing] call FUNC(countItem);
        };
        case "eod": {
            [["ACE_Clacker", "ACE_M26_Clacker"],    1, "Clacker", _missing] call FUNC(countItem);
            [["ACE_DefusalKit"],                    1, "Defusal Kit", _missing] call FUNC(countItem);
            [["ACE_wirecutter"],                    1, "Wirecutter", _missing] call FUNC(countItem);
            [["DemoCharge_Remote_Mag"],             2, "M112 Demo Block", _missing] call FUNC(countItem);
            [["ace_marker_flags_green"],            5, "Marker Flag (Green)", _missing]  call FUNC(countItem);
            [["ace_marker_flags_red"],              5, "Marker Flag (Red)", _missing]   call FUNC(countItem);

            if !(handgunWeapon _unit in ["ACE_VMH3", "ACE_VMM3"]) then {
                _missing pushBackUnique "Mine Detector";
            };
        };
        case "signaller": {
            [["ACE_Chemlight_HiRed"],      3, "Chemlight (Hi Red)", _missing] call FUNC(countItem);
            [["ACE_Chemlight_HiGreen"],    3, "Chemlight (Hi Green)", _missing] call FUNC(countItem);
            [["ACE_Chemlight_HiYellow"],   6, "Chemlight (Hi Yellow)", _missing] call FUNC(countItem);
            [["ACE_HandFlare_Green"],      2, "Hand Flare (Green)", _missing] call FUNC(countItem);
            [["ACE_HandFlare_Red"],        2, "Hand Flare (Red)", _missing] call FUNC(countItem);
            [["ACE_HandFlare_Yellow"],     4, "Hand Flare (Yellow)", _missing] call FUNC(countItem);
            [["SmokeShellOrange"],         2, "Smoke Shell (Orange)", _missing] call FUNC(countItem);
        };
        case "ol";
        case "el";
        case "ftl": {
            [["ACRE_PRC152", "ACRE_PRC117F"], 1, "AN/PRC-152", _missing] call FUNC(countItem);
        };
        case "sl": {
            [["ACRE_PRC152", "ACRE_PRC117F"], 2, "AN/PRC-152", _missing] call FUNC(countItem);
        };
        default { };
    };
} forEach _roles;

_missing
