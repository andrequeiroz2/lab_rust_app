use actix_web::{FromRequest, HttpMessage};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::{Display, EnumString};
use std::future;
use jsonwebtoken::Header;
use crate::handler::{self, error_handler::ErrorEnum};


#[derive(Debug)]
pub struct TokenData<T> {
    pub header: Header,
    pub claims: T,
}

#[derive(EnumString, Display)]
pub enum JwtKeysEnum {
    private_key,
    public_key
}

impl JwtKeysEnum {  
    pub fn to_lowercase(&self) -> String{
        self.to_string().to_string()
    }
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub email: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Login{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims{
    pub sub: String,
    pub exp: usize,
    pub email: String,
    pub id: i32
}

impl FromRequest for Claims{

    type Error = handler::error_handler::ErrorEnum;
    type Future = future::Ready<Result<Self,Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest, 
        payload: &mut actix_web::dev::Payload
    ) -> std::future::Ready<Result<Claims, ErrorEnum>> {
        
        match req.extensions().get::<Claims>() {
            Some(claim)=> future::ready(Ok(claim.clone())),        
            None => future::ready(Err(ErrorEnum::Unauthorized("Unauthorized.".into(), "Bad Claims".to_string())))
        }

    }
}


#[derive(Clone, PartialEq)]
pub enum Role{
    User,
    Admin
}

impl Role{
    pub fn from_str(role: &str)-> Role{
        match role.to_lowercase().as_str(){
            "admin"=> Role::Admin,
            _=> Role::User,
        }
    }
}

impl fmt::Display for Role{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin")
        }
    }
}