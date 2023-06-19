/// Validation model
use crate::taxonomy::model::common::TaxonomyListRequest;
use crate::taxonomy::model::errorcode::{ApplicationError, ErrorType};
use std::str::FromStr;

/// Error text if tsn is incorrect.
const LONGNAME_TSN_INCORRECT: &str = "Tsn input must be 32 bit integer";
/// Error text if number of elements is less than 0.
const NUMBER_OF_ELEMENTS_MIN_CHECK: &str = "Number of elments must be greater than 0";
/// Error text if number of elements is greater than 500.
const NUMBER_OF_ELEMENTS_MAX_CHECK: &str = "Number of elements must be less than or euals to 500";
/// Error text if start index to to large.
const START_INDEX_TOO_HIGH: &str = "Start index to large, please lower start index or increase filtering";
/// Max number of elements in a page.
const MAX_ELEMENTS: i64 = 500;
/// Min number of elements in a page.
const MIN_ELEMENTS: i64 = 0;

///
/// Validate list taxonomy input.
/// Number of elements must be between 0 and 500.
///
/// @param list_request The list request to be validated
/// @return input error empty success
///
///
/// TODO: Make the 10000 configurable.
pub fn validate_list_tsn_request(
    list_request: &TaxonomyListRequest
) -> Result<(), ApplicationError> {
    if list_request.start_index > 10000 {
        return Err(ApplicationError::new(
            ErrorType::InputError,
            START_INDEX_TOO_HIGH.to_string()
        ));
    }
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
/// Validate get taxonomy input. Ok if it's an i32.
///
/// @param tsn_str the string to validate
/// @return input error or i32 as success
///
/// Example success
/// ```
/// let valid_str = "1";
/// assertEq(validate_specific_tsn_request(valid_str));
/// ```
/// Example error
/// ```
/// let valid_str = "2";
/// assertEq(validate_specific_tsn_request(valid_str));
/// ```
///
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
