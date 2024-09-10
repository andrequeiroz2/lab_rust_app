use sqlx::MySqlPool;

use crate::handler::error_handler::ErrorEnum;
// use crate::handler::

// pub async fn get_login_db(
//     pool: &MySqlPool,
//     email: String,
//     password: String,
// ) -> Result<User, ErrorEnum>{

//     let user_row: User = sqlx::query_as!(
//         User,
//         "SELECT * FROM ezy_course_c6 WHERE tutor_id = $1 AND course_id = $2",
//         email,
//         password
//     )
//     .fetch_optional(pool)
//     .await?;
// }