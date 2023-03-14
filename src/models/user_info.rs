use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow,Queryable)]
pub struct UserInfo {
    pub user_id: i32,
    pub status: String,
    pub avatar: String,
    pub regi_date: usize,
}