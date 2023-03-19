use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::{models::releases::{Release, ReleaseWithEpisodes}, AppState};

#[derive(Deserialize)]
pub struct Pagination {
    pub cursor: i32,
}

#[derive(Deserialize)]
pub struct SearchRequest {
    pub request: String,
}

#[get("/releases/popular/show")]
pub async fn get_popular_releases(
    state: Data<AppState>,
    pagination: web::Query<Pagination>,
) -> impl Responder {

    match Release::get_all_by_rating_with_pagination(pagination.cursor, 10, &state).await {
        Ok(rel) => {
            return HttpResponse::Accepted().json(rel);
        }
        Err(e) => return HttpResponse::BadRequest().body(format!("{}:?", e)),
    }
}

#[get("/releases/search")]
pub async fn search_releases(
    state: Data<AppState>,
    request: web::Query<SearchRequest>,
) -> impl Responder {
    match Release::get_all_by_simalar_name(request.request.as_str(), &state).await {
        Ok(rel) => {
            return HttpResponse::Accepted().json(rel);
        }
        Err(e) => return HttpResponse::BadRequest().body(format!("{}:?", e)),
    }
}

#[get("/release/{release_id}")]
pub async fn get_release(
    release_id: web::Path<i32>,
    state: Data<AppState>,
) -> impl Responder {
    match Release::get_by_id(release_id.into_inner(), &state).await {
        Ok(rel) => {
            let episodes = rel.get_all_episodes(&state).await.unwrap_or(vec![]);
            return HttpResponse::Accepted().json(ReleaseWithEpisodes{release: rel, episodes});
        }
        Err(e) => return HttpResponse::BadRequest().body(format!("{}:?", e)),
    }
}

