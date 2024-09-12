use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::{Display, EnumString};

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

#[derive(Deserialize, Debug, Clone)]
pub struct Login{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims{
    pub sub: String,
    pub exp: usize,
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