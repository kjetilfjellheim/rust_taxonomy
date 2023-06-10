use diesel::{ prelude::*, r2d2::ConnectionManager };
use r2d2::PooledConnection;

use crate::taxonomy::model::{ErrorType, ApplicationError, Longname, ListRequest, ListResponse };
use crate::taxonomy::model::longnames;

const QUERY_ERROR_STRING: &str = "Error querying longnames table";

fn get_connection() -> Result<PooledConnection<ConnectionManager<PgConnection>>, ApplicationError> {
    match crate::taxonomy::connection() {
        Ok(conn) => Ok(conn),
        Err(application_error) => Err(application_error)
    }
}

pub fn find_all(list_request: ListRequest) -> Result<ListResponse<Longname>, ApplicationError> {
    let connection = &mut get_connection()?;
    let query_result = longnames.limit(list_request.number_of_elements + 1).offset(list_request.start_index).select(Longname::as_select()).load(connection);
    match query_result {
        Ok(query_result) => Ok(ListResponse::new(list_request.start_index, list_request.number_of_elements, list_request.number_of_elements + 1, query_result)),
        Err(_application_error) => Err(ApplicationError::new(ErrorType::DbProgramError, QUERY_ERROR_STRING.to_string()))
    }
}
