use actix_web::web;
use crate::handler::health_check_handler::health_check_handler;

pub fn health_check_cfg(cfg: &mut web::ServiceConfig){
    cfg.route("/health_check", web::get().to(health_check_handler));
}
