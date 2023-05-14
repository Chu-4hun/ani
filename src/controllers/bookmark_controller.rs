use actix_web::{
    get, post,
    web::{Data, Query},
    HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

use crate::{
    models::bookmark::{Bookmark, SimpleBookmark},
    repo::user_repo::get_user_by_id,
    token::TokenClaims,
    AppState,
};

// Commented code for maybe add feature to check others bookmars
#[get("/")]
async fn get_bookmarks_by_user(
    // user_id: web::Path<i32>,
    credentials: BearerAuth,
    state: Data<AppState>,
) -> HttpResponse {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    let sender = get_user_by_id(calims.id, &state).await.unwrap();
    // let id = user_id.into_inner();
    // if sender.id != id {
    //     return HttpResponse::NotAcceptable().body("You cant watch others bookmarks");
    // }
    match Bookmark::get_bookmarks_by_user(sender.id, &state).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::BadRequest().body("wrong release id"),
    }
}
#[derive(Deserialize)]
pub struct BookmarkRequest {
    pub release_id: i32,
    pub name:String,

}

#[post("/add")]
async fn add_to_bookmarks(
    params: Query<BookmarkRequest>,
    credentials: BearerAuth,
    state: Data<AppState>,
) -> HttpResponse {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    let sender = get_user_by_id(calims.id, &state).await.unwrap();

    match Bookmark::create_bookmark(
        &SimpleBookmark {
            bookmark_name: params.name.to_owned(),
            user_fk: sender.id,
            release_fk: params.release_id,
        },
        &state,
    )
    .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::BadRequest().body(format!("Error {}", e.to_string())),
    }
}
