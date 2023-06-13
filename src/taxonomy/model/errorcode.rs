use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::warn;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

// Common api response object from the api layer.
#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, String>>,
}

// Error types used in the code.
#[derive(Debug, Serialize)]
pub enum ErrorType {
    DbProgramError,
    ConnectionError,
    NotFoundError,
    InputError,
}

impl ErrorType {
    fn get_errorcode(&self) -> i32 {
        match *self {
            Self::DbProgramError => 5001,
            Self::ConnectionError => 5002,
            Self::InputError => 5004,
            Self::NotFoundError => 5005,
        }
    }

    fn get_statuscode(&self) -> StatusCode {
        match *self {
            Self::DbProgramError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InputError => StatusCode::BAD_REQUEST,
            Self::NotFoundError => StatusCode::NOT_FOUND,
        }
    }
}

// Application errors used the logic.
#[derive(Debug, Serialize)]
pub struct ApplicationError {
    pub error_type: ErrorType,
    pub message: String,
}

impl ApplicationError {
    pub fn new(error_type: ErrorType, message: String) -> Self {
        ApplicationError {
            error_type: error_type,
            message: message,
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Convert application error to api response error.
impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        warn!("Response failure: {}", &self);
        let error_response = AppErrorResponse {
            code: self.error_type.get_errorcode(),
            message: self.message.clone(),
            params: None,
        };
        HttpResponse::build(self.error_type.get_statuscode()).json(&error_response)
    }
}
