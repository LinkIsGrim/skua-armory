// parseNumber is slower than comparing the string directly, so we'll just deal with it
// these MUST match the Rust extension's DatabaseState enum (see extension/src/database/state.rs)
#define DATABASESTATE_AWAITCONNECT       ("0")
#define DATABASESTATE_CONNECTEDINIT      ("1")
#define DATABASESTATE_CONNECTEDAWAITINIT ("2")
#define DATABASESTATE_FAILED             ("3")

// QueryState appears in callback payloads as a bare number (after parseSimpleArray),
// so these macros are scalars — match with `_state isEqualTo QUERYSTATE_DONE`.
// these MUST match the Rust extension's QueryState enum (see extension/src/error/query.rs)
#define QUERYSTATE_PROCESSING       0
#define QUERYSTATE_DONE             1
#define QUERYSTATE_INVALIDARGUMENT  2
#define QUERYSTATE_TRANSIENTFAILURE 3
#define QUERYSTATE_TIMEOUT          4
