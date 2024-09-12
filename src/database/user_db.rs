use sqlx::MySqlPool;
use chrono::Utc;

use crate::{
    handler::error_handler::ErrorEnum, 
    model::user::user_model::{
        User, UserCreate, UserComplit, UserUpdate
    }
};

pub async fn post_user_db(
    pool: &MySqlPool,
    user: UserCreate,
    password_hash: String,
)-> Result<User, ErrorEnum> {
    
    let datetime_now = Utc::now().naive_utc();

    let post_user = match sqlx::query!(
        "INSERT INTO users (username, email, password, created_at) VALUES (?, ?, ?, ?)",
        user.username,
        user.email,
        password_hash,
        datetime_now
    )
    .execute(pool)
    .await{
        Ok(r)=> r.last_insert_id(),
        Err(e)=>Err(ErrorEnum::NotFound("User Not Found".into(), e.to_string()))?
    };
    
    
    let user_last_insert = match sqlx::query_as!(
        User,
        "SELECT id, username, email, created_at, updated_at FROM users WHERE id=?",
        post_user
    )
    .fetch_one(pool)
    .await{
        Ok(r) => r,
        Err(e) => Err(ErrorEnum::NotFound("User Not Found".into(), e.to_string()))?
    };


    Ok(user_last_insert)
}


pub async fn check_existence_user(
    pool: &MySqlPool,
    email: &String,
)-> Result<i64, ErrorEnum> {

    let user_count =  match sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE email=?",
        email
    ).fetch_one(pool)
    .await{
        Ok(r)=> r,
        Err(e)=>Err(ErrorEnum::DBError("Internal Server Error".into(), e.to_string()))?
    };

    Ok(user_count)    
}

pub async fn check_exixtence_user_by_id(
    pool: &MySqlPool,
    id: i32,
)->Result<i64, ErrorEnum>{

    let user_acount = match sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE id=?",
        id
    ).fetch_one(pool)
    .await{
        Ok(r)=>r,
        Err(e)=> Err(ErrorEnum::DBError("Internal Server Error".into(), e.to_string()))?
    };

    Ok(user_acount)
}

pub async fn get_user_by_id_db(
    pool: &MySqlPool, 
    id: i32
)-> Result<User, ErrorEnum>{

    let user_row = sqlx::query!(
        "SELECT id, username, email, created_at, updated_at FROM users WHERE id=?",
        id
    )
    .fetch_one(pool)
    .await
    .map(|user_row|
        User{
            id: user_row.id,
            username: user_row.username,
            email: user_row.email,
            created_at: user_row.created_at,
            updated_at: user_row.updated_at,
        }
    )
    .map_err(|err| ErrorEnum::NotFound("User Not Found".into(), err.to_string()))?;

    Ok(user_row)
}

pub async fn get_user_by_email_db(
    pool: &MySqlPool,
    email: &String
)-> Result<User, ErrorEnum>{

    let user_row= sqlx::query!(
        "SELECT id, username, email, created_at, updated_at FROM users WHERE email=?",
        email
    )
    .fetch_one(pool)
    .await
    .map(|user_row|
        User{
            id: user_row.id,
            username: user_row.username,
            email: user_row.email,
            created_at: user_row.created_at,
            updated_at: user_row.updated_at,
        }
    )
    .map_err(|err| ErrorEnum::NotFound("User Not Found".into(), err.to_string()))?;

    Ok(user_row)
}

pub async fn get_user_complet_by_email_db(
    pool: &MySqlPool,
    email: &String
)-> Result<UserComplit, ErrorEnum>{

    let user_row= sqlx::query!(
        "SELECT id, username, email, password, created_at, updated_at FROM users WHERE email=?",
        email
    )
    .fetch_one(pool)
    .await
    .map(|user_row|
        UserComplit{
            id: user_row.id,
            username: user_row.username,
            email: user_row.email,
            password: user_row.password,
            created_at: user_row.created_at,
            updated_at: user_row.updated_at,
        }
    )
    .map_err(|err| ErrorEnum::NotFound("User Not Found".into(), err.to_string()))?;

    Ok(user_row)
}

pub async fn patch_user_db(
    pool: &MySqlPool,
    user_id: i32,
    update_user: UserUpdate
)-> Result<User, ErrorEnum>{

    let user = match get_user_by_id_db(&pool, user_id).await{
        Ok(r) =>r,
        Err(e)=> Err(e)?
    };

    let username: String = if let Some(username) = update_user.username {
        username
    }else{
        user.username
    };

    let email: String = if let Some(email) = update_user.email {
        email
    }else{
        user.email
    };

    match sqlx::query_as!(
        User,
        "UPDATE users SET username=?, email=? WHERE id=?",
        username,
        email,
        user_id,
    )
    .execute(pool)
    .await {
        Ok(r)=> r,
        Err(e)=> Err(ErrorEnum::DBError("Internal Server Error".into(), e.to_string()))?
    };

    let updated_user = match get_user_by_id_db(pool, user_id).await{
        Ok(r)=> r,
        Err(e)=> Err(e)?
    };
    
    Ok(updated_user)
}

pub async fn delete_user_db(
    pool: &MySqlPool,
    user_id: i32,
)-> Result<u64, ErrorEnum>{

    
    let user_count = match check_exixtence_user_by_id(pool, user_id).await{
        Ok(r)=> r,
        Err(e)=> Err(e)?
    };

    if user_count > 0 {
        
    }else {
        Err(ErrorEnum::NotFound("User Not Found".into(), format!("User Not Found id:{}", user_id)))?
    }

    let user = match sqlx::query!(
        "DELETE FROM users WHERE id=?",
        user_id
    )
    .execute(pool)
    .await{
        Ok(r)=>r.rows_affected(),
        Err(e)=>Err(ErrorEnum::DBError("Internal Server Error".into(), e.to_string()))?
    };

    Ok(user)

}
