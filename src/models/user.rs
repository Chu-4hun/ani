use serde::{Serialize, Deserialize};
use sqlx::FromRow;

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