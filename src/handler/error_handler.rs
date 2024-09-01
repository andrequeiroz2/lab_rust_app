use serde::Serialize;
use actix_web::{error, http::StatusCode, HttpResponse};
use log::error;
use std::fmt;
use sqlx::error::Error as SQLxError;

#[derive(Debug, Serialize)]
pub enum ErrorEnum {
    DBError(String),
    ActixError(String)
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse{
    error_message: String
}

impl ErrorEnum{
    fn error_response(&self) -> String{

        match self {
            
            ErrorEnum::DBError(msg)=>{
                error!("Database error occurred: {:?}", msg);
                "Database Error".into()
            },

            ErrorEnum::ActixError(msg)=>{
                error!("Server error occurred: {:?}", msg);
                "Internal Server Error".into()
            }
        }
    }
}

impl error::ResponseError for ErrorEnum {

    fn status_code(&self) -> StatusCode {
        match self {
            ErrorEnum::DBError(_msg) | ErrorEnum::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for ErrorEnum{
    fn fmt(&self, f: &mut fmt::Formatter)-> Result<(), fmt::Error>{
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for ErrorEnum {
    
    fn from(err: actix_web::error::Error) -> Self {
        ErrorEnum::ActixError(err.to_string())
    }
}

impl From<SQLxError> for ErrorEnum{
    fn from(err: SQLxError)-> Self{
        ErrorEnum::DBError(err.to_string())
    }
}
