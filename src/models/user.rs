use actix_web::http::Error;
use actix_web::web::Data;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::ConnectionPool;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct DbUser {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub email: String,
}

// pub async fn get_user_by_name(login: &str, state: Data<AppState>) -> Result<DbUser, sqlx::Error> {
//     let user = sqlx::query_as::<_, DbUser>(
//         "
//         SELECT *
//         FROM users
//         WHERE login = $1
//         ",
//     )
//     .bind(login)
//     .fetch_one(&state.db)
//     .await?;
//     Ok(user)
// }
pub fn get_user_by_name(_login: &str, db: ConnectionPool) -> Result<DbUser, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let connection = db.get().expect("Couldn't db database connection");

    users
        .filter(login.eq(_login))
        .first::<DbUser>(&mut connection)
}

pub async fn user_is_unique(
    login: &str,
    email: &str,
    db: ConnectionPool,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let count = users
        .filter(login.eq(login))
        .filter(email.eq(email))
        .count()
        .get_result(&mut db.get().expect("Couldn't db database connection")).unwrap();
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
