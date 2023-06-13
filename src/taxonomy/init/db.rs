use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use log::error;
use r2d2::{self, PooledConnection};
use std::env;
use std::time::Duration;

use crate::taxonomy::model::{ApplicationError, ErrorType};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

const DB_URL_PROP: &str = "DB_URL";
const DB_MIN_CONNECTION_POOL_PROP: &str = "DB_MIN_CONNECTION_POOL";
const DB_MAX_CONNECTION_POOL_PROP: &str = "DB_MAX_CONNECTION_POOL";
const DB_CONNECTION_TIMEOUT: &str = "DB_CONNECTION_TIMEOUT";
const DB_MAX_LIFETIME: &str = "DB_MAX_LIFETIME";

const DEFAULT_MAX_CONNECTION_POOL: &str = "20";
const DEFAULT_MIN_CONNECTION_POOL: &str = "1";
const DEFAULT_CONNECTION_TIMEOUT: &str = "3";
const DEFAULT_MAX_LIFETIME: &str = "30";

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var(DB_URL_PROP).expect("Database url not set");
        let min_conn_pool = env::var(DB_MIN_CONNECTION_POOL_PROP)
            .unwrap_or(DEFAULT_MIN_CONNECTION_POOL.to_string())
            .parse::<u32>()
            .unwrap();
        let max_conn_pool = env::var(DB_MAX_CONNECTION_POOL_PROP)
            .unwrap_or(DEFAULT_MAX_CONNECTION_POOL.to_string())
            .parse::<u32>()
            .unwrap();
        let connection_timeout = Duration::from_secs(
            env::var(DB_CONNECTION_TIMEOUT)
                .unwrap_or(DEFAULT_CONNECTION_TIMEOUT.to_string())
                .parse::<u64>()
                .unwrap(),
        );
        let max_lifetime = Duration::from_secs(
            env::var(DB_MAX_LIFETIME)
                .unwrap_or(DEFAULT_MAX_LIFETIME.to_string())
                .parse::<u64>()
                .unwrap(),
        );

        let manager = ConnectionManager::<PgConnection>::new(db_url);
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
/// Function used to start db connection initialization.
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

fn map_connection_error(error: r2d2::Error) -> ApplicationError {
    error!("Failed to get connection: {}", error);
    ApplicationError::new(ErrorType::ConnectionError, error.to_string())
}
