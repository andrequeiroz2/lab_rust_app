use actix_web::{
    body::MessageBody, dev::{
        ServiceRequest, 
        ServiceResponse,
    }, 
    http::header::AUTHORIZATION, 
    web, 
    HttpMessage, 
    HttpResponse,
    Error
};

use chrono::{Duration, Utc};
use log::error;
use jsonwebtoken::{
    decode, 
    encode, 
    DecodingKey, 
    EncodingKey, 
    Header, 
    TokenData, 
    Validation
};
use scrypt::{
    password_hash::{ 
        PasswordHash,  
        PasswordVerifier,
    },
    Scrypt,
};
use actix_web_lab::middleware::Next;

use crate::model::auth::auth_model::TokenResponse;
use crate::model::user::user_model::UserComplit;
use crate::state::AppState;
use crate::model::auth::auth_model::{Claims, Login};
use crate::config::Config;
use crate::handler::certify_handler;
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


fn get_jwt_for_user(user: &UserComplit) -> Result<TokenResponse, ErrorEnum>{

    let config = Config::init();

    let expiration_time = match Utc::now().checked_add_signed(Duration::seconds(config.jwt_expiration_time_seconds)){
        Some(time) => time.timestamp(),

        None=>{
            error!("expiration_time failed");
            Err(ErrorEnum::FileError("Intrenal Server Error.".into(), "expiration_time failed".to_string()))?
        }
    };
    
    let user_claims = Claims{
        sub: user.username.clone(),
        exp: expiration_time as usize,
        email: user.email.clone(),
        id: user.id
    };

    let secret = certify_handler::PrivateKey.clone();

    let token = match encode(
        &Header::default(), 
        &user_claims, 
        &EncodingKey::from_secret(&secret),){
            Ok(r) => r,
            Err(e)=> Err(e)?,
        };
    
    Ok(
        TokenResponse{
            token: token,
            email: user.email.clone()
        }
    )
}


fn verify_password(password: &str, password_hash: &str)-> Result<bool, ErrorEnum>{
    
    let parsed_hash = match PasswordHash::new(password_hash){
        Ok(r)=>r,
        Err(e)=> Err(ErrorEnum::Unauthorized("User Unauthorized.".into(), e.to_string()))?
    };

    Ok(Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

fn decode_jwt(jwt: String)-> Result<TokenData<Claims>, jsonwebtoken::errors::Error>{
    
    let secret = certify_handler::PrivateKey.clone();
    
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &jwt, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    );
    println!("{:?}", claim_data);
    claim_data
}


pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>,Error> {

        let auth = req.headers().get(AUTHORIZATION);

        if auth.is_none(){
            Err(ErrorEnum::Unauthorized("Unauthorized.".into(), "Invalid BarerToken".to_string()))?;
        }

        let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();
        let claim = match decode_jwt(token){
            Ok(r)=> r,
            Err(err) => Err(ErrorEnum::Unauthorized("Unauthorized.".into(), err.to_string()))?
        };
        
        req.extensions_mut().insert(claim.claims);
    
        next.call(req).await
        .map_err(|err| Error::from(ErrorEnum::Unauthorized("Unauthorized.".into(), err.to_string())))

    }
