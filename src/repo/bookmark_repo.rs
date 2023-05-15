use actix_web::web::Data;

use crate::{models::bookmark::{Bookmark, SimpleBookmark, BookmarkResponse}, AppState};

impl Bookmark {
    pub async fn create_bookmark(
        bookmark: &SimpleBookmark,
        state: &Data<AppState>,
    ) -> Result<Option<Bookmark>, sqlx::Error> {
        let row = sqlx::query_as!(
            Bookmark,
            "INSERT INTO bookmark (user_fk, bookmark_name, release_FK)
            VALUES ($1, $2, $3)
            RETURNING id, user_fk, bookmark_name, release_FK",
            bookmark.user_fk,
            bookmark.bookmark_name,
            bookmark.release_fk
        )
        .fetch_optional(&state.db)
        .await?;
        Ok(row)
    }
    pub async fn insert(&self, state: &Data<AppState>) -> Result<Bookmark, sqlx::Error> {
        let row = sqlx::query_as!(
            Bookmark,
            "INSERT INTO bookmark (user_fk, bookmark_name, release_FK)
            VALUES ($1, $2, $3)
            RETURNING id, user_fk, bookmark_name, release_FK",
            &self.user_fk,
            &self.bookmark_name,
            &self.release_fk
        )
        .fetch_one(&state.db)
        .await?;

        Ok(row)
    }

    // Get a bookmark by ID
    pub async fn get_bookmark_by_id(
        bookmark_id: i32,
        state: &Data<AppState>,
    ) -> Result<Option<Bookmark>, sqlx::Error> {
        let row = sqlx::query_as!(
            Bookmark,
            r#"SELECT id, user_fk, bookmark_name, release_FK
            FROM bookmark
            WHERE id = $1"#,
            bookmark_id
        )
        .fetch_optional(&state.db)
        .await?;

        Ok(row)
    }

    // Get all bookmarks for a user
    pub async fn get_bookmarks_by_user(
        user_id: i32,
        state: &Data<AppState>,
    ) -> Result<Vec<BookmarkResponse>, sqlx::Error> {
        let rows = sqlx::query_as!(
            BookmarkResponse,
            "SELECT b.id, b.user_fk, b.bookmark_name, b.release_FK, r.release_name, r.img, r.rating
                FROM bookmark AS b
                JOIN releases AS r ON b.release_FK = r.id
                WHERE b.user_fk = $1;",
            user_id
        )
        .fetch_all(&state.db)
        .await?;

        Ok(rows)
    }
    pub async fn delete(&self, state: &Data<AppState>) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM bookmark WHERE id = $1", self.id)
            .execute(&state.db)
            .await?;
        Ok(())
    }

    pub async fn delete_by_id(id: i32, state: &Data<AppState>) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM bookmark WHERE id = $1", id)
            .execute(&state.db)
            .await?;
        Ok(())
    }
}
