#include "..\script_component.hpp"
/*
 * Author: LinkIsGrim
 * Client-side: asks the server for a (typically offline) player's persistent
 * cert ids. Online players' certs ride on setVariable broadcasts already —
 * this is for the historical-roster path where the unit isn't present.
 * Result lands via QGVAR(offlineCertsPushed) → fnc_onCertificationGetPlayerCallback.
 *
 * Arguments:
 * 0: Player Steam UID <STRING>
 *
 * Return Value:
 * None.
 *
 * Public: No
 */

params [["_granteeUID", "", [""]]];

if (!hasInterface || {_granteeUID isEqualTo ""}) exitWith {};
TRACE_1("requesting offline certs",_granteeUID);
[QGVAR(fetchOfflineCertsRequest), [_granteeUID]] call CBA_fnc_serverEvent;
