#define COMPONENT arsenal
#define COMPONENT_BEAUTIFIED Arsenal
#include "\z\skua\addons\main\script_mod.hpp"

// #define DEBUG_MODE_FULL
// #define DISABLE_COMPILE_CACHE
// #define ENABLE_PERFORMANCE_COUNTERS

#ifdef DEBUG_ENABLED_MAIN
    #define DEBUG_MODE_FULL
#endif

#ifdef DEBUG_SETTINGS_MAIN
    #define DEBUG_SETTINGS DEBUG_SETTINGS_MAIN
#endif

#include "\z\skua\addons\main\script_macros.hpp"

#define PIXEL_SCALE 0.25
#define GRID_W (pixelW * pixelGridNoUIScale * PIXEL_SCALE)
#define GRID_H (pixelH * pixelGridNoUIScale * PIXEL_SCALE)

// Deliberately the same as Synixe
#define IDC_requiredGearBox 24835
#define IDC_requiredGearTitle 24836
#define IDC_requiredGearText 24837

#define CUSTOM_BOXES [IDC_requiredGearBox, IDC_requiredGearTitle]