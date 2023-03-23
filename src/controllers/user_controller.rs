use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    controllers::releases_controller::Pagination,
    models::{user_info::*, utils::query_requests::SearchRequest},
    repo::user_repo::{get_user_by_id, get_users_by_simalar_name},
    token::TokenClaims,
    AppState,
};

#[post("/edit")]
pub async fn edit_profile(
    state: Data<AppState>,
    credentials: BearerAuth,
    input_info: Json<UserInfo>,
) -> impl Responder {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();

    let user = match get_user_by_id(calims.id, &state).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().body("Not valid auth user"),
    };
    let current_info = UserInfo::get_by_id(user.id, &state)
        .await
        .expect("DB error");

    match input_info.0.update(current_info.id, &state).await {
        Ok(res) => {
            if res {
                return HttpResponse::Accepted().body("success");
            } else {
                HttpResponse::BadRequest().body("no fields has been affected")
            }
        }
        Err(_) => return HttpResponse::InternalServerError().body("DB error"),
    };

    HttpResponse::InternalServerError().body("Wierd behavior")
}

#[get("/search")]
pub async fn search_profile(
    state: Data<AppState>,
    request: web::Query<SearchRequest>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    match get_users_by_simalar_name(request.0.request.as_str(), pagination.cursor, 40, &state).await
    {
        Ok(res) => {
            return HttpResponse::Accepted().json(res);
        }
        Err(e) => return HttpResponse::NotFound().body(format!("No users was found {}", e)),
    };
}
#[get("/{id}")]
pub async fn get_profile(
    state: Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    match UserInfo::get_by_id(id.into_inner(), &state).await
    {
        Ok(res) => {
            return HttpResponse::Accepted().json(res);
        }
        Err(e) => return HttpResponse::NotFound().body(format!("No users was found {}", e)),
    };
}