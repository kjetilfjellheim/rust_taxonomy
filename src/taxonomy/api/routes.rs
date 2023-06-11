use crate::taxonomy::api::request::ListRequestQuery;
use actix_web::{get, web, web::Query, web::Path, HttpResponse};

use crate::taxonomy::api::response::{ LongnameResponseType, LongnameType };
use crate::taxonomy::dao::{ find_all, find_longname } ;
use crate::taxonomy::model::{ApplicationError, ListRequest};

///
/// Common constants. Move these to the configuration later.
///
const DEFAULT_START_INDEX: i64 = 0;
const DEFAULT_PAGE_SIZE: i64 = 500;

///
/// List longnames
///
#[get("/longnames")]
pub async fn list_longnames(
    list_request_params: Query<ListRequestQuery>,
) -> Result<HttpResponse, ApplicationError> {
    let list_request = ListRequest::new(list_request_params.start_index.unwrap_or(DEFAULT_START_INDEX), list_request_params.page_size.unwrap_or(DEFAULT_PAGE_SIZE));
    let longnames = web::block(|| find_all(list_request)).await.unwrap();
    match longnames {
        Ok(longnames) => Ok(HttpResponse::Ok().json(LongnameResponseType::from(longnames))),
        Err(application_error) => Err(application_error),
    }
}

///
/// Get longname
///
#[get("/longnames/{tsn}")]
pub async fn get_longname(
    tsn: Path<String>) -> Result<HttpResponse, ApplicationError> {
    let longname = web::block(move || find_longname(&tsn)).await.unwrap();
    match longname {
        Ok(longname) => Ok(HttpResponse::Ok().json(LongnameType::from(longname))),
        Err(application_error) => Err(application_error),
    }
}
