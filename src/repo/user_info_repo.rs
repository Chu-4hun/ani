use actix_web::web::Data;

use crate::{models::user_info::*, AppState};

impl UserInfo {
    pub async fn get_by_id(
        user_id: i32,
        state: &Data<AppState>,
    ) -> Result<UserInfo, sqlx::Error> {
        sqlx::query_as::<_, UserInfo>(
            "
        SELECT * FROM user_info WHERE id = $1",
        )
        .bind(user_id)
        .fetch_one(&state.db)
        .await
    }
    pub async fn is_valid(&self, state: &Data<AppState>) -> bool {
        let request = match sqlx::query_as::<_, UserInfo>(
            "
        SELECT * FROM user_friend_requests WHERE id = $1 AND status = $2 AND avatar =$3",
        )
        .bind(&self.id)
        .bind(&self.status)
        .bind(&self.avatar)
        .fetch_all(&state.db)
        .await
        {
            Ok(user) => user,
            Err(_) => {
                return false;
            }
        };
        request.len() > 0
    }

    pub async fn update(
        &self,
        id: i32,
        state: &Data<AppState>,
    ) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query!(
            "UPDATE user_info  SET status = $1, avatar = $2 WHERE id = $3
        ",
            &self.status,
            &self.avatar,
            id
        )
        .execute(&state.db)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    pub async fn change_user_avatar(
        from_user: i32,
        avatar: &str,
        state: &Data<AppState>,
    ) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query!(
            "UPDATE user_info  SET avatar = $1 WHERE id = $2
        ",
            avatar,
            from_user
        )
        .execute(&state.db)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    pub async fn change_user_status(
        from_user: i32,
        status: &str,
        state: &Data<AppState>,
    ) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query!(
            "UPDATE user_info  SET status = $1 WHERE id = $2
        ",
            status,
            from_user
        )
        .execute(&state.db)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
}
