use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

use crate::AppState;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub login: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserNoPassword {
    pub id: i32,
    pub login: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DbUser {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub email: String,
}

pub async fn get_user_by_name(login: &str, state: Data<AppState>) -> Result<DbUser, sqlx::Error> {
    let user = sqlx::query_as::<_, DbUser>(
        "
        SELECT *
        FROM users
        WHERE login = $1
        ",
    )
    .bind(login)
    .fetch_one(&state.db)
    .await?;
    Ok(user)
}
pub async fn user_is_unique(
    login: &str,
    email: &str,
    state: &Data<AppState>,
) -> Result<bool, sqlx::Error> {
    let count = sqlx::query_scalar!(
        "SELECT count(id) FROM users WHERE login = $1 OR email = $2",
        login,
        email
    )
    .fetch_one(&state.db)
    .await?
    .unwrap_or(0);
    Ok(count == 0)
}

pub async fn get_user_by_name_and_email(
    login: &str,
    email: &str,
    state: Data<AppState>,
) -> Result<DbUser, sqlx::Error> {
    let user = sqlx::query_as::<_, DbUser>(
        "
        SELECT *
        FROM users
        WHERE login = $1, email = $2
        ",
    )
    .bind(login)
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

pub async fn get_users_by_simalar_name(
    login: &str,
    state: Data<AppState>,
) -> Result<Vec<DbUser>, sqlx::Error> {
    let user = sqlx::query_as::<_, DbUser>(
        "
        SELECT *
        FROM users
        WHERE login LIKE $1
        ",
    )
    .bind(format!("%{}%", login))
    .fetch_all(&state.db)
    .await?;
    Ok(user)
}
