use actix_web::web::Data;

use crate::{models::user_info::*, AppState};


impl UserInfo {
    pub async fn get_user_info(
        user_id: i32,
        state: &Data<AppState>,
    ) -> Result<Vec<UserInfo>, sqlx::Error> {
        sqlx::query_as::<_, UserInfo>(
            "
        SELECT * FROM user_info WHERE user_fk = $1",
        )
        .bind(user_id)
        .fetch_all(&state.db)
        .await
    }
    pub async fn change_user_avatar(
        from_user: i32,
        avatar: &str,
        state: &Data<AppState>,
    ) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query!(
            "UPDATE user_info  SET avatar = $1 WHERE user_FK = $2
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
            "UPDATE user_info  SET status = $1 WHERE user_FK = $2
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
