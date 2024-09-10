#[derive(Debug, Clone)]
pub struct Config{
    pub max_connections: u32,
    pub database_url: String,
    pub allowed_origin: String,
    pub max_age: usize,
    pub host_port: String,
    pub jwt_expiration_time_seconds: i64
}

impl Config{
    pub fn init() -> Config{
        Config{
            max_connections: std::env::var("MAX_CONNECTIONS")
                .expect("MAX_CONNECTIONS must be specified")
                .parse::<u32>().unwrap(),

            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be specified"),

            allowed_origin: std::env::var("ALLOWED_ORIGIN")
                .expect("ALLOWED_ORIGIN must be specified"),

            max_age: std::env::var("MAX_AGE")
                .expect("MAX_AGE must be specified")
                .parse::<usize>().unwrap(),

            host_port: std::env::var("HOST_PORT")
                .expect("HOST_PORT must be specified"),

            jwt_expiration_time_seconds: std::env::var("JWT_EXPIRATION_TIME_SECONDS")
            .expect("JWT_EXPIRATION_TIME_SECONDS must be specified").parse::<i64>().unwrap(),    
        }
    }
}