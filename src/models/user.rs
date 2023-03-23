use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbUser {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub email: String,
}


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserWithInfo {
    pub id: i32,
    pub login: String,
    pub avatar: String,
    pub status: String,
}