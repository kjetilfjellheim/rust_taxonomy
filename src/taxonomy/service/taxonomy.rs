use crate::taxonomy::connection;
use crate::taxonomy::dao::{find_taxonomies as find_taxonomies_dao, find_taxonomy as find_taxonomy_dao};
use crate::taxonomy::dao::TaxonomicUnit;
use crate::taxonomy::model::ErrorType;
use crate::taxonomy::model::{
    ApplicationError, TaxonomyListElement, TaxonomyListRequest, TaxonomyListResponse, TaxonomyGetRequest, TaxonomyGetResponse,
};
use log::warn;

/// Error text if unknown error occurs during query.
const QUERY_ERROR_STRING: &str = "Error querying taxonomic data";
const TAXONOMY_NOT_FOUND: &str = "Did not find taxonomic data";
///
/// Find all taxonomy elements using start_index and page_size.
///
pub fn find_taxonomies(
    list_request: TaxonomyListRequest
) -> Result<TaxonomyListResponse, ApplicationError> {
    // Get connection
    let mut conn = connection()?;

    conn.build_transaction().read_only().run(
        |conn| -> Result<TaxonomyListResponse, ApplicationError> {

            let query_result: Result<Vec<TaxonomicUnit>, diesel::result::Error> =
                find_taxonomies_dao(
                    conn,
                    list_request.start_index,
                    list_request.number_of_elements + 1,
                );
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
            } // end match

        } // end transaction
    )

}

///
/// Get specific taxonomy
///
pub fn find_taxonomy(taxonomy_request: TaxonomyGetRequest) -> Result<TaxonomyGetResponse, ApplicationError> {
// Get connection
    let mut conn = connection()?;

    conn.build_transaction().read_only().run(
        |conn| -> Result<TaxonomyGetResponse, ApplicationError> {

            let taxonomy_unit: Result<TaxonomicUnit, diesel::result::Error> =
                find_taxonomy_dao(
                    conn,
                    taxonomy_request.tsn
                );
            // Test query result.
            match taxonomy_unit {
                Ok(query_result) => Ok(TaxonomyGetResponse::new(
                    query_result.tsn,
                    query_result.complete_name

                )),
                Err(diesel::result::Error::NotFound) => Err(ApplicationError::new(ErrorType::NotFoundError, TAXONOMY_NOT_FOUND.to_string())),
                Err(_) => Err(ApplicationError::new(ErrorType::DbProgramError, QUERY_ERROR_STRING.to_string())),
            } // end match

        } // end transaction
    )
}

///
/// Convert queries elements.
/// TODO: Move to conversion.
///
fn convert_queried_elements(queried_result: Vec<TaxonomicUnit>) -> Vec<TaxonomyListElement> {
    queried_result
        .into_iter()
        .map(|element| TaxonomyListElement::new(element.tsn, element.complete_name))
        .collect()
}
