use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, FromRow, PartialEq)]
pub struct UserInfo {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub status: String,
    pub avatar: String,
    
    #[sqlx(default)]
    pub login: Option<String>,

    #[serde(skip_deserializing)]
    pub register_date: DateTime<Utc>,
}
