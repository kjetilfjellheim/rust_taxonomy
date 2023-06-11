use crate::taxonomy::connection;
use crate::taxonomy::dao::TaxonomicUnit;
use crate::taxonomy::dao::{
    taxonomic_units, taxonomic_units::dsl::taxonomic_units as taxonomic_units_dsl,
};
use crate::taxonomy::model::{ApplicationError, ErrorType, ListRequest, ListResponse};
use diesel::prelude::*;
use diesel::result::Error::*;
use log::{debug, warn};

///
/// Error messages.
///
const QUERY_ERROR_STRING: &str = "Error querying taxonomic data";
const LONGNAME_NOT_FOUND: &str = "Did not find that tsn number";

///
/// Find all longnames using start_index and page_size.
///
pub fn find_all(
    list_request: ListRequest,
) -> Result<ListResponse<TaxonomicUnit>, ApplicationError> {
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
pub fn find_longname(tsn: &i32) -> Result<TaxonomicUnit, ApplicationError> {
    let connection = &mut connection()?;
    let query_result = taxonomic_units_dsl
        .select((taxonomic_units::tsn, taxonomic_units::complete_name))
        .find(tsn)
        .first(connection);
    match query_result {
        Ok(longname) => Ok(longname),
        Err(NotFound) => {
            debug!("Did not find tsn {}", tsn);
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
