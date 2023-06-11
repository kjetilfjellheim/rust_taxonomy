use crate::taxonomy::connection;
use crate::taxonomy::dao::TaxonomicUnit;
use crate::taxonomy::dao::{
    taxonomic_units, taxonomic_units::dsl::taxonomic_units as taxonomic_units_dsl,
};
use crate::taxonomy::model::{ApplicationError, ErrorType, ListRequest, ListResponse};
use diesel::prelude::*;
use diesel::result::Error::*;
use log::{debug, warn};
use std::str::FromStr;

///
/// Error messages.
///
const QUERY_ERROR_STRING: &str = "Error querying taxonomic data";
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
/// Find all longnames using start_index and page_size.
///
pub fn find_all(
    list_request: ListRequest,
) -> Result<ListResponse<TaxonomicUnit>, ApplicationError> {
    validate_request(&list_request)?;
    let connection = &mut connection()?;
    let query_result = taxonomic_units_dsl
        .limit(list_request.number_of_elements + 1)
        .offset(list_request.start_index)
        .select((taxonomic_units::tsn, taxonomic_units::complete_name))
        .load(connection);
    match query_result {
        Ok(query_result) => Ok(ListResponse::new(
            list_request.start_index,
            list_request.number_of_elements,
            list_request.number_of_elements + 1,
            query_result,
        )),
        Err(error) => {
            warn!("Error occured quering taxonomy list: {}", error);
            Err(ApplicationError::new(
                ErrorType::DbProgramError,
                QUERY_ERROR_STRING.to_string(),
            ))
        }
    }
}

///
/// Find single longname row.
///
pub fn find_longname(tsn_query: &String) -> Result<TaxonomicUnit, ApplicationError> {
    let tsn_value = validate_tsn(tsn_query)?;
    let connection = &mut connection()?;
    let query_result = taxonomic_units_dsl
        .select((taxonomic_units::tsn, taxonomic_units::complete_name))
        .find(tsn_value)
        .first(connection);
    match query_result {
        Ok(longname) => Ok(longname),
        Err(NotFound) => {
            debug!("Did not find tsn {}", tsn_query);
            Err(ApplicationError::new(
                ErrorType::NotFoundError,
                LONGNAME_NOT_FOUND.to_string(),
            ))
        }
        Err(error) => {
            warn!("Error occured quering specific taxonomy: {}", &error);
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
