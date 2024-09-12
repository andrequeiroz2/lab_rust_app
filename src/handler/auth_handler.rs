use actix_web::HttpResponse;
use actix_web::web;
use chrono::{Duration, Utc};
use log::error;
use log::info;
use std::fs::OpenOptions;
use std::{fs::File, io::Read, path::Path};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
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

use crate::model::user::user_model::UserComplit;
use crate::state::AppState;
use crate::model::{
    user::user_model::User,
    auth::auth_model::{Claims, JwtKeysEnum, Login}
};
use crate::config::Config;
use super::error_handler::ErrorEnum;
use crate::database::user_db::get_user_complet_by_email_db;


pub async fn auth_login(
    login: web::Json<Login>,
    app_state: web::Data<AppState>
) -> Result<HttpResponse, ErrorEnum>{
    
    let user = match get_user_complet_by_email_db(&app_state.db, &login.email)
    .await{
        Ok(r)=>r,
        Err(e)=> Err(e)?
    };

    match verify_password(&login.password, &user.password){
        Ok(r)=>r,
        Err(e)=>Err(e)?
    };

    let token = match get_jwt_for_user(&user){
        Ok(r)=>r,
        Err(e)=>Err(e)?
    };

    Ok(HttpResponse::Ok().json(token))
}

fn read_key_file(key: String)-> Result<File, ErrorEnum>{

    // let file_path = &format!("../../security/{}.pem", key);
    info!("{key}");
    let file_path = Path::new("./security").join(format!("{}.pem", key));

    let mut options = OpenOptions::new();
    let file = match options.read(true).open(file_path){
        
        Ok(f) => f,

        Err(e) => {
            // let err = format!("file_path: {}, error: {}", file_path, e);
            // error!("{err}");
            Err(ErrorEnum::FileError("Intrenal Server Error.".into(), e.to_string()))?
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
            Err(ErrorEnum::FileError("Intrenal Server Error.".into(), e.to_string()))?
        }
    }
}

fn get_jwt_for_user(user: &UserComplit) -> Result<String, ErrorEnum>{

    let config = Config::init();

    let expiration_time = match Utc::now().checked_add_signed(Duration::seconds(config.jwt_expiration_time_seconds)){
        Some(time) => time.timestamp(),

        None=>{
            error!("expiration_time failed");
            Err(ErrorEnum::FileError("Intrenal Server Error.".into(), "".to_string()))?
        }
    };
    
    let user_claims = Claims{
        sub: user.username.clone(),
        // role: user.role.clone(),
        exp: expiration_time as usize
    };

    let mut key_file = match read_key_file(JwtKeysEnum::private_key.to_lowercase()){
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

}

fn get_hashed_password(password: &str)-> Result<String, ErrorEnum>{
    
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Scrypt.hash_password(password.as_bytes(), &salt)?.to_string();

    Ok(password_hash)        
}

fn verify_password(password: &str, password_hash: &str)-> Result<bool, ErrorEnum>{
    
    let parsed_hash = match PasswordHash::new(password_hash){
        Ok(r)=>r,
        Err(e)=> Err(ErrorEnum::Unauthorized("User Unauthorized".into(), e.to_string()))?
    };

    Ok(Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
