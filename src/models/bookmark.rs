use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bookmark {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_fk: i32,
    pub bookmark_name: String,
    pub release_fk: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SimpleBookmark {
    pub user_fk: i32,
    pub bookmark_name: String,
    pub release_fk: i32,
}
