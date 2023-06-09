#[derive(Debug)]
pub enum ErrorType {
    DbProgramError,
    ConnectionError
}

#[derive(Debug)]
pub struct ApplicationError {
    pub error_type: ErrorType,
    pub message: String
}

impl ApplicationError {
    pub fn new(error_type: ErrorType, message: String) -> Self{
        ApplicationError {
            error_type: error_type,
            message: message,
        }
    }
}
