#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Receive the full client addon map from the server, cache it in uiNamespace,
 * and notify the open admin menu so the Addons tab can redraw.
 *
 * Arguments:
 * 0: Client addon map <HASHMAP<UID, [extras, missing, extrasModMap]>>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params ["_map"];

uiNamespace setVariable [QGVAR(clientAddonMap), _map];

[QGVAR(addonMapLoaded)] call CBA_fnc_localEvent;
