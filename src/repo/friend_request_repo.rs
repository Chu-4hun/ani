use actix_web::web::Data;

use crate::{models::{friend_request::*, user::DbUser}, AppState};


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
        .bind(FriendRequestStatus::Pending)
        .fetch_one(&state.db)
        .await
    }

    pub async fn get_friend_requests(
        from_user: i32,
        state: &Data<AppState>,
    ) -> Result<Vec<FriendRequest>, sqlx::Error> {
        sqlx::query_as::<_, FriendRequest>(
            "
        SELECT * FROM user_friend_requests WHERE usr = $1 OR friend = $1",
        )
        .bind(from_user)
        .fetch_all(&state.db)
        .await
    }
    pub async fn get_all_user_friends(
        from_user: i32,
        state: &Data<AppState>,
    ) -> Result<Vec<FriendRequest>, sqlx::Error> {
        sqlx::query_as::<_, FriendRequest>(
            "
        SELECT * FROM user_friend_requests WHERE (usr = $1 OR friend =$1) AND request_status = 2",
        )
        .bind(from_user)
        .fetch_all(&state.db)
        .await
    }
    pub async fn update_status(
        &self,
        status: FriendRequestStatus,
        state: &Data<AppState>,
    ) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query!(
            "UPDATE user_friend_requests  SET request_status = $1 
            WHERE (usr = $2 OR friend =$2) AND (usr = $3 OR friend =$3)
        ",
            status as i32,
            self.friend,
            self.usr
        )
        .execute(&state.db)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
    pub async fn is_valid(&self, state: &Data<AppState>) -> bool {
        let request = sqlx::query_as::<_, FriendRequest>(
            "
        SELECT * FROM user_friend_requests WHERE(usr = $1 OR friend =$1) AND (usr = $2 OR friend =$2)",
        )
        .bind(self.usr)
        .bind(self.friend)
        .fetch_one(&state.db)
        .await;
        request.is_ok()
    }
    pub async fn can_update_status(&self, state: &Data<AppState>) -> bool {
        let request = sqlx::query(
            "
        SELECT * FROM user_friend_requests WHERE usr = $1 AND friend = $2 AND request_status = $3",
        )
        .bind(self.usr)
        .bind(self.friend)
        .bind(FriendRequestStatus::Pending)
        .execute(&state.db)
        .await
        .unwrap();
        request.rows_affected() == 0
    }
    pub async fn delete(&self, state: &Data<AppState>) -> bool {
        match sqlx::query(
            "
        DELETE FROM user_friend_requests WHERE usr = $1 AND friend = $2",
        )
        .bind(self.usr)
        .bind(self.friend)
        .execute(&state.db)
        .await
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
