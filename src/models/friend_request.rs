use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

use super::user::DbUser;

#[derive(Serialize, Deserialize, FromRow)]
pub struct FriendRequest {
    pub id: i32,
    pub usr: String,
    pub friend: String,
    pub request_status: FriendRequestStatus,
}

#[derive(Serialize, Deserialize, Clone, PartialEq,sqlx::Type)]
#[repr(i32)]
pub enum FriendRequestStatus {
    Pending,
    Rejected,
    Accepted,
}
impl FriendRequest {
    pub async fn send_friend_request(from_user: DbUser, to_user: DbUser, state: &Data<AppState>) -> Result<FriendRequest, sqlx::Error> {
         sqlx::query_as::<_, FriendRequest>(
            "
        INSERT INTO user_friend_requests (usr, friend, request_status)
        VALUES ($1, $2, $3)
        RETURNING *;
        ",
        )
        .bind(from_user.id)
        .bind(to_user.id)
        .bind(1)
        .fetch_one(&state.db)
        .await
    }
    pub async fn get_friend_requests(from_user: DbUser, state: &Data<AppState>) -> Result<FriendRequest, sqlx::Error> {
         sqlx::query_as::<_, FriendRequest>(
            "
        SELECT INTO user_friend_requests (usr, friend, request_status)
        VALUES ($1, $2, $3)
        RETURNING *;
        ",
        )
        .bind(from_user.id)
        .bind(1)
        .fetch_one(&state.db)
        .await
    }
}
