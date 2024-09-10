use actix_web::web;

pub fn auth_cfg(cfg: &mut web::ServiceConfig){
    cfg.service(web::scope("/auth"));
}

// pub fn login_cfg(cfg: &mut web::ServiceConfig){
//     cfg.service(
//         web::scope("/login")
//         .route("/", web::post().to(get_login))
//     );
// }