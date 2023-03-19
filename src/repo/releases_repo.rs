use actix_web::web::Data;
use chrono::{DateTime, Utc};

use crate::{
    models::{releases::*, episode::Episode},
    AppState,
};

impl Release {
    pub async fn new_insert(
        release_type: ReleaseType,
        release_date: DateTime<Utc>,
        rating: f32,
        min_age: f32,
        director: String,
        author: String,
        studio: String,
        description: String,
        release_name: String,
        state: &Data<AppState>,
    ) -> Result<Release, sqlx::Error> {
        sqlx::query_as::<_, Release>(
            "
        INSERT INTO releases (release_type, release_date, rating, min_age, director, author,studio, description, release_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *;
        ",
        )
        .bind(release_type)
        .bind(release_date)
        .bind(rating)
        .bind(min_age)
        .bind(director)
        .bind(author)
        .bind(studio)
        .bind(description)
        .bind(release_name)
        .fetch_one(&state.db)
        .await
    }
    pub async fn insert(&self, state: &Data<AppState>) -> Result<Release, sqlx::Error> {
        sqlx::query_as::<_, Release>(
            "
        INSERT INTO releases (release_type, release_date, rating, min_age, director, author,studio, description,release_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8,$8)
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
        WHERE release_name LIKE $1
        ",
        )
        .bind(format!("%{}%", release_name))
        .fetch_all(&state.db)
        .await?;
        Ok(user)
    }
    pub async fn get_all_episodes(
        &self,
        state: &Data<AppState>,
    ) -> Result<Vec<Episode>, sqlx::Error> {
        sqlx::query_as::<_, Episode>(
            "
        SELECT e.*
        FROM episode e
        JOIN releases r ON e.release_FK = r.id
        WHERE r.id = $1;
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
