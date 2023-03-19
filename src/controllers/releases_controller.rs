use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::{models::releases::Release, AppState};

#[derive(Deserialize)]
pub struct Pagination {
    pub cursor: i32,
}

#[get("/releases/popular/show")]
pub async fn get_popular_releases(
    state: Data<AppState>,
    pagination: web::Query<Pagination>,
) -> impl Responder {

    match Release::get_all_by_rating_with_pagination(pagination.cursor, 10, &state).await {
        Ok(rel) => {
            let str = serde_json::to_string(&rel).expect("error");
            return HttpResponse::Accepted().body(str);
        }
        Err(e) => return HttpResponse::BadRequest().body(format!("{}:?", e)),
    }
}
