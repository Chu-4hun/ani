use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow,)]
pub struct Friend_request {
    pub id: i32,
    pub usr: String,
    pub friend: String,
    pub request_status: FriendRequestStatus,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum FriendRequestStatus {
    Pending,
    Rejected,
    Accepted,
}