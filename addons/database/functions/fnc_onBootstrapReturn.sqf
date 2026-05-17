#include "..\script_component.hpp"
/*
 * Authors: LinkIsGrim
 * Handles extension callback for bootstrap. Updates database state, raises event.
 *
 * Arguments:
 * 0: Data returned from the extension <STRING>
 *
 * Return Value:
 * None.
 *
 * Example:
 * [params] call PREFIX_~/arma3/skua-armory/addons/database/functions_fnc_onBootstrapReturn
 *
 * Public: No
 */

params ["_data"];
TRACE_1("fnc_onBootstrapReturn",_this);

(parseSimpleArray _data) params ["_status", "_return"];

if (_status != QUERYSTATE_DONE) exitWith {
    ERROR_2("Database bootstrap failed: %1: %2",_status,_return);
};

GVAR(state) = DATABASESTATE_CONNECTEDAWAITINIT;

// Have the extension give us database state
("skua" callExtension ["database:state", []]) params ["_state"];

GVAR(state) = _state;
switch GVAR(state) do {
    case DATABASESTATE_CONNECTEDAWAITINIT: {
        // we're connected to the database but it hasn't finished initializing yet, wait for init callback
        INFO("Database connected, awaiting initialization...");
    };
    case DATABASESTATE_CONNECTEDINIT: {
        // database is fully initialized and ready to use; broadcast so deferred
        // code and other addons can proceed. JIP so handlers registered later
        // (e.g. addons whose preInit runs after bootstrap) still fire.
        INFO("Database connected and initialized.");
        [QGVAR(initialized), []] call CBA_fnc_globalEventJIP;
    };
    case DATABASESTATE_FAILED: {
        ERROR("Database connection failed, check previous log messages for details.");
    };
    default {
        ERROR_1("Received unknown database state: %1",GVAR(state));
    };
}
