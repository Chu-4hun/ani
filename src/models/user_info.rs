use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, FromRow)]
pub struct UserInfo {
    pub id: i32,
    pub status: String,
    pub avatar: String,
    pub regi_date: DateTime<Utc>,
}
