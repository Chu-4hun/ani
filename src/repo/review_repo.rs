use actix_web::web::Data;

use crate::{models::review::{Review, SimpleReview}, AppState};

impl Review {
    pub async fn insert(user_fk:i32,request: SimpleReview, state: &Data<AppState>) -> Result<Review, sqlx::Error> {
        let result = sqlx::query_as!(
            Review,
            "INSERT INTO review (user_fk, review_text , rating, release_fk)
              VALUES ($1, $2, $3, $4) RETURNING id, user_fk, review_text, rev_data, rating, release_fk",
            user_fk,
            request.review_text,
            request.rating,
            request.release_fk
        )
        .fetch_one(&state.db)
        .await?;
        Ok(result)
    }

    pub async fn find_by_id(
        id: i32,
        state: &Data<AppState>,
    ) -> Result<Option<Review>, sqlx::Error> {
        let review = sqlx::query_as!(
            Review,
            r#"
            SELECT * FROM review
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&state.db)
        .await?;
        Ok(review)
    }

    pub async fn update(&self, state: &Data<AppState>) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            Review,
            "
            UPDATE review
            SET user_fk = $2, review_text = $3, rev_data = $4, rating = $5, release_fk = $6
            WHERE id = $1
            ",
            self.id,
            self.user_fk,
            self.review_text,
            self.rev_data,
            self.rating,
            self.release_fk
        )
        .execute(&state.db)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, state: &Data<AppState>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            DELETE FROM review
            WHERE id = $1
            ",
        )
        .bind(&self.id)
        .execute(&state.db)
        .await?;
        Ok(())
    }
    pub async fn get_all_by_release(
        release_id: i32,
        page: i32,
        state: &Data<AppState>,
    ) -> Result<Vec<Review>, sqlx::Error> {
        let reviews = sqlx::query_as!(
            Review,
            "SELECT review.*
            FROM review
            INNER JOIN releases ON review.release_FK = releases.id
            WHERE releases.id = $1
            ORDER BY review.rating DESC
            LIMIT 10
            OFFSET $2;",
            release_id,
            page as i64 * 10
        )
        .fetch_all(&state.db)
        .await?;
        Ok(reviews)
    }
}
