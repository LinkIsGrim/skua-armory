#include "script_component.hpp"

if (!isServer) exitWith {};
// Singleplayer (incl. editor preview) has no Steam UIDs (`getPlayerUID` returns
// "_SP_PLAYER_") and no persistence is meaningful — skip bootstrap entirely.
if (!isMultiplayer) exitWith {};

["CBA_settingsInitialized", {
    GVAR(campaignKey) call FUNC(bootstrap);
}] call CBA_fnc_addEventHandler;
