use actix_web::web::Data;

use crate::{
    models::user::{DbUser, UserWithInfo},
    AppState,
};

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
        WHERE id = $1
        ",
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;
Ok(user)
}

pub async fn get_users_by_simalar_name(
    login: &str,
    cursor: i32,
    limit: i32,
    state: &Data<AppState>,
) -> Result<Vec<UserWithInfo>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserWithInfo>(
        "
        SELECT u.id, u.login, ui.avatar, ui.status
        FROM users u
        LEFT JOIN user_info ui ON u.id = ui.id
        WHERE u.login ILIKE $1 AND u.id >= $2 LIMIT $3
        ",
    )
    .bind(format!("%{}%", login))
    .bind(cursor)
    .bind(limit)
    .fetch_all(&state.db)
    .await?;
    Ok(user)
}
