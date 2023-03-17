use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    models::{
        friend_request::{FriendRequest, FriendRequestStatus},
    },
    token::TokenClaims,
    AppState, repo::user_repo::get_user_by_id,
};

#[post("/friend/add/{user_id}")]
pub async fn send_friend_request(
    state: Data<AppState>,
    friend_id: web::Path<i32>,
    credentials: BearerAuth,
) -> impl Responder {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    let sender = get_user_by_id(calims.id, &state).await.unwrap();
    let i = friend_id.into_inner();

    match get_user_by_id(i, &state).await {
        Ok(user) => match FriendRequest::send_friend_request(sender, user, &state).await {
            Ok(request) => HttpResponse::Accepted().json(request),
            Err(error) => match error {
                sqlx::Error::Database(error) => {
                    match error
                        .downcast_ref::<sqlx::postgres::PgDatabaseError>()
                        .code()
                        == "23505"
                    {
                        true => HttpResponse::BadRequest().json("request already sent"),
                        false => HttpResponse::BadRequest().body("wrong user id"),
                    }
                }
                _ => HttpResponse::BadRequest().body("wrong user id"),
            },
        },
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}

#[get("/friend/show")]
pub async fn get_friend_requests(state: Data<AppState>, credentials: BearerAuth) -> impl Responder {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    let sender = get_user_by_id(calims.id, &state).await.unwrap();

    match FriendRequest::get_friend_requests(sender.id, &state).await {
        Ok(requests) => HttpResponse::Accepted().json(requests),
        Err(e) => HttpResponse::BadRequest().body(format!("wrong user id \n{}", e.to_string())),
    }
}

#[post("/friend/change")]
pub async fn change_friend_status(
    state: Data<AppState>,
    request: Json<FriendRequest>,
    credentials: BearerAuth,
) -> impl Responder {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    let sender = get_user_by_id(calims.id, &state)
        .await
        .expect("There IS a valid token, but user wasnt found in database");
    let mut _req = request;
    _req.usr = sender.id;

    if !_req.is_valid(&state).await {
        return HttpResponse::BadRequest().body("Friend Request is not valid");
    }
    if _req.request_status != FriendRequestStatus::Rejected && !_req.can_update_status(&state).await
    {
        return HttpResponse::BadRequest().body("You cant change status to this type.");
    }

    match _req.request_status {
        FriendRequestStatus::Pending => {
            return HttpResponse::BadRequest().body("You cant change status to this type")
        }
        FriendRequestStatus::Rejected => match _req.delete(&state).await {
            true => return HttpResponse::Accepted().body("As you say"),
            false => return HttpResponse::NotFound().body("Cant find this friend"),
        },
        status => return update_status_handler(_req, status, &state).await,
    }
}

async fn update_status_handler(
    request: Json<FriendRequest>,
    status: FriendRequestStatus,
    state: &Data<AppState>,
) -> HttpResponse {
    match request.update_status(status, &state).await {
        Ok(_) => return HttpResponse::Accepted().body("success"),
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!(
                "please, send this error message to developers\n{:?}",
                e
            ))
        }
    }
}
