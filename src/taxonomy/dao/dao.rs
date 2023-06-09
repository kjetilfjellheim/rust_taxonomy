use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use serde::Serialize;

use crate::taxonomy::dao::schema::longnames::dsl::*;
use crate::taxonomy::model::{ErrorType, ApplicationError };

const QUERY_ERROR_STRING: &str = "Error querying longnames table";

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::taxonomy::dao::schema::longnames)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Longname {
    pub tsn: i32,
    pub completename: String
}

fn get_connection() -> Result<PooledConnection<ConnectionManager<PgConnection>>, ApplicationError> {
    match crate::taxonomy::connection() {
        Ok(conn) => Ok(conn),
        Err(application_error) => Err(application_error)
    }
}

pub fn find_all() -> Result<Vec<Longname>, ApplicationError> {
    let connection = &mut get_connection()?;
    let query_result = longnames.limit(100).offset(1).select(Longname::as_select()).load(connection);
    match query_result {
        Ok(query_result) => Ok(query_result),
        Err(_application_error) => Err(ApplicationError::new(ErrorType::DbProgramError, QUERY_ERROR_STRING.to_string()))
    }
}
