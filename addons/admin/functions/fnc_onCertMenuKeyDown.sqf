#include "..\script_component.hpp"
#include "..\ui\defines.hpp"
#include "\a3\ui_f\hpp\defineDIKCodes.inc"
/*
 * Author: LinkIsGrim
 * Display-level KeyDown handler for the admin cert menu. Handles Ctrl-C:
 * copies the currently-selected player's Steam UID to the system clipboard
 * (lbData on each player row is the UID). Returns true to consume the event
 * so it doesn't propagate to other handlers. Other keys are passed through.
 *
 * Modeled on ACE arsenal's fnc_onKeyDown — same display-EH signature.
 *
 * Arguments:
 * 0: Display <DISPLAY>
 * 1: Key code <NUMBER>
 * 2: Shift state <BOOL>
 * 3: Ctrl state <BOOL>
 * 4: Alt state <BOOL>
 *
 * Return Value:
 * Handled <BOOL> (true to consume the keystroke)
 *
 * Public: No
 */

params ["_display", "_key", "", "_ctrl", ""];

if !(_key == DIK_C && _ctrl) exitWith {false};

private _playerList = _display displayCtrl IDC_ADMINCERT_PLAYER_LIST;
private _sel = lbCurSel _playerList;
if (_sel < 0) exitWith {false};

private _uid = _playerList lbData _sel;
if (_uid isEqualTo "") exitWith {false};

copyToClipboard _uid;
systemChat format ["Copied Steam ID to clipboard: %1", _uid];
TRACE_1("copied uid to clipboard",_uid);
true
