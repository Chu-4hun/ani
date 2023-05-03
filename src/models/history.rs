use chrono::{DateTime, Utc, serde::ts_seconds_option};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{user::DbUser, episode::Episode};

#[derive(Serialize, Deserialize, FromRow)]
pub struct DBHistory {
    #[serde(skip_deserializing)]
    pub id: i32,

    pub user_fk: i32,
    pub episode: i32,

    pub date_watched: DateTime<Utc>,
    
    pub duration: f64,
}
