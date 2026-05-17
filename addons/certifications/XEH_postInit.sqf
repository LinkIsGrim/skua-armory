#include "script_component.hpp"

if (!isServer) exitWith {};

["CBA_settingsInitialized", {
    GVAR(loaded) = false;
    GVAR(list) = []; // Array of Hashmaps containing cert data
    GVAR(map) = createHashMap; // Map of id to cert data for easy lookup
    addMissionEventHandler ["ExtensionCallback", LINKFUNC(extCallback)];
    private _result = call FUNC(updateCerts); // Load certs from database, will return via extCallback
    if (_result select 0 != "0") then {
        ERROR_1("Failed to fetch certifications from database: %1",_result);
    } else {
        INFO("Requested certification list from database successfully.");
    };
}] call CBA_fnc_addEventHandler;
