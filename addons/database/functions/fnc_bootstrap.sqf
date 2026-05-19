#include "..\script_component.hpp"

params ["_campaignKey"];

("skua" callExtension ["database:bootstrap", [_campaignKey]]) params ["_return", "_error", "_details"];
if (_error != 0) then {
    ERROR_2("Database bootstrap failed with error code %1. Campaign key: %2",_error,_campaignKey);
} else {
    INFO_1("Database bootstrap successful. Campaign key: %1",_campaignKey);
};

[_return, _error, _details]
