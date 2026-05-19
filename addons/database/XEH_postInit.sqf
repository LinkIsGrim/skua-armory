#include "script_component.hpp"

if (!isServer) exitWith {};
// Singleplayer (incl. editor preview) has no Steam UIDs (`getPlayerUID` returns
// "_SP_PLAYER_") and no persistence is meaningful — skip bootstrap entirely.
if (!isMultiplayer) exitWith {};

["CBA_settingsInitialized", {
    INFO("Database post-init event handler executing");
    GVAR(campaignKey) call FUNC(bootstrap);
}] call CBA_fnc_addEventHandler;

// Bootstrap watchdog: there's no timeout on the Rust side, so a wedged
// connection would silently leave clients waiting forever. If we haven't
// reached CONNECTEDINIT within 60s, surface it everywhere via globalEventJIP.
[{
    if (GVAR(state) != DATABASESTATE_CONNECTEDINIT) then {
        [QGVAR(bootstrapTimeout), [GVAR(state)]] call CBA_fnc_globalEventJIP;
    };
}, [], 60] call CBA_fnc_waitAndExecute;
