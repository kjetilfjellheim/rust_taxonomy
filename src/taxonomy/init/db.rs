use crate::taxonomy::model::{ApplicationError, ErrorType};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use log::error;
use r2d2::{self, PooledConnection};
use std::env;
use std::time::Duration;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Property name in .env used for DB connection.
const DB_URL_PROP: &str = "DB_URL";
/// Property name in .env used to define min number of connection.
const DB_MIN_CONNECTION_POOL_PROP: &str = "DB_MIN_CONNECTION_POOL";
/// Property name in .env used to define max number of connection.
const DB_MAX_CONNECTION_POOL_PROP: &str = "DB_MAX_CONNECTION_POOL";
/// Property name in .env used to define connection timeout.
const DB_CONNECTION_TIMEOUT: &str = "DB_CONNECTION_TIMEOUT";
/// Property name in .env used to define max lifetime of connection.
const DB_MAX_LIFETIME: &str = "DB_MAX_LIFETIME";

/// If max connection is not defined in .env use this value.
const DEFAULT_MAX_CONNECTION_POOL: &str = "20";
/// If min connection is not defined in .env use this value.
const DEFAULT_MIN_CONNECTION_POOL: &str = "1";
/// If connection timeout is not defined  in .env use this value.
const DEFAULT_CONNECTION_TIMEOUT: &str = "3";
/// If max lifetime is not defined  in .env use this value.
const DEFAULT_MAX_LIFETIME: &str = "30";

lazy_static! {

    // Initialize connection pool
    static ref POOL: Pool = {
        // Get database url from environment.
        let db_url = env::var(DB_URL_PROP).expect("Database url not set");
        // Get minimum connections.
        let min_conn_pool = env::var(DB_MIN_CONNECTION_POOL_PROP)
            .unwrap_or(DEFAULT_MIN_CONNECTION_POOL.to_string())
            .parse::<u32>()
            .unwrap();
        // Get max connections.
        let max_conn_pool = env::var(DB_MAX_CONNECTION_POOL_PROP)
            .unwrap_or(DEFAULT_MAX_CONNECTION_POOL.to_string())
            .parse::<u32>()
            .unwrap();
        // Get connection timeout.
        let connection_timeout = Duration::from_secs(
            env::var(DB_CONNECTION_TIMEOUT)
                .unwrap_or(DEFAULT_CONNECTION_TIMEOUT.to_string())
                .parse::<u64>()
                .unwrap(),
        );
        // Get max time a connection stays alive.
        let max_lifetime = Duration::from_secs(
            env::var(DB_MAX_LIFETIME)
                .unwrap_or(DEFAULT_MAX_LIFETIME.to_string())
                .parse::<u64>()
                .unwrap(),
        );
        // Inialize connection.
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        // Initialize connection pool.
        r2d2::Pool::builder()
            .min_idle(Some(min_conn_pool))
            .max_size(max_conn_pool)
            .connection_timeout(connection_timeout)
            .max_lifetime(Some(max_lifetime))
            .build(manager)
            .unwrap()
    };
}

///
/// Initialize database connection.
///
pub fn init_db() {
    lazy_static::initialize(&POOL);

    let _conn: PooledConnection<ConnectionManager<PgConnection>> =
        connection().expect("Failed to get db connection");
}

///
/// Get connection from connection pool
///
pub fn connection() -> Result<PooledConnection<ConnectionManager<PgConnection>>, ApplicationError> {
    POOL.get().map_err(map_connection_error)
}
///
/// If error occurs during get connection.
///
fn map_connection_error(error: r2d2::Error) -> ApplicationError {
    error!("Failed to get connection: {}", error);
    ApplicationError::new(ErrorType::ConnectionError, error.to_string())
}
///
/// Get connection pool state
///
pub fn get_connection_pool_status() -> (u32, u32) {
    (POOL.state().connections, POOL.state().idle_connections)
}
