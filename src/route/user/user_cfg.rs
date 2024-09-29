use actix_web::{web, dev::fn_service};
use actix_web_lab::middleware::{from_fn, Next};
use crate::handler::auth_handler::auth_middleware;

use crate::handler::user_handler::{
    user_create, 
    user_get_by_id,
    user_get_by_email,
    update_user,
    delete_user,
};

pub fn user_cfg(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/users")
        .route("/create", web::post().to(user_create))
        .service(web::scope("")
            .wrap(from_fn(auth_middleware))
            .route("/id/{user_id}", web::get().to(user_get_by_id))
            .route("/email/{user_email}", web::get().to(user_get_by_email))
            .route("/{user_id}", web::put().to(update_user))
            .route("/{user_id}", web::delete().to(delete_user))
        )
        
    );
}