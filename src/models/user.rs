use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use sqlx_crud::SqlxCrud;

#[derive(Serialize, Deserialize, FromRow,)]
pub struct User {
    pub user_name: String,
    pub pass: String,
    pub email: String,
}


#[derive(Serialize, Deserialize,FromRow,SqlxCrud)]
pub struct DbUser {
    pub id: i32,
    pub user_name: String,
    pub pass: String,
    pub email: String,
}