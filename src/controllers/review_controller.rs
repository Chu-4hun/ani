use actix_web::{
    get, put,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    models::review::{Review, SimpleReview},
    repo::user_repo::get_user_by_id,
    token::TokenClaims,
    AppState,
};

#[get("/{release_id}/{page}")]
async fn get_reviews(
    release_id: web::Path<(i32,i32)>,
    state: Data<AppState>,
) -> HttpResponse {
    match Review::get_all_by_release(release_id.0, release_id.1, &state).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::BadRequest().body("wrong release id"),
    }
}

#[put("/insert")]
async fn insert_review(
    state: Data<AppState>,
    credentials: BearerAuth,
    request: Json<SimpleReview>,
) -> impl Responder {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    let sender = get_user_by_id(calims.id, &state).await.unwrap();
    match Review::insert(sender.id, request.0, &state).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            HttpResponse::BadRequest().body(format!("check your request. Err: {}", e.to_string()))
        }
    }
}
