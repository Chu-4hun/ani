use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
struct Release {
    id: i32,
    release_type: ReleaseType,
    release_date: DateTime<Utc>,
    rating: f32,
    min_age: f32,
    director: String,
    author: String,
    studio: String,
    description: String,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, sqlx::Type, Copy)]
#[repr(i32)]
pub enum ReleaseType {
    Cinema,
    Series,
    Animation,
}
