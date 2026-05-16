# Database

A simple `compose.yaml` (for use with Docker or Podman) to spin up a Postgres
instance for local development.

The integration tests under `extension/src/database/tests.rs` do not need this —
they manage their own containers via `testcontainers`. This compose file is for
ad-hoc local testing where you want to point the running extension (or `psql`)
at a stable database.

```sh
docker compose -f database/compose.yaml up -d
# DATABASE_HOST=127.0.0.1 DATABASE_PORT=55432 DATABASE_USER=postgres DATABASE_PASSWORD=changeit
```

SQL files used by the bootstrap live in `extension/src/database/sql/`.
