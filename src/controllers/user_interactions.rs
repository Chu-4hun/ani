use crate::{
    entity::user_friend_requests::Entity as FriendRequest,
    entity::{users::Entity as User, self},
    token::TokenClaims,
    AppState,
};
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

#[post("/friend/add/{user_id}")]
pub async fn send_friend_request(
    state: Data<AppState>,
    friend_id: web::Path<i32>,
    credentials: BearerAuth,
) -> impl Responder {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();

    let sender = User::find_by_id(calims.id)
        .one(&state.db)
        .await
        .unwrap()
        .unwrap();

    // let sender = get_user_by_id(calims.id, &state).await.unwrap();
    let reciever_friend = friend_id.into_inner();
    let friend_req = entity::user_friend_requests::ActiveModel {
        usr: ActiveValue::Set(sender.id),
        friend: ActiveValue::Set(reciever_friend),
        request_status: ActiveValue::Set(0),
    };
    match friend_req.insert(&state.db).await {
        Ok(_) => todo!(),
        Err(error) => match error {
            _ => HttpResponse::BadRequest().body("wrong user id"),
            sea_orm::DbErr::RecordNotInserted => HttpResponse::BadRequest().body("wrong user id"),
        },
    }
}

#[get("/friend/show")]
pub async fn get_friend_requests(state: Data<AppState>, credentials: BearerAuth) -> impl Responder {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    let sender: entity::users::Model = User::find_by_id(calims.id)
        .one(&state.db)
        .await
        .unwrap()
        .unwrap();
    let id = sender.id;

    match FriendRequest::find().filter(entity::user_friend_requests::Column::Usr.eq(id)).all(&state.db).await
    {
        Ok(requests) => HttpResponse::Accepted().json(requests),
        Err(e) => HttpResponse::BadRequest().body(format!("wrong user id \n{}", e.to_string())),
    }
}
