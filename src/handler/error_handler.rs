use serde::Serialize;
use actix_web::{error, http::StatusCode, HttpResponse};
use log::error;
use std::fmt;
use sqlx::error::Error as SQLxError;
use scrypt::password_hash::Error as ScryptError;

#[derive(Debug, Serialize)]
pub enum ErrorEnum {
    DBError(String, String),
    ActixError(String),
    ScryptError(String),
    FileError(String),
    JsonWebTokenError(String),
    BadRequest(String),
    NotFound(String, String),
    Conflict(String, String)
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse{
    error_message: String
}

impl ErrorEnum{
    fn error_response(&self) -> String{

        match self {
            
            ErrorEnum::DBError(msg, e)=>{
                error!("Database error occurred: {:?}", e);
                "Database Error".into()
            },

            ErrorEnum::ActixError(msg)=>{
                error!("Server error occurred: {:?}", msg);
                "Internal Server Error".into()
            },

            ErrorEnum::ScryptError(msg)=>{
                error!("Scrypt error occured: {:?}", msg);
                "Internal Server Error".into()
            },

            ErrorEnum::FileError(msg)=>{
                error!("File error occured: {:?}", msg);
                "Internal Server Error".into()
            },

            ErrorEnum::JsonWebTokenError(msg)=>{
                error!("JsonWebToken error occured: {:?}", msg);
                "Token Error".into()
            }

            ErrorEnum::BadRequest(msg)=>{
                error!("Bad Request error occured: {:?}", msg);
                msg.into()
            }

            ErrorEnum::NotFound(msg, e)=>{
                error!("Not Found occured: {:?}", e);
                msg.into()
            }

            ErrorEnum::Conflict(msg, e)=>{
                error!("Conflict occured: {:?}", e);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for ErrorEnum {

    fn status_code(&self) -> StatusCode {
        match self {

            ErrorEnum::BadRequest(_msg)=>StatusCode::BAD_REQUEST,
            ErrorEnum::NotFound(_msg, _)=>StatusCode::NOT_FOUND,

            // ErrorEnum::DBError(_msg) | 
            // ErrorEnum::ActixError(_msg) |
            // ErrorEnum::ScryptError(_msg) |
            // ErrorEnum::FileError(_msg) |
            // ErrorEnum::JsonWebTokenError(_msg)
            _=> StatusCode::INTERNAL_SERVER_ERROR
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
    fn from(err: actix_web::error::Error) -> Self{
        ErrorEnum::ActixError(err.to_string())
    }
}

impl From<SQLxError> for ErrorEnum{
    fn from(err: SQLxError)-> Self{
        ErrorEnum::DBError(err.to_string(), "".to_string())
    }
}

impl From<ScryptError> for ErrorEnum {
    fn from(err: ScryptError)-> Self{
        ErrorEnum::ScryptError(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for ErrorEnum {
    fn from(err: jsonwebtoken::errors::Error)-> Self {
        ErrorEnum::JsonWebTokenError(err.to_string())
    }
}


