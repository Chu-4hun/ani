use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

use super::user::DbUser;

#[derive(Serialize, Deserialize, FromRow)]
pub struct FriendRequest {
    pub usr: i32,
    pub friend: i32,
    pub request_status: FriendRequestStatus,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[repr(i32)]
pub enum FriendRequestStatus {
    Pending,
    Rejected,
    Accepted,
}
impl FriendRequest {
    pub async fn send_friend_request(
        from_user: DbUser,
        to_user: DbUser,
        state: &Data<AppState>,
    ) -> Result<FriendRequest, sqlx::Error> {
        sqlx::query_as::<_, FriendRequest>(
            "
        INSERT INTO user_friend_requests (usr, friend, request_status)
        VALUES ($1, $2, $3)
        RETURNING *;
        ",
        )
        .bind(from_user.id)
        .bind(to_user.id)
        .bind(0)
        .fetch_one(&state.db)
        .await
    }
    
    pub async fn get_friend_requests(
        from_user: i32,
        state: &Data<AppState>,
    ) -> Result<Vec<FriendRequest>, sqlx::Error> {
        sqlx::query_as::<_, FriendRequest>(
            "
        SELECT * FROM user_friend_requests WHERE usr = $1",
        )
        .bind(from_user)
        .fetch_all(&state.db).await
    }
}
