use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Review {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_fk: i32,
    pub review_text: String,

    pub rev_data: Option<chrono::DateTime<chrono::Utc>> ,
    pub rating: i16,
    pub release_fk: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReviewResponse {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_fk: i32,
    pub review_text: String,

    pub rev_data: Option<chrono::DateTime<chrono::Utc>> ,
    pub rating: i16,
    pub release_fk: i32,
    pub login: String,
    pub avatar: String
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SimpleReview {
    pub review_text: String,
    pub rating: i16,
    pub release_fk: i32,
}