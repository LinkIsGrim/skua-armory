#include "..\script_component.hpp"

params ["_campaignKey"];

"skua" callExtension ["database:bootstrap", [_campaignKey]];
