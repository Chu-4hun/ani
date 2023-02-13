use actix_web::{post, web::{Data, Json}, Responder, HttpResponse};

use crate::{AppState, models::user::User};


#[post("/friend_request")]
async fn create_user(state: Data<AppState>, body: Json<User>) -> impl Responder {


    HttpResponse::Accepted().json(body)
}