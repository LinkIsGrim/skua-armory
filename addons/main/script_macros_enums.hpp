// parseNumber is slower than comparing the string directly, so we'll just deal with it
// these MUST match the Rust extension's DatabaseState enum (see extension/src/database/state.rs)
#define DATABASESTATE_AWAITCONNECT       ("0")
#define DATABASESTATE_CONNECTEDINIT      ("1")
#define DATABASESTATE_CONNECTEDAWAITINIT ("2")
#define DATABASESTATE_FAILED             ("3")
