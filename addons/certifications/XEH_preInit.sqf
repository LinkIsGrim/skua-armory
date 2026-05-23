#include "script_component.hpp"

ADDON = false;

PREP_RECOMPILE_START;
#include "XEH_PREP.hpp"
PREP_RECOMPILE_END;

#include "events.hpp"

// Cert list state needs to exist on both server and clients: the server gets
// it from the extension's `skua:event` channel, clients get it from the
// server's `_GLOBAL` rebroadcast. UIs (e.g. the admin cert menu) read GVAR(list)
// and GVAR(map) directly.
GVAR(loaded) = false;
GVAR(list) = []; // Array of Hashmaps containing cert data
GVAR(map) = createHashMap; // Map of id to cert data for easy lookup

if (isServer) then {
    // Grant/revoke callbacks that arrive before the static cert list has
    // loaded are queued here as `[event_type, _playerID, _certID]` triples;
    // fnc_onCertificationListChanged flushes them once GVAR(map) is populated.
    // The extension fires per-player grants as part of database:player_connect
    // and the watchdog may emit grants/revokes between bootstrap and the
    // cert-list push landing.
    GVAR(pendingCertEvents) = [];

    [QEV_CERTIFICATION_GRANTED, LINKFUNC(onCertificationGranted)] call CBA_fnc_addEventHandler;
    [QEV_CERTIFICATION_REVOKED, LINKFUNC(onCertificationRevoked)] call CBA_fnc_addEventHandler;
    [QEV_CERTIFICATION_LIST_CHANGED, LINKFUNC(onCertificationListChanged)] call CBA_fnc_addEventHandler;

    // Server re-broadcasts the canonical cert events as `_GLOBAL` variants so
    // hasInterface clients (admin UI etc.) can react without needing the
    // extension's local-only event stream. Grant/revoke are non-JIP — clients
    // get held certs via player setVariable broadcast; admin UI cache mutations
    // for missed events are harmless since the menu force-refreshes on open.
    // List-changed IS JIP so late joiners hydrate their GVAR(list)/GVAR(map).
    [QEV_CERTIFICATION_GRANTED, {
        [QEV_CERTIFICATION_GRANTED_GLOBAL, _this] call CBA_fnc_globalEvent;
    }] call CBA_fnc_addEventHandler;
    [QEV_CERTIFICATION_REVOKED, {
        [QEV_CERTIFICATION_REVOKED_GLOBAL, _this] call CBA_fnc_globalEvent;
    }] call CBA_fnc_addEventHandler;
    [QEV_CERTIFICATION_LIST_CHANGED, {
        [QEV_CERTIFICATION_LIST_CHANGED_GLOBAL, _this, QGVAR(certListJip)] call CBA_fnc_globalEventJIP;
    }] call CBA_fnc_addEventHandler;
};

if (!isServer) then {
    // Mirror the server's static cert list onto clients so UIs reading
    // GVAR(list)/GVAR(map) (admin menu etc.) work without round-tripping to
    // the server. We deliberately skip the pending-events flush logic; the
    // per-player grant/revoke events fire on the server and update player
    // setVariables broadcast-globally — clients don't need the queue.
    [QEV_CERTIFICATION_LIST_CHANGED_GLOBAL, {
        params ["_certs"];
        GVAR(list) = _certs;
        GVAR(map) = createHashMap;
        {
            GVAR(map) set [_x get "id", _x];
        } forEach GVAR(list);
        GVAR(loaded) = true;
    }] call CBA_fnc_addEventHandler;
};

ADDON = true;
