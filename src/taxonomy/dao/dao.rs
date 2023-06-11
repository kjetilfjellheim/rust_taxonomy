use std::str::FromStr;

use diesel::{prelude::*, r2d2::ConnectionManager};
use diesel::result::Error::*;
use r2d2::PooledConnection;
use crate::taxonomy::dao::{ Longname };
use crate::taxonomy::dao::longnames::dsl::longnames;
use crate::taxonomy::model::{ ApplicationError, ErrorType, ListRequest, ListResponse };

///
/// Error messages.
///
const QUERY_ERROR_STRING: &str = "Error querying longnames table";
const NUMBER_OF_ELEMENTS_MIN_CHECK: &str = "Number of elments must be greater than 0";
const NUMBER_OF_ELEMENTS_MAX_CHECK: &str = "Number of elements must be less than or euals to 500";
const LONGNAME_NOT_FOUND: &str = "Did not find that tsn number";
const LONGNAME_TSN_INCORRECT: &str = "Tsn input must be 32 bit integer";

///
/// Constants used for validation. This should be must to configuration.
///
const MAX_ELEMENTS: i64 = 500;
const MIN_ELEMENTS: i64 = 0;

///
/// get connection from connection pool.
///
fn get_connection() -> Result<PooledConnection<ConnectionManager<PgConnection>>, ApplicationError> {
    match crate::taxonomy::connection() {
        Ok(conn) => Ok(conn),
        Err(application_error) => Err(application_error),
    }
}

///
/// Find all longnames using start_index and page_size.
///
pub fn find_all(list_request: ListRequest) -> Result<ListResponse<Longname>, ApplicationError> {
    validate_request(&list_request)?;
    let connection = &mut get_connection()?;
    let query_result = longnames
        .limit(list_request.number_of_elements + 1)
        .offset(list_request.start_index)
        .select(Longname::as_select())
        .load(connection);
    match query_result {
        Ok(query_result) => Ok(ListResponse::new(
            list_request.start_index,
            list_request.number_of_elements,
            list_request.number_of_elements + 1,
            query_result,
        )),
        Err(_error) => Err(ApplicationError::new(
            ErrorType::DbProgramError,
            QUERY_ERROR_STRING.to_string(),
        )),
    }
}

///
/// Find single longname row.
///
pub fn find_longname(tsn_query: &String) -> Result<Longname, ApplicationError> {
    let tsn_value = validate_tsn(tsn_query)?;
    let connection = &mut get_connection()?;
    let query_result = longnames.select(Longname::as_select()).find(tsn_value).first(connection);
    match query_result {
        Ok(longname) => Ok(longname),
        Err(NotFound) => {
            Err(ApplicationError::new(
                ErrorType::NotFoundError,
                LONGNAME_NOT_FOUND.to_string(),
            ))
        },
        Err(_) => {
            Err(ApplicationError::new(
                ErrorType::DbProgramError,
                QUERY_ERROR_STRING.to_string(),
            ))
        }
    }
}


///
/// Validate input. Move this to common validation service.
///
fn validate_request(list_request: &ListRequest) -> Result<(), ApplicationError> {
    match MAX_ELEMENTS.cmp(&list_request.number_of_elements) {
        std::cmp::Ordering::Less => {
            return Err(ApplicationError::new(
                ErrorType::InputError,
                NUMBER_OF_ELEMENTS_MAX_CHECK.to_string(),
            ));
        }
        std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {}
    }
    match MIN_ELEMENTS.cmp(&list_request.number_of_elements) {
        std::cmp::Ordering::Greater => {
            return Err(ApplicationError::new(
                ErrorType::InputError,
                NUMBER_OF_ELEMENTS_MIN_CHECK.to_string(),
            ));
        }
        std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {}
    }
    Ok(())
}

///
/// Validate input. Move this to common validation service.
///
fn validate_tsn(tsn_str: &String) -> Result<i32, ApplicationError> {
    match <i32 as FromStr>::from_str(&tsn_str) {
        Ok(val) => Ok(val),
        Err(_) => {
            return Err(ApplicationError::new(
                ErrorType::InputError,
                LONGNAME_TSN_INCORRECT.to_string(),
            ));
        }
    }
}
