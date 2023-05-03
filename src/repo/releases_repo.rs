use actix_web::web::Data;

use crate::{
    models::{episode::Episode, releases::*, dub::Dub},
    AppState,
};

impl Release {
    pub async fn insert(&self, state: &Data<AppState>) -> Result<Release, sqlx::Error> {
        sqlx::query_as::<_, Release>(
            "
        INSERT INTO releases (release_type, release_date, rating, min_age, director, author,studio, description, release_name,img,external_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,$11)
        RETURNING *;
        ",
        )
        .bind(&self.release_type)
        .bind(&self.release_date)
        .bind(&self.rating)
        .bind(&self.min_age)
        .bind(&self.director)
        .bind(&self.author)
        .bind(&self.studio)
        .bind(&self.description)
        .bind(&self.release_name)
        .bind(&self.img)
        .bind(&self.external_id)

        .fetch_one(&state.db)
        .await
    }

    pub async fn get_all_by_rating_with_pagination(
        cursor: i32,
        limit: i32,
        state: &Data<AppState>,
    ) -> Result<Vec<Release>, sqlx::Error> {
        sqlx::query_as::<_, Release>(
            "
        SELECT * FROM releases WHERE id >= $1 ORDER BY rating ASC LIMIT $2",
        )
        .bind(cursor)
        .bind(limit)
        .fetch_all(&state.db)
        .await
    }

    pub async fn get_all_by_simalar_name(
        release_name: &str,
        state: &Data<AppState>,
    ) -> Result<Vec<Release>, sqlx::Error> {
        let user = sqlx::query_as::<_, Release>(
            "
        SELECT *
        FROM releases
        WHERE release_name ILIKE $1
        ",
        )
        .bind(format!("%{}%", release_name))
        .fetch_all(&state.db)
        .await?;
        Ok(user)
    }
    pub async fn get_all_episodes_of_dub(
        &self,
        dub_id:i32,
        state: &Data<AppState>,
    ) -> Result<Vec<Episode>, sqlx::Error> {
        sqlx::query_as::<_, Episode>(
            "
            SELECT episode.id, episode.release_fk, episode.dub_fk, episode.ep_name, episode.url, episode.position
            FROM episode
            INNER JOIN releases ON episode.release_fk = releases.id
            INNER JOIN dub ON episode.dub_fk = dub.id
            WHERE releases.id = $1 AND dub.id = $2
            ORDER BY position ASC;
        ",
        )
        .bind(&self.id)
        .bind(dub_id)
        .fetch_all(&state.db)
        .await
    }

    pub async fn get_all_dub_options(
        &self,
        state: &Data<AppState>,
    ) -> Result<Vec<Dub>, sqlx::Error> {
        sqlx::query_as::<_, Dub>(
            "
            SELECT DISTINCT on (dub.id) dub.id, dub.name
            FROM dub
            INNER JOIN episode ON dub.id = episode.dub_fk
            WHERE episode.release_fk = $1;
        ",
        )
        .bind(&self.id)
        .fetch_all(&state.db)
        .await
    }

    pub async fn get_by_id(id: i32, state: &Data<AppState>) -> Result<Release, sqlx::Error> {
        sqlx::query_as::<_, Release>(
            "
        SELECT *
        FROM releases
        WHERE id = $1;
        ",
        )
        .bind(id)
        .fetch_one(&state.db)
        .await
    }
}
