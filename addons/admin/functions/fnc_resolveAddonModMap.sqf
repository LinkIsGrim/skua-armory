#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Resolve a list of addon class names to their source mods on the executing
 * machine. Per-addon variant of fnc_getModNamesFromAddons — mirrors that
 * function's canonical pattern (build dir set from addons → filter loaded
 * mods info by dir) but returns a HashMap so callers can group by mod.
 *
 * Addons must exist locally — used by the client (sending its own addon list
 * to the server) and by the server (when resolving addons it has loaded).
 *
 * Arguments:
 * 0: List of addons <ARRAY of STRING>
 *
 * Return Value:
 * HASHMAP<addonClass, [modDir (lowercase), modName]>. Addons whose source
 * mod isn't in getLoadedModsInfo get ["", ""].
 *
 * Example:
 * ["ace_medical", "cba_jr"] call skua_admin_fnc_resolveAddonModMap;
 *
 * Public: No
 */

private _allLoadedModsInfo = getLoadedModsInfo;

private _cfgPatches = configFile >> "CfgPatches";

// Mirror fnc_getModNamesFromAddons: build the set of source mod dirs from the
// input addons first, then filter the loaded-mods info down to that set.
private _sourceModDirs = createHashMap;
{
    _sourceModDirs set [(toLowerANSI configSourceMod (_cfgPatches >> _x)), nil];
} forEach _this;

private _addonsLoadedModsInfo = _allLoadedModsInfo select {(toLowerANSI (_x select 1)) in _sourceModDirs};

// Build a dir → name lookup for the resolved subset, then assign each addon
// its [dir, name] tuple. Unresolved addons (dir not in loaded mods info) get
// ["", ""].
private _dirToName = createHashMap;
{
    _dirToName set [toLowerANSI (_x select 1), _x select 0];
} forEach _addonsLoadedModsInfo;

private _modMap = createHashMap;
{
    private _dir = toLowerANSI configSourceMod (_cfgPatches >> _x);
    private _name = _dirToName getOrDefault [_dir, ""];
    _modMap set [_x, [_dir, _name]];
} forEach _this;

_modMap
