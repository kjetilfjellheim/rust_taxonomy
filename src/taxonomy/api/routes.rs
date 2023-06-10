use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

use crate::taxonomy::api::error_handler::CustomError;
use crate::taxonomy::api::response::LongnameResponseType;
use crate::taxonomy::model::{ErrorType, ApplicationError, ListRequest };
use crate::taxonomy::dao::{find_all};

const DEFAULT_START_INDEX: i64 = 0;
const DEFAULT_PAGE_SIZE: i64 = 500;

#[get("/longnames")]
pub async fn list_longnames(list_request_params: web::Query<ListRequestQuery>) -> Result<HttpResponse, CustomError> {
    let list_request = ListRequest::new(list_request_params.start_index.unwrap_or(DEFAULT_START_INDEX), list_request_params.page_size.unwrap_or(DEFAULT_PAGE_SIZE));
    let longnames = web::block(|| find_all(list_request)).await.unwrap();
    match longnames {
        Ok(longnames) => Ok(HttpResponse::Ok().json(LongnameResponseType::from(longnames))),
        Err(application_error) => { handle_error(application_error) }
    }
}

fn handle_error(application_error: ApplicationError) -> Result<HttpResponse, CustomError> {
    match application_error.error_type {
        ErrorType::ConnectionError => { Err(CustomError::new(500, application_error.message)) },
        ErrorType::DbProgramError => { Err(CustomError::new(500, application_error.message)) },
    }
}

#[derive(Debug, Deserialize)]
pub struct ListRequestQuery {
    start_index: Option<i64>,
    page_size: Option<i64>
}
