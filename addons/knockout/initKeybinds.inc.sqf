["Skua Mods", QUOTE(ADDON), "Punch", {
    if !([ACE_player] call FUNC(canPunch)) exitWith {false};
    [ACE_player] call FUNC(punchTarget)
}, {}, [199, [false,false,false]]] call CBA_fnc_addKeybind;
