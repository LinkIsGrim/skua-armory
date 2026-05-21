// Pause menu hook for the Admin Menu. CBA_fnc_addPauseMenuOption (called
// from XEH_postInit when the local player is an admin) registers an entry
// in the ESC menu "Options" tab pointing at this display class. The display
// is a one-shot helper — it opens the admin menu in its onLoad, then closes
// itself so the pause menu unblocks the input.
//
// Pattern lifted from ACE's optionsmenu addon (see ace3/addons/optionsmenu/
// gui/pauseMenu.hpp — the same trick that wires up "ACE Headbug Fix").

#pragma hemtt suppress pw3_padded_arg file

class RscDisplayEmpty;
class GVAR(PauseMenuHelperAdminMenu): RscDisplayEmpty {
    // Close the pause menu (IDD 49 covers both SP RscDisplayInterrupt and MP
    // RscDisplayMPInterrupt) AND this helper, then open the admin menu on
    // the next frame — createDialog races with mid-frame display teardown
    // otherwise and the menus end up stacked.
    onLoad = QUOTE(\
        (findDisplay 49) closeDisplay 0;\
        (_this select 0) closeDisplay 0;\
        [{call FUNC(openAdminMenu)}] call CBA_fnc_execNextFrame;\
    );
};
