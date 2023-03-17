use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct FriendRequest {
    #[serde(skip_deserializing)]
    pub usr: i32,
    pub friend: i32,
    pub request_status: FriendRequestStatus,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, sqlx::Type, Copy)]
#[repr(i32)]
pub enum FriendRequestStatus {
    Pending,
    Rejected,
    Accepted,
}