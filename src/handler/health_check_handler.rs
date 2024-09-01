use actix_web::{web, HttpResponse};
use crate::state::AppState;

use super::error_handler::ErrorEnum;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> Result<HttpResponse, ErrorEnum>{
    let helth_check_response = &app_state.health_check;
    Ok(HttpResponse::Ok().json(helth_check_response))
}