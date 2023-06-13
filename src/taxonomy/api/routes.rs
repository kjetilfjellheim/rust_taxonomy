use crate::taxonomy::api::request::TaxonomyListRequestQuery;
use actix_web::{get, web, web::Path, web::Query, HttpResponse};

use crate::taxonomy::api::response::{TaxonomyElementType, TaxonomyListResponseType};
use crate::taxonomy::dao::{find_all_tsn, find_specific_tsn};
use crate::taxonomy::model::{validate_list_tsn_request, validate_specific_tsn_request};
use crate::taxonomy::model::{ApplicationError, TaxonomyGetRequest, TaxonomyListRequest};

// Common constants. Move these to the configuration later.
const DEFAULT_START_INDEX: i64 = 0;
const DEFAULT_PAGE_SIZE: i64 = 500;

/**
 * List all taxonomy elements.
 */
#[get("/taxonomy")]
pub async fn list_tsn(
    list_request_params: Query<TaxonomyListRequestQuery>
) -> Result<HttpResponse, ApplicationError> {
    // Create list request object.
    let list_request = TaxonomyListRequest::new(
        list_request_params
            .start_index
            .unwrap_or(DEFAULT_START_INDEX),
        list_request_params.page_size.unwrap_or(DEFAULT_PAGE_SIZE),
    );
    // Validate list request.
    validate_list_tsn_request(&list_request)?;
    // Get taxonomy elements.
    let taxonomy_elements = web::block(|| find_all_tsn(list_request)).await.unwrap();
    // Handle taxonomy elements result.
    match taxonomy_elements {
        Ok(taxonomy_elements) => {
            Ok(HttpResponse::Ok().json(TaxonomyListResponseType::from(taxonomy_elements)))
        }
        Err(application_error) => Err(application_error),
    }
}

/**
 * Get single taxonomy element.
 */
#[get("/taxonomy/{tsn}")]
pub async fn get_specific_tsn(tsn: Path<String>) -> Result<HttpResponse, ApplicationError> {
    //Validate taxonomy value.
    let tsn = validate_specific_tsn_request(&tsn.into_inner())?;
    // Create taxonomy request.
    let taxonomy_request = TaxonomyGetRequest::new(tsn);
    // Get taxonomy element.
    let taxonomy_element = web::block(|| find_specific_tsn(taxonomy_request))
        .await
        .unwrap();
    // Handle taxonomy element result.
    match taxonomy_element {
        Ok(data) => Ok(HttpResponse::Ok().json(TaxonomyElementType::from(data))),
        Err(application_error) => Err(application_error),
    }
}
