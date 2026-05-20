#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
/*
 * Author: LinkIsGrim
 * Opens the admin cert management dialog. Caller is expected to be an admin
 * (the ACE self-action's condition gates this). Bails on dedicated/headless
 * (no display).
 *
 * Arguments:
 * None.
 *
 * Return Value:
 * Dialog created <BOOL>
 *
 * Public: Yes
 */

if (!hasInterface) exitWith {false};

if (!isNull (findDisplay IDD_ADMIN_CERT_MENU)) exitWith {
    INFO("Admin cert menu already open");
    true
};

createDialog "skua_admin_AdminCertMenu"
