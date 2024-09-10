use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use log::{error, info};


pub async fn get_db_pool(database_url: &str, max_connections: u32)-> Pool<MySql>{

    let pool = MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await;

    match pool {
        Ok(p) => {
            info!("🐬 Successfully connected to target MySql server!");
            return p;
        }

        Err(err)=> {
            error!("💥 Failed to connect to the target MySql server!");
            error!("💥 Error: {:?}", err);
            std::process::exit(1);
        }
    }

}