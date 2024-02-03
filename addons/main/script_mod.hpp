// COMPONENT should be defined in the script_component.hpp and included BEFORE this hpp

#define MAINPREFIX z
#define PREFIX skua

// TODO: Consider Mod-wide or Component-narrow versions  (or both, depending on wishes!)
// We will use the DATE for the BUILD# in the format YYMMDD - VM
#include "script_version.hpp"

#define VERSION     MAJOR.MINOR
#define VERSION_STR MAJOR.MINOR.PATCHLVL.BUILD
#define VERSION_AR  MAJOR,MINOR,PATCHLVL,BUILD

// MINIMAL required version for the Mod. Components can specify others..
#define REQUIRED_VERSION 2.14
#define REQUIRED_CBA_VERSION {3,16,1}
#define REQUIRED_ACE_VERSION {3,16,1}

#ifdef COMPONENT_BEAUTIFIED
    #define COMPONENT_NAME QUOTE(Skua - COMPONENT_BEAUTIFIED)
#else
    #define COMPONENT_NAME QUOTE(Skua - COMPONENT)
#endif
