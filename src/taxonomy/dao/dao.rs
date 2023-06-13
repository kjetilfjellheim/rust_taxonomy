use crate::taxonomy::connection;
use crate::taxonomy::dao::TaxonomicUnit;
use crate::taxonomy::dao::{
    taxonomic_units, taxonomic_units::dsl::taxonomic_units as taxonomic_units_dsl,
};
use crate::taxonomy::model::{
    ApplicationError, ErrorType, TaxonomyGetRequest, TaxonomyGetResponse, TaxonomyListElement,
    TaxonomyListRequest, TaxonomyListResponse,
};
use diesel::prelude::*;
use diesel::result::Error::*;
use log::{debug, warn};

// Error messages.
const QUERY_ERROR_STRING: &str = "Error querying taxonomic data";
const LONGNAME_NOT_FOUND: &str = "Did not find that tsn number";

/**
 * Find all longnames using start_index and page_size.
 */
pub fn find_all_tsn(
    list_request: TaxonomyListRequest,
) -> Result<TaxonomyListResponse, ApplicationError> {
    // GEt connection
    let connection = &mut connection()?;
    // Query list
    let query_result = taxonomic_units_dsl
        .limit(list_request.number_of_elements + 1)
        .offset(list_request.start_index)
        .select((taxonomic_units::tsn, taxonomic_units::complete_name))
        .load(connection);
    // Test query result.
    match query_result {
        Ok(query_result) => Ok(TaxonomyListResponse::new(
            list_request.start_index,
            list_request.number_of_elements,
            list_request.number_of_elements + 1,
            convert_queried_elements(query_result),
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

/**
 * Query single tsn
 */
pub fn find_specific_tsn(
    get_tsn_request: TaxonomyGetRequest,
) -> Result<TaxonomyGetResponse, ApplicationError> {
    //Get connection
    let connection = &mut connection()?;
    // Query tsn
    let query_result: Result<TaxonomicUnit, diesel::result::Error> = taxonomic_units_dsl
        .select((taxonomic_units::tsn, taxonomic_units::complete_name))
        .find(get_tsn_request.tsn)
        .first(connection);
    // Test query result.
    match query_result {
        Ok(taxonomic_unit) => Ok(TaxonomyGetResponse::new(
            taxonomic_unit.tsn,
            taxonomic_unit.complete_name,
        )),
        Err(NotFound) => {
            debug!("Did not find tsn {}", get_tsn_request.tsn);
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

/**
 * Convert queries elements.
 */
fn convert_queried_elements(queried_result: Vec<TaxonomicUnit>) -> Vec<TaxonomyListElement> {
    queried_result
        .into_iter()
        .map(|element| TaxonomyListElement::new(element.tsn, element.complete_name))
        .collect()
}
