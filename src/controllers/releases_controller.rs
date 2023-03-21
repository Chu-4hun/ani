use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::{
    models::releases::{Release, ReleaseWithEpisodes},
    AppState,
};

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
pub async fn get_release(release_id: web::Path<i32>, state: Data<AppState>) -> impl Responder {
    match Release::get_by_id(release_id.into_inner(), &state).await {
        Ok(rel) => {
            //TODO add dub support
            match rel.get_all_dub_options(&state).await {
                Ok(dub) => return HttpResponse::Accepted().json(dub),
                Err(e) => return HttpResponse::Gone().body(format!("{}",e)),
            };
        }
        Err(e) => return HttpResponse::BadRequest().body(format!("{}:?", e)),
    }
}

#[get("/release/{release_id}/{dub_id}")]
pub async fn get_episodes(ids: web::Path<(i32, i32)>, state: Data<AppState>) -> impl Responder {
    let (release_id, dub_id) = ids.into_inner();

    match Release::get_by_id(release_id, &state).await {
        Ok(rel) => match rel.get_all_episodes_of_dub(dub_id, &state).await {
            Ok(episodes) => return HttpResponse::Accepted().json(episodes),
            Err(_) => return HttpResponse::Gone().body("no episodes found"),
        },
        Err(e) => return HttpResponse::BadRequest().body(format!("{}:?", e)),
    }
}
