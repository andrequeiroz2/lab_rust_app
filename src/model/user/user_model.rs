use serde::{Serialize, Deserialize};
use actix_web::web;
use chrono::Utc;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(default)]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[serde(default)]
    pub updated_at: Option<chrono::DateTime<Utc>>
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserCreate{
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    // pub role: String,
}

impl From<web::Json<UserCreate>> for UserCreate{
    fn from(user: web::Json<UserCreate>) -> Self {
        UserCreate{
            username: user.username.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
            confirm_password: user.confirm_password.clone(),
            // role: user.role.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserUpdate{
    pub username: Option<String>,
    pub email: Option<String>
}

impl From<web::Json<UserUpdate>> for UserUpdate{
    fn from(user: web::Json<UserUpdate>)-> Self {
        UserUpdate{
            username: user.username.clone(),
            email: user.email.clone(),    
        }
    }
}