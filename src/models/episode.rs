use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Episode {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub release_id: i32,
    pub ep_name: String,
    pub url: String
}