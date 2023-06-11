use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2::{self, PooledConnection};
use std::env;

use crate::taxonomy::model::{ApplicationError, ErrorType};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        r2d2::Pool::builder().max_size(15).build(manager).unwrap()
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
    POOL.get()
        .map_err(|e| ApplicationError::new(ErrorType::ConnectionError, e.to_string()))
}
