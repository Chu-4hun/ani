use actix_web::{
    get, put,
    web::{self, Data},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::{basic::BasicAuth, bearer::BearerAuth};
use serde::Deserialize;

use crate::{
    models::{
        episode::Episode,
        history::{self, DBHistory},
    },
    repo::user_repo::get_user_by_id,
    token::TokenClaims,
    AppState,
};

#[get("/{user_id}")]
async fn get_user_history(user_id: web::Path<i32>, state: Data<AppState>) -> HttpResponse {
    let id: i32 = user_id.into_inner();
    //TODO add setting to hide your history
    match DBHistory::get_all_by_user(id, &state).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::BadRequest().body("wrong user id"),
    }
}
#[derive(Deserialize)]
struct HistoryQuery {
    episode_id: i32,
    duration: f64,
}

#[put("/insert")]
async fn insert_user_history(
    state: Data<AppState>,
    credentials: BearerAuth,
    query: web::Query<HistoryQuery>,
) -> impl Responder {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    let sender = get_user_by_id(calims.id, &state).await.unwrap();

    match Episode::get_by_id(query.episode_id, &state).await {
        Ok(result) => {
            match DBHistory::insert_values(sender, result.id, query.duration, &state).await {
                Ok(history) => HttpResponse::Ok().json(history),
                Err(_) => {
                    HttpResponse::InternalServerError().body("History is unable to be written")
                }
            }
            // HttpResponse::Ok().json(history)
        }
        Err(_) => HttpResponse::BadRequest().body("wrong user id"),
    }
}
