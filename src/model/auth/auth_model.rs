use chrono::{Duration, Utc};
use log::error;
use std::fs::OpenOptions;
use std::{fmt, fs::File, io::Read};
use serde::{Deserialize, Serialize};
use scrypt::{
    password_hash::{
        rand_core::OsRng, 
        PasswordHash, 
        PasswordHasher, 
        PasswordVerifier,
        SaltString
    },
    Scrypt,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::config::Config;
use crate::handler::error_handler::ErrorEnum;
use crate::model::user::user_model::User;
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display)]
enum JwtKeysEnum {
    PrivateKeys,
    PublicKeys
}

impl JwtKeysEnum {
    
    fn to_lowercase(&self) -> String{
        self.to_string().to_string()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Login{
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims{
    pub sub: String,
    // pub role: String,
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

fn read_key_file(key: String)-> Result<File, ErrorEnum>{

    let file_path = &format!("../../../security/{}.pem", key);

    let mut options = OpenOptions::new();
    let file = match options.read(true).open(file_path){
        
        Ok(f) => f,

        Err(e) => {
            let err = format!("file_path: {}, error: {}", file_path, e);
            error!("{err}");
            Err(ErrorEnum::FileError("Intrenal Server Error.".into()))?
        }
    };

    Ok(file)
}


fn get_secret(file: &mut File) -> Result<Vec<u8>, ErrorEnum> {
    
    let mut buffer = Vec::new();
    
    match file.read_to_end(&mut buffer){
        
        Ok(r )=> Ok(r.to_le_bytes().to_vec()),
        
        Err(e) =>{
            let err = format!("{}", e);
            error!("Convert file on vec<8> failed: {err}");
            Err(ErrorEnum::FileError("Intrenal Server Error.".into()))?
        }
    }
}


pub fn get_jwt_for_user(user: &User) -> Result<String, ErrorEnum>{

    let config = Config::init();

    let expiration_time = match Utc::now().checked_add_signed(Duration::seconds(config.jwt_expiration_time_seconds)){
        Some(time) => time.timestamp(),

        None=>{
            error!("expiration_time failed");
            Err(ErrorEnum::FileError("Intrenal Server Error.".into()))?
        }
    };
    
    let user_claims = Claims{
        sub: user.username.clone(),
        // role: user.role.clone(),
        exp: expiration_time as usize
    };

    let mut key_file = match read_key_file(JwtKeysEnum::PrivateKeys.to_lowercase()){
        Ok(r) => r,
        Err(e) => Err(e)?
    };

    let secret = match get_secret(&mut key_file){
        Ok(r) => r,
        Err(e) => Err(e)?
    };

    let token = match encode(
        &Header::default(), 
        &user_claims, 
        &EncodingKey::from_secret(&secret),){
            Ok(r) => r,
            Err(e)=> Err(e)?,
        };
    
    Ok(token)

    // let toke
}


pub fn get_hashed_password(password: &str)-> Result<String, ErrorEnum>{
    
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Scrypt.hash_password(password.as_bytes(), &salt)?.to_string();

    Ok(password_hash)        
}

pub fn verify_password(password: &str, password_hash: &str)-> Result<bool, ErrorEnum>{
    
    let parsed_hash: PasswordHash<'_> = PasswordHash::new(password_hash)?;

    Ok(Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
