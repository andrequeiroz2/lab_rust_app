use actix_web::{http::StatusCode, HttpResponse};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use crate::{
    model::user::user_model::{UserCreate, UserUpdate}, 
    state::AppState
};
use super::error_handler::ErrorEnum;
use crate::database::user_db::{
    post_user_db, 
    check_existence_user, 
    get_user_by_id_db,
    get_user_by_email_db,
    patch_user_db,
    delete_user_db
};
use actix_web::web;


fn get_password_hash(password: &String)-> Result<String, ErrorEnum>{
    
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = match Scrypt.hash_password(password.as_bytes(), &salt){
        Ok(r) => r.to_string(),
        Err(e)=>Err(e)?
    };

    Ok(password_hash)
}


pub async fn user_create(
    user: web::Json<UserCreate>,
    app_state: web::Data<AppState>
) -> Result<HttpResponse, ErrorEnum>{

    if user.password != user.confirm_password{
        
        Err(ErrorEnum::BadRequest("".into()))?
    
    }else{
        // pass
    };

    match  check_existence_user(&app_state.db, &user.email).await{
        Ok(r)=> r,
        Err(e)=>Err(e)?
    };

    let pass_hash = match get_password_hash(&user.password){
        Ok(r) => r,
        Err(e) => Err(e)?,
    };

    post_user_db(&app_state.db, user.into(), pass_hash)
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn user_get_by_id(
    user_id: web::Path<i32>,
    app_state: web::Data<AppState>
)-> Result<HttpResponse, ErrorEnum>{
    
    let id = user_id.into_inner();

    get_user_by_id_db(&app_state.db, id)
    .await
    .map(|user| HttpResponse::Ok().json(user))
}

pub async fn user_get_by_email(
    user_email: web::Path<String>,
    app_state: web::Data<AppState>
)-> Result<HttpResponse, ErrorEnum>{

    let user_email = &user_email.into_inner();

    get_user_by_email_db(&app_state.db, user_email)
    .await
    .map(|user| HttpResponse::Ok().json(user))
}

pub async fn update_user(
    user_id: web::Path<i32>,
    user_update: web::Json<UserUpdate>,
    app_state: web::Data<AppState>
)-> Result<HttpResponse, ErrorEnum> {

    let user_id = user_id.into_inner();

    patch_user_db(&app_state.db, user_id, user_update.into())
    .await
    .map(|user| HttpResponse::Ok().json(user))
}

pub async fn delete_user(
    user_id: web::Path<i32>,
    app_state: web::Data<AppState>
)-> Result<HttpResponse, ErrorEnum>{

    let user_id = user_id.into_inner();

    match delete_user_db(&app_state.db, user_id)
    .await{
        Ok(r)=> r,
        Err(e)=> Err(e)?
    };

    Ok(HttpResponse::Accepted().finish())

}
