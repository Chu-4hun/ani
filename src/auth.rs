use crate::{AppState, TokenClaims};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use sqlx;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::models::user::*;

#[post("/user")]
async fn create_user(state: Data<AppState>, body: Json<User>) -> impl Responder {
    let user: User = body.into_inner();
    let hash = Argon2::default()
        .hash_password(user.pass.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    match sqlx::query_as::<_, User>(
        "INSERT INTO users (user_name, pass, email)
        VALUES ($1, $2, $3)
        RETURNING user_id, user_name",
    )
    .bind(user.user_name)
    .bind(hash)
    .bind(user.email)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}
#[get("/")]
async fn root() -> HttpResponse {
    HttpResponse::Ok().body("hi")
}

#[get("/auth")]
async fn basic_auth(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .unwrap();
    let user_name = credentials.user_id();
    let pass = credentials.password();

    match pass {
        None => HttpResponse::Unauthorized().json("Must provide user_name and password"),
        Some(pass) => {
            match sqlx::query_as::<_, DbUser>(
                "SELECT user_id, user_name, pass FROM users WHERE user_name = $1",
            )
            .bind(user_name.to_string())
            .fetch_one(&state.db)
            .await
            {
                Ok(user) => {
                    let parsed_hash = PasswordHash::new(&user.pass).unwrap();
                    let is_valid = Argon2::default()
                        .verify_password(pass.as_bytes(), &parsed_hash)
                        .is_ok();
                    if is_valid {
                        let claims = TokenClaims { id: user.id };
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(token_str)
                    } else {
                        HttpResponse::Unauthorized().json("Incorrect user_name or password")
                    }
                }
                Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
            }
        }
    }
}
