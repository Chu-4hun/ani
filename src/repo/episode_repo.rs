use actix_web::web::Data;

use crate::{models::episode::Episode, AppState};

impl Episode{
    pub async fn insert(&self, state: &Data<AppState>) -> Result<Episode, sqlx::Error> {
        sqlx::query_as::<_, Episode>(
            "
        INSERT INTO episode (release_id, ep_name, url)
        VALUES ($1, $2, $3)
        RETURNING *;
        ",
        )
        .bind(&self.release_id)
        .bind(&self.ep_name)
        .bind(&self.url)

        .fetch_one(&state.db)
        .await
    }
    pub async fn get_by_id(id: i32, state: Data<AppState>) -> Result<Episode, sqlx::Error> {
        sqlx::query_as::<_, Episode>(
            "
        SELECT *
        FROM episode
        WHERE id = $1;
        ",
        )
        .bind(id)
        .fetch_one(&state.db)
        .await
    }
}