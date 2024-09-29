use std::io;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod config;
mod state;
mod handler;
mod model;
mod database;
mod route;
use route::{
    health_check::health_check_cfg::health_check_cfg,
    auth::auth_cfg::auth_cfg,
    user::user_cfg::user_cfg,
};
use actix_web_lab::middleware::from_fn;


#[actix_rt::main]
async fn main()-> io::Result<()> {
    
    dotenv().ok();

    log4rs::init_file("log_config.yml", Default::default()).expect("Log config file not found.");

    let config = config::Config::init();

    let db_pool = database::connection_db::get_db_pool(&config.database_url, config.max_connections).await;

    let shared_data = state::app_state(db_pool);

    let app = move ||{
        App::new()
        .app_data(shared_data.clone())
        .configure(health_check_cfg)
        .configure(auth_cfg)
        .configure(user_cfg)
    };

    let host_port = config.host_port;

    HttpServer::new(app).bind(&host_port)?.run().await

}
