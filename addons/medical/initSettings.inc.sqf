[
    QGVAR(preventInstantDeath), "CHECKBOX",
    ["Prevent Instant Death", "Simulate pre-rewrite ""Prevent Instant Death"" logic."],
    ["Skua Mods", "Medical Options"],
    false,
    1
] call CBA_fnc_addSetting;

[
    QACEGVAR(medical,_const_stableVitalsBloodThreshold), "SLIDER",
    ["Blood Volume Wake-Up Threshold", "Blood volume needs to be above this amount, in liters, to allow units to wake up."],
    ["Skua Mods", "Medical Options"],
    [3.0, 6.0, 5.1, 1],
    1
] call CBA_fnc_addSetting;

[
    QACEGVAR(medical,_const_bloodLossKnockOutThreshold), "SLIDER",
    ["Blood Loss KO Threshold", "Blood Loss KO Threshold. Higher makes it easier to wake up. 2 ignores Blood Loss in Stable Vitals calculation. 0 makes bleeding units unable to wake up."],
    ["Skua Mods", "Medical Options"],
    [0, 2, 0.5, 1],
    1
] call CBA_fnc_addSetting;

[
    QACEGVAR(medical,_const_wakeUpCheckInterval), "SLIDER",
    ["Unconscious Wake-Up Interval", "Base time for calculation of unconscious state stable vitals check. Lower means it's ran more often and thus units will wake up sooner on average."],
    ["Skua Mods", "Medical Options"],
    [0, 30, 15, 0],
    1
] call CBA_fnc_addSetting;