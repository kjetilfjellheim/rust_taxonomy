use crate::taxonomy::api::request::{TaxonomyListRequestQuery };
use actix_web::{get, web, web::Path, web::Query, HttpResponse};

use crate::taxonomy::api::response::{TaxonomyElementType, TaxonomyListResponseType};
use crate::taxonomy::dao::{ find_all_tsn, find_specific_tsn };
use crate::taxonomy::model::{ApplicationError, TaxonomyListRequest, TaxonomyGetRequest};
use crate::taxonomy::model::{ validate_list_tsn_request, validate_specific_tsn_request};
///
/// Common constants. Move these to the configuration later.
///
const DEFAULT_START_INDEX: i64 = 0;
const DEFAULT_PAGE_SIZE: i64 = 500;

///
/// List all tsn
///
#[get("/taxonomy")]
pub async fn list_tsn(
    list_request_params: Query<TaxonomyListRequestQuery>,
) -> Result<HttpResponse, ApplicationError> {
    let list_request = TaxonomyListRequest::new(
        list_request_params
            .start_index
            .unwrap_or(DEFAULT_START_INDEX),
        list_request_params.page_size.unwrap_or(DEFAULT_PAGE_SIZE),
    );
    validate_list_tsn_request(&list_request)?;
    let longnames = web::block(|| find_all_tsn(list_request)).await.unwrap();
    match longnames {
        Ok(longnames) => Ok(HttpResponse::Ok().json(TaxonomyListResponseType::from(longnames))),
        Err(application_error) => Err(application_error),
    }
}

///
/// Get specific tsn details
///
#[get("/taxonomy/{tsn}")]
pub async fn get_specific_tsn(tsn: Path<String>) -> Result<HttpResponse, ApplicationError> {
    let tsn = validate_specific_tsn_request(&tsn.into_inner())?;
    let get_specific_tsn_request = TaxonomyGetRequest::new(tsn);
    let longname = web::block(|| find_specific_tsn(get_specific_tsn_request)).await.unwrap();
    match longname {
        Ok(longname) => Ok(HttpResponse::Ok().json(TaxonomyElementType::from(longname))),
        Err(application_error) => Err(application_error),
    }
}
