use crate::taxonomy::model::common::TaxonomyListRequest;
use crate::taxonomy::model::errorcode::{ApplicationError, ErrorType};
use std::str::FromStr;

const LONGNAME_TSN_INCORRECT: &str = "Tsn input must be 32 bit integer";
const NUMBER_OF_ELEMENTS_MIN_CHECK: &str = "Number of elments must be greater than 0";
const NUMBER_OF_ELEMENTS_MAX_CHECK: &str = "Number of elements must be less than or euals to 500";

// Constants used for validation. This should be must to configuration.
const MAX_ELEMENTS: i64 = 500;
const MIN_ELEMENTS: i64 = 0;

// Validate list taxonomy input.
pub fn validate_list_tsn_request(
    list_request: &TaxonomyListRequest
) -> Result<(), ApplicationError> {
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

// Validate get taxonomy input.
pub fn validate_specific_tsn_request(tsn_str: &String) -> Result<i32, ApplicationError> {
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
