use actix_web::{get, web, HttpResponse};

use crate::taxonomy::api::error_handler::CustomError;
use crate::taxonomy::model::{ErrorType, ApplicationError };
use crate::taxonomy::dao::{find_all};

#[get("/longnames")]
pub async fn list_longnames() -> Result<HttpResponse, CustomError> {
    let longnames = web::block(|| find_all()).await.unwrap();
    match longnames {
        Ok(longnames) => Ok(HttpResponse::Ok().json(longnames)),
        Err(application_error) => { handle_error(application_error) }
    }
}

fn handle_error(application_error: ApplicationError) -> Result<HttpResponse, CustomError> {
    match application_error.error_type {
        ErrorType::ConnectionError => { Err(CustomError::new(500, application_error.message)) },
        ErrorType::DbProgramError => { Err(CustomError::new(500, application_error.message)) },
    }
}
