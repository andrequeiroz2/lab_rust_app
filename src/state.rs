use sqlx::mysql::MySqlPool;
use actix_web::web::Data;


pub struct AppState{
    pub health_check: String,
    pub db: MySqlPool
}


pub fn app_state(db_pool: MySqlPool) -> Data<AppState>{
    
    let shared_data= Data::new(AppState {
        health_check: "I'm good.".to_string(),
        db: db_pool,
    });

    return shared_data;
}

