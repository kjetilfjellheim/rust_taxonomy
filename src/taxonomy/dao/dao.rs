use diesel::{ prelude::*, r2d2::ConnectionManager };
use r2d2::PooledConnection;

use crate::taxonomy::model::{ErrorType, ApplicationError, Longname, ListRequest, ListResponse };
use crate::taxonomy::model::longnames;

const QUERY_ERROR_STRING: &str = "Error querying longnames table";
const NUMBER_OF_ELEMENTS_MIN_CHECK: &str = "Number of elments must be greater than 0";
const NUMBER_OF_ELEMENTS_MAX_CHECK: &str = "Number of elements must be less than or euals to 500";

const MAX_ELEMENTS: i64 = 500;
const MIN_ELEMENTS: i64 = 0;

fn get_connection() -> Result<PooledConnection<ConnectionManager<PgConnection>>, ApplicationError> {
    match crate::taxonomy::connection() {
        Ok(conn) => Ok(conn),
        Err(application_error) => Err(application_error)
    }
}

pub fn find_all(list_request: ListRequest) -> Result<ListResponse<Longname>, ApplicationError> {
    match validate_request(&list_request) {
        Ok(()) => {},
        Err(application_error) => return Err(application_error)
    }
    let connection = &mut get_connection()?;
    let query_result = longnames.limit(list_request.number_of_elements + 1).offset(list_request.start_index).select(Longname::as_select()).load(connection);
    match query_result {
        Ok(query_result) => Ok(ListResponse::new(list_request.start_index, list_request.number_of_elements, list_request.number_of_elements + 1, query_result)),
        Err(_error) => Err(ApplicationError::new(ErrorType::DbProgramError, QUERY_ERROR_STRING.to_string()))
    }
}

fn validate_request(list_request: &ListRequest) -> Result<(), ApplicationError> {
    match MAX_ELEMENTS.cmp(&list_request.number_of_elements) {
        std::cmp::Ordering::Less => { return Err(ApplicationError::new(ErrorType::InputError, NUMBER_OF_ELEMENTS_MAX_CHECK.to_string())); },
        std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {},
    }
    match MIN_ELEMENTS.cmp(&list_request.number_of_elements) {
        std::cmp::Ordering::Greater => { return Err(ApplicationError::new(ErrorType::InputError, NUMBER_OF_ELEMENTS_MIN_CHECK.to_string())); },
        std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {},
    }
    Ok(())
}
