use actix_web::web::Data;

use crate::{
    models::{
        history::{DBHistory, HistoryResponse},
        user::DbUser,
    },
    AppState,
};

impl DBHistory {
    pub async fn insert(&self, state: &Data<AppState>) -> Result<DBHistory, sqlx::Error> {
        let result = sqlx::query_as!(
            DBHistory,
            "INSERT INTO history (user_fk, episode, date_watched, duration)
             VALUES ($1, $2, $3, $4) RETURNING id, user_fk, episode, date_watched, duration",
            self.user_fk,
            self.episode,
            self.date_watched,
            self.duration
        )
        .fetch_one(&state.db)
        .await?;
        Ok(result)
    }
    pub async fn insert_values(
        user_fk: DbUser,
        episode: i32,
        duration: f64,
        state: &Data<AppState>,
    ) -> Result<Option<DBHistory>, sqlx::Error> {
        let result = sqlx::query_as!(
            DBHistory,
            "INSERT INTO history (user_fk, episode, duration)
             VALUES ($1, $2, $3) RETURNING id, user_fk, date_watched, episode, duration",
            user_fk.id,
            episode,
            duration
        )
        .fetch_optional(&state.db)
        .await?;
        Ok(result)
    }

    pub async fn get_by_id(
        id: i32,
        state: &Data<AppState>,
    ) -> Result<Option<DBHistory>, sqlx::Error> {
        let result = sqlx::query_as!(
            DBHistory,
            "SELECT id, user_fk, episode, date_watched, duration FROM history WHERE id = $1",
            id
        )
        .fetch_optional(&state.db)
        .await?;
        Ok(result)
    }
    pub async fn get_all_by_user(
        user_id: i32,
        state: &Data<AppState>,
    ) -> Result<Vec<DBHistory>, sqlx::Error> {
        let histories = sqlx::query_as!(
            DBHistory,
            "SELECT * FROM history WHERE user_fk = $1",
            user_id
        )
        .fetch_all(&state.db)
        .await?;
        Ok(histories)
    }
    pub async fn get_all_with_release_info(
        user_id: i32,
        state: &Data<AppState>,
    ) -> Result<Vec<HistoryResponse>, sqlx::Error> {
        let histories = sqlx::query_as!(
            HistoryResponse,
            "SELECT r.release_name, r.description, r.img, h.date_watched, h.duration, h.episode
            FROM releases AS r
            INNER JOIN history AS h ON r.id = h.episode
            WHERE h.user_fk =$1",
            user_id
        )
        .fetch_all(&state.db)
        .await?;
        Ok(histories)
    }

    pub async fn update(&self, state: &Data<AppState>) -> Result<DBHistory, sqlx::Error> {
        let result = sqlx::query_as!(
            DBHistory,
            "UPDATE history SET user_fk = $1, episode = $2, date_watched = $3, duration = $4 WHERE id = $5 RETURNING id, user_fk, episode, date_watched, duration",
            self.user_fk,
            self.episode,
            self.date_watched,
            self.duration,
            self.id
        )
        .fetch_one(&state.db)
        .await?;
        Ok(result)
    }

    pub async fn delete(&self, state: &Data<AppState>) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM history WHERE id = $1", self.id)
            .execute(&state.db)
            .await?;
        Ok(())
    }

    pub async fn count_rows_by_user_id(
        id: i32,
        state: &Data<AppState>,
    ) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!("SELECT count(id) FROM history WHERE user_fk =$1", id)
            .fetch_one(&state.db)
            .await?
            .unwrap_or(0);
        Ok(count)
    }
}
