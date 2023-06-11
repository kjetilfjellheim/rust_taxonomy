use crate::taxonomy::api::request::ListRequestQuery;
use actix_web::{get, web, web::Path, web::Query, HttpResponse};

use crate::taxonomy::api::response::{LongnameResponseType, LongnameType};
use crate::taxonomy::dao::{ find_all_tsn, find_specific_tsn };
use crate::taxonomy::model::{ApplicationError, ListRequest};
use crate::taxonomy::model::{ validate_request, validate_tsn};
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
    list_request_params: Query<ListRequestQuery>,
) -> Result<HttpResponse, ApplicationError> {
    let list_request = ListRequest::new(
        list_request_params
            .start_index
            .unwrap_or(DEFAULT_START_INDEX),
        list_request_params.page_size.unwrap_or(DEFAULT_PAGE_SIZE),
    );
    validate_request(&list_request)?;
    let longnames = web::block(|| find_all_tsn(list_request)).await.unwrap();
    match longnames {
        Ok(longnames) => Ok(HttpResponse::Ok().json(LongnameResponseType::from(longnames))),
        Err(application_error) => Err(application_error),
    }
}

///
/// Get specific tsn details
///
#[get("/taxonomy/{tsn}")]
pub async fn get_specific_tsn(tsn: Path<String>) -> Result<HttpResponse, ApplicationError> {
    let tsn = validate_tsn(&tsn.into_inner())?;
    let longname = web::block(move || find_specific_tsn(&tsn)).await.unwrap();
    match longname {
        Ok(longname) => Ok(HttpResponse::Ok().json(LongnameType::from(longname))),
        Err(application_error) => Err(application_error),
    }
}
