//! Database-specific error types.

#[derive(Debug)]
pub enum DbError {
    /// Failed to initialize the database (pool construction / first connect).
    Init(tokio_postgres::Error),
    /// Failed to get a connection from the pool.
    Pool(deadpool_postgres::PoolError),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::Init(e) => write!(f, "database initialization failed: {e}"),
            DbError::Pool(e) => write!(f, "pool connection failed: {e}"),
        }
    }
}

impl std::error::Error for DbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DbError::Init(e) => Some(e),
            DbError::Pool(e) => Some(e),
        }
    }
}
