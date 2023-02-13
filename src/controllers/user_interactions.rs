use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    models::{friend_request::FriendRequest, user::get_user_by_id},
    token::TokenClaims,
    AppState,
};

#[post("/friend/add/{user_id}")]
pub async fn send_friend_request(
    state: Data<AppState>,
    user_id: web::Path<i32>,
    credentials: BearerAuth,
) -> impl Responder {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    let sender = get_user_by_id(calims.id, &state).await.unwrap();

    match get_user_by_id(*user_id, &state).await {
        Ok(user) => match FriendRequest::send_friend_request(sender, user, &state).await {
            Ok(request) => HttpResponse::Accepted().json(request),
            Err(_) => HttpResponse::BadRequest().body("wrong user id"),
        },
        Err(_) => HttpResponse::BadRequest().body("wrong user id"),
    };
    HttpResponse::Accepted().json(calims.id)
}
