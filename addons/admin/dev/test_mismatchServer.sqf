// call compileScript ["\z\skua\addons\admin\dev\test_mismatchServer.sqf"]
//
// Singleplayer dev helper. Injects a fake mismatch into the admin addon-map
// for the local player so the Admin Menu's Addon List tab has something to
// render. Run from the debug console.
//
// Layout (all values are made up):
//   missing: two real classnames so they resolve to actual mod display names.
//   extras : two fake addon classes with fake [dir, name] entries — the
//            server can't resolve these locally (it doesn't have them), the
//            modMap is what the client would have shipped over the wire.
//
// Idempotent: re-running overwrites the entry for the local UID.

if (!isServer) exitWith {
    diag_log "[skua_admin] test_mismatchServer must be run on the server";
};

if (isNil "skua_admin_clientAddonMap") exitWith {
    diag_log "[skua_admin] GVAR(clientAddonMap) not initialized yet — wait for postInit";
};

private _uid = getPlayerUID player;

// "Missing" addons: pick classnames that actually exist locally so the
// server-side resolver returns real mod names. ace_main and cba_main are
// safe bets in this stack.
private _missing = ["ace_main", "cba_main"];

// "Extra" addons: synthetic classes the server doesn't have. The modMap
// mirrors what fnc_sendClientAddons would have produced on the client.
private _extras = ["fake_super_mod_core", "fake_super_mod_weapons", "lone_orphan_addon"];

private _extrasModMap = createHashMap;
_extrasModMap set ["fake_super_mod_core",    ["@fake_super_mod", "Fake Super Mod"]];
_extrasModMap set ["fake_super_mod_weapons", ["@fake_super_mod", "Fake Super Mod"]];
// Orphan: empty dir + name → ends up in the Extra Addons (unresolved) column.
_extrasModMap set ["lone_orphan_addon",      ["", ""]];

skua_admin_clientAddonMap set [_uid, [_extras, _missing, _extrasModMap]];

diag_log format ["[skua_admin] Injected fake mismatch for UID %1: extras=%2 missing=%3", _uid, _extras, _missing];

// Invalidate any cached client copy so reopening the menu re-fetches.
uiNamespace setVariable ["skua_admin_clientAddonMap", nil];
