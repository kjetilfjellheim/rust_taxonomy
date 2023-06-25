use crate::taxonomy::connection;
use crate::taxonomy::dao::TaxonomicUnit;
use crate::taxonomy::dao::{
    find_child_taxonomies as find_child_taxonomies_dao, find_taxonomies as find_taxonomies_dao,
    find_taxonomy as find_taxonomy_dao,
};
use crate::taxonomy::model::ErrorType;
use crate::taxonomy::model::{
    ApplicationError, TaxonomyGetChild, TaxonomyGetRequest, TaxonomyGetResponse,
    TaxonomyListElement, TaxonomyListRequest, TaxonomyListResponse, TaxonomyHierarchyElement, TaxonomyHierarchyResponse,
};
use log::warn;
use std::str::FromStr;

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
                    list_request.number_of_elements,
                    list_request.taxonomy_list_sort,
                    list_request.taxonomy_list_order,
                    list_request.filter_kingdomname,
                    list_request.filter_rankname,
                    list_request.filter_name
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
        }, // end transaction
    )
}

///
/// Get specific taxonomy
///
pub fn find_taxonomy(
    taxonomy_request: TaxonomyGetRequest
) -> Result<TaxonomyGetResponse, ApplicationError> {
    // Get connection
    let mut conn = connection()?;

    conn.build_transaction().read_only().run(
        |conn| -> Result<TaxonomyGetResponse, ApplicationError> {
            let taxonomy_unit: Result<TaxonomicUnit, diesel::result::Error> =
                find_taxonomy_dao(conn, taxonomy_request.tsn);
            match taxonomy_unit {
                Ok(taxonomy_unit) => Ok(taxonomy_unit),
                Err(diesel::result::Error::NotFound) => Err(ApplicationError::new(
                    ErrorType::NotFoundError,
                    TAXONOMY_NOT_FOUND.to_string(),
                )),
                Err(_) => Err(ApplicationError::new(
                    ErrorType::DbProgramError,
                    QUERY_ERROR_STRING.to_string(),
                )),
            }
            .and_then(|taxonomy_unit: TaxonomicUnit| {
                let child_taxonomies: Result<Vec<TaxonomicUnit>, diesel::result::Error> =
                    find_child_taxonomies_dao(conn, taxonomy_unit.tsn);
                let mut parent_tsn: Option<i32> = None;
                let mut parent_name: Option<String> = None;
                let parent_taxonomy = find_taxonomy_dao(conn, taxonomy_unit.parent_tsn);
                let children = match child_taxonomies {
                    Ok(children) => Some(
                        children
                            .iter()
                            .map(|taxonomy_unit: &TaxonomicUnit| {
                                TaxonomyGetChild::new(
                                    taxonomy_unit.tsn,
                                    taxonomy_unit.complete_name.clone(),
                                )
                            })
                            .collect(),
                    ),
                    Err(err) => {
                        warn!("Error occured quering child taxonomy: {}", err);
                        return Err(ApplicationError::new(
                            ErrorType::DbProgramError,
                            QUERY_ERROR_STRING.to_string(),
                        ));
                    }
                };
                if taxonomy_unit.tsn != taxonomy_unit.parent_tsn && taxonomy_unit.parent_tsn != 0 {
                    match parent_taxonomy {
                        Ok(parent_taxonomy) => {
                            parent_tsn = Some(taxonomy_unit.parent_tsn);
                            parent_name = Some(parent_taxonomy.complete_name);
                        }
                        Err(err) => {
                            warn!("Error occured quering parent taxonomy: {}", err);
                            return Err(ApplicationError::new(
                                ErrorType::DbProgramError,
                                QUERY_ERROR_STRING.to_string(),
                            ));
                        }
                    }
                }
                Ok(TaxonomyGetResponse::new(
                    taxonomy_unit.tsn,
                    taxonomy_unit.complete_name,
                    taxonomy_unit.kingdom_name.trim().to_string(),
                    taxonomy_unit.rank_name.trim().to_string(),
                    parent_tsn,
                    parent_name,
                    children,
                ))
            }) // end match
        }, // end transaction
    )
}

///
/// Get hierarchy
///
pub fn find_taxonomy_hierarchy(
    taxonomy_request: TaxonomyGetRequest
) -> Result<TaxonomyHierarchyResponse, ApplicationError> {
    // Get connection
    let mut conn = connection()?;

    conn.build_transaction().read_only().run(
        |conn| -> Result<TaxonomyHierarchyResponse, ApplicationError> {
            let taxonomy_unit: Result<TaxonomicUnit, diesel::result::Error> =
                find_taxonomy_dao(conn, taxonomy_request.tsn);

            match taxonomy_unit {
                Ok(taxonomy_unit) => Ok(taxonomy_unit),
                Err(diesel::result::Error::NotFound) => Err(ApplicationError::new(
                    ErrorType::NotFoundError,
                    TAXONOMY_NOT_FOUND.to_string(),
                )),
                Err(_) => Err(ApplicationError::new(
                    ErrorType::DbProgramError,
                    QUERY_ERROR_STRING.to_string(),
                )),
            }
            .and_then(|taxonomy_unit: TaxonomicUnit| {
                let mut hierarchy: Vec<TaxonomyHierarchyElement> = vec!();
                let tsn_coll = taxonomy_unit.hierarchy_string.split('-').map(|f| { <i32 as FromStr>::from_str(f).unwrap() }).collect::<Vec<i32>>();
                for tsn in tsn_coll {
                    print!("{}", tsn);
                    let taxonomy_unit: Result<TaxonomicUnit, diesel::result::Error> = find_taxonomy_dao(conn, tsn);
                    let taxonomy_element = match_taxonomy_unit(taxonomy_unit)?;
                    hierarchy.push(taxonomy_element)
                }
                Ok(TaxonomyHierarchyResponse {
                    hierarchy: hierarchy
                })
            }
        )}
    )
}

fn match_taxonomy_unit(taxonomy_unit: Result<TaxonomicUnit, diesel::result::Error>) -> Result<TaxonomyHierarchyElement, ApplicationError> {
    match taxonomy_unit {
        Err(_err) => { return Err(ApplicationError::new(ErrorType::DbProgramError, QUERY_ERROR_STRING.to_string()))},
        Ok(taxonomy) => {
            Ok(TaxonomyHierarchyElement::new(
                taxonomy.tsn,
                taxonomy.complete_name,
                taxonomy.kingdom_name,
                taxonomy.rank_name))
        }
    }
}

///
/// Convert queries elements.
/// TODO: Move to conversion.
///
fn convert_queried_elements(queried_result: Vec<TaxonomicUnit>) -> Vec<TaxonomyListElement> {
    queried_result
        .into_iter()
        .map(|element| {
            TaxonomyListElement::new(
                element.tsn,
                element.complete_name,
                element.kingdom_name.trim().to_string(),
                element.rank_name.trim().to_string(),
            )
        })
        .collect()
}
