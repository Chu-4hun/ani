use actix_web::web::Data;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::AppState;

#[derive(Serialize, Deserialize, FromRow,)]
pub struct User {
    pub user_name: String,
    pub password: String,
    pub email: String,
}


#[derive(Serialize, Deserialize, FromRow,)]
pub struct UserNoPassword {
    pub user_id: i32,
    pub user_name: String,
}


#[derive(Serialize, Deserialize,FromRow)]
pub struct DbUser {
    pub user_id: i32,
    pub user_name: String,
    pub password: String,
    pub email: String,
}

pub async fn get_user_by_name(user_name: &str,state: Data<AppState>) -> Result<DbUser, sqlx::Error> {
    
    let user = sqlx::query_as::<_, DbUser>(
        "
        SELECT *
        FROM users
        WHERE user_name = $1
        ",
    )
    .bind(user_name)
    .fetch_one(&state.db)
    .await?;
    Ok(user)
}

pub async fn get_user_by_name_and_email(user_name: &str, email: &str,state: Data<AppState>) -> Result<DbUser, sqlx::Error> {
    
    let user = sqlx::query_as::<_, DbUser>(
        "
        SELECT *
        FROM users
        WHERE user_name = $1, email = $2
        ",
    )
    .bind(user_name)
    .bind(email)
    .fetch_one(&state.db)
    .await?;
    Ok(user)
}


pub async fn get_user_by_id(user_id: i32, state: &Data<AppState>) -> Result<DbUser, sqlx::Error> {
    let user = sqlx::query_as::<_, DbUser>(
        "
        SELECT *
        FROM users
        WHERE user_id = $1
        ",
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;
    Ok(user)
}

pub async fn get_users_by_simalar_name(user_name: &str, state: Data<AppState>) -> Result<Vec<DbUser>, sqlx::Error> {
    let user = sqlx::query_as::<_, DbUser>(
        "
        SELECT *
        FROM users
        WHERE user_name LIKE $1
        ",
    )
    .bind(format!("%{}%", user_name))
    .fetch_all(&state.db)
    .await?;
    Ok(user)
}