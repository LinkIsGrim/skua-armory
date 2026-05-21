#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Server-only handler for QGVAR(fetchOfflineCertsRequest). Fires the
 * `certification:get_player` extension call; the result lands on
 * `skua:certification` in fnc_extCallback which then targets the parsed cert
 * list at admin clients only via QGVAR(offlineCertsPushed).
 *
 * Arguments:
 * 0: Grantee Steam UID <STRING>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params [["_granteeUID", "", [""]]];
if (_granteeUID isEqualTo "") exitWith {};

"skua" callExtension ["certification:get_player", [_granteeUID]];
