#include "..\script_component.hpp"
/*
 * Authors: Geddie, LinkIsGrim
 * Display welcome message to player joining server
 *
 * Arguments:
 * None
 *
 * Return Value:
 * None
 *
 * Example:
 * call skua_missions_fnc_missionHint;
 *
 * Public: No
 */

[{ 
    private _ts = parseText "<a href='ts3server://skua.international'>Teamspeak: skua.international (Click me!)</a>";
    private _discord = parseText "<a href='https://discord.gg/z6qEYBTqKn'>Join our Discord (Click me!)</a>";
    private _arsenal = parseText "You can access the Arsenal by self-interacting (Ctrl+Windows)";

    "Welcome to Skua International" hintC [_ts, _discord, _arsenal];
 }, nil, 1] call CBA_fnc_waitAndExecute;
