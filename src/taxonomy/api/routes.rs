use crate::taxonomy::api::request::{TaxonomyListRequestQuery, TaxonomyListRequestBody};
use crate::taxonomy::api::response::{TaxonomyElementType, TaxonomyListResponseType, TaxonomyHierarchyType};
use crate::taxonomy::model::{validate_list_tsn_request, validate_specific_tsn_request};
use crate::taxonomy::model::{ApplicationError, TaxonomyGetRequest, TaxonomyListSort, TaxonomyListOrder, TaxonomyListRequest};
use crate::taxonomy::service::{
    find_taxonomies as find_taxonomies_service, find_taxonomy as find_taxonomy_service, find_taxonomy_hierarchy as find_taxonomy_hierarchy_service,
};
use actix_web::{post, get, web, web::Path, web::Query, HttpResponse, web::Json};

///Default value if start index is not specified.
const DEFAULT_START_INDEX: i64 = 0;
/// Default value if page size is not specified.
const DEFAULT_PAGE_SIZE: i64 = 500;

///
/// List taxonomy elements.
///
#[post("/taxonomy")]
pub async fn find_taxonomies(
    list_request_params: Query<TaxonomyListRequestQuery>,
    body: Json<TaxonomyListRequestBody>
) -> Result<HttpResponse, ApplicationError> {
    // Create list request object.
    let list_request = TaxonomyListRequest::new(
        list_request_params
            .start_index
            .unwrap_or(DEFAULT_START_INDEX),
        list_request_params
            .page_size
            .unwrap_or(DEFAULT_PAGE_SIZE),
        get_sort(&list_request_params.sort),
        get_order(&list_request_params.order),
        body.kingdom_name.clone(),
        body.rank_name.clone(),
        body.name.clone()
    );
    // Validate list request.
    validate_list_tsn_request(&list_request)?;
    // Get taxonomy elements.
    let taxonomy_elements = web::block(|| find_taxonomies_service(list_request))
        .await
        .unwrap();
    // Handle taxonomy elements result.
    match taxonomy_elements {
        Ok(taxonomy_elements) => {
            Ok(HttpResponse::Ok().json(TaxonomyListResponseType::from(taxonomy_elements)))
        }
        Err(application_error) => Err(application_error),
    }
}

///
/// Get single taxonomy element.
///
#[get("/taxonomy/{tsn}")]
pub async fn find_taxonomy(tsn: Path<String>) -> Result<HttpResponse, ApplicationError> {
    //Validate taxonomy value.
    let tsn = validate_specific_tsn_request(&tsn.into_inner())?;
    // Create taxonomy request.
    let taxonomy_request = TaxonomyGetRequest::new(tsn);
    // Get taxonomy element.
    let taxonomy_element = web::block(|| find_taxonomy_service(taxonomy_request))
        .await
        .unwrap();
    // Handle taxonomy element result.
    match taxonomy_element {
        Ok(data) => Ok(HttpResponse::Ok().json(TaxonomyElementType::from(data))),
        Err(application_error) => Err(application_error),
    }
}

///
/// Get taxonomy hierarchy.
///
#[get("/taxonomy/{tsn}/hierarchy")]
pub async fn find_taxonomy_hierarchy(tsn: Path<String>) -> Result<HttpResponse, ApplicationError> {
    //Validate taxonomy value.
    let tsn = validate_specific_tsn_request(&tsn.into_inner())?;
    // Create taxonomy request.
    let taxonomy_request = TaxonomyGetRequest::new(tsn);
    // Get taxonomy hierarchy.
    let taxonomy_hierarchy = web::block(|| find_taxonomy_hierarchy_service(taxonomy_request))
    .await
    .unwrap();
    // Handle taxonomy hierarchy result.
    match taxonomy_hierarchy {
        Ok(data) => Ok(HttpResponse::Ok().json(TaxonomyHierarchyType::from(data))),
        Err(application_error) => Err(application_error),
    }
}

fn get_sort(sort: &Option<crate::taxonomy::api::request::TaxonomyListSort>) -> TaxonomyListSort {
    match sort {
        None => TaxonomyListSort::Tsn,
        Some(value) => {
            match value {
                crate::taxonomy::api::request::TaxonomyListSort::Tsn => TaxonomyListSort::Tsn,
                crate::taxonomy::api::request::TaxonomyListSort::Name => TaxonomyListSort::Name,
            }
        }
    }
}

fn get_order(order: &Option<crate::taxonomy::api::request::TaxonomyListOrder>) -> TaxonomyListOrder {
    match order {
        None => TaxonomyListOrder::Asc,
        Some(value) => {
            match value {
                crate::taxonomy::api::request::TaxonomyListOrder::Asc => TaxonomyListOrder::Asc,
                crate::taxonomy::api::request::TaxonomyListOrder::Desc => TaxonomyListOrder::Desc,
            }
        }
    }
}
