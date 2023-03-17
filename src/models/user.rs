use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub login: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DbUser {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub email: String,
}
