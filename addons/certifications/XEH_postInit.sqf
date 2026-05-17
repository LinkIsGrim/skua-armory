#include "script_component.hpp"

if (!isServer) exitWith {};

// Wait until the database is bootstrapped before fetching the cert list. The
// database addon broadcasts QGVAR(initialized) via globalEventJIP after a
// successful bootstrap, so this handler fires once at most.
[QEGVAR(database,initialized), {
    (call FUNC(updateCerts)) params ["_result"];
    if (_result isNotEqualTo QUERYSTATE_PROCESSING) then {
        ERROR_1("Failed to fetch certifications from database: %1",_result);
    } else {
        INFO("Requested certification list from database successfully.");
    };
}] call CBA_fnc_addEventHandler;
