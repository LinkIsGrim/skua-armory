// Event-name macros for the unified skua:event channel.
// MUST match `Event::event_name()` in extension/src/event/mod.rs.
// SQF subscribers register handlers via `[QEV_*, ...] call CBA_fnc_addEventHandler`.
#define QEV_CERTIFICATION_GRANTED             "skua_certification_granted"
#define QEV_CERTIFICATION_REVOKED             "skua_certification_revoked"
#define QEV_CERTIFICATION_LIST_CHANGED        "skua_certification_list_changed"
#define QEV_RANK_CHANGED                      "skua_rank_changed"
#define QEV_RANK_LIST_CHANGED                 "skua_rank_list_changed"
#define QEV_PLAYER_CONNECTED                  "skua_player_connected"
#define QEV_PLAYER_DISCONNECTED               "skua_player_disconnected"

// Client-facing rebroadcasts of the canonical events above. Fired by the
// server's own handler of each canonical event via CBA_fnc_globalEvent
// (non-JIP), so hasInterface clients can react to live state changes without
// needing the extension to reach them directly.
#define QEV_CERTIFICATION_GRANTED_GLOBAL      "skua_certification_granted_global"
#define QEV_CERTIFICATION_REVOKED_GLOBAL      "skua_certification_revoked_global"
#define QEV_CERTIFICATION_LIST_CHANGED_GLOBAL "skua_certification_list_changed_global"
