use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct DBHistory {
    #[serde(skip_deserializing)]
    pub id: i32,

    pub user_fk: i32,
    pub episode: i32,
    
    pub date_watched: DateTime<Utc>,
    
    pub duration: f64,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct HistoryResponse {
    pub release_name: String,
    pub description: String,
    pub img: String,
    pub episode_id: i32,
    pub duration: f64,
    pub date_watched: chrono::DateTime<chrono::Utc>,
    pub dub_id:i32,
}
