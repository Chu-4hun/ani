use crate::{
    token::{TokenClaims},
    AppState,
};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;

use sqlx;
use crate::models::user::*;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[post("/register")]
async fn create_user(state: Data<AppState>, body: Json<User>) -> impl Responder {
    let user: User = body.into_inner();
    let hash = Argon2::default()
        .hash_password(user.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    match sqlx::query_as::<_, UserNoPassword>(
        "
        INSERT INTO users (user_name, password, email)
        VALUES ($1, $2, $3)
        RETURNING user_id, user_name;
        ",
    )
    .bind(user.user_name)
    .bind(hash)
    .bind(user.email)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => match error {
            sqlx::Error::Database(error) => {
                match error
                    .downcast_ref::<sqlx::postgres::PgDatabaseError>()
                    .code()
                    == "23505"
                {
                    // AAAAAA 1.5 hours to downcast_ref::<sqlx::postgres::PgDatabaseError>
                    true => HttpResponse::BadRequest().json("username or email already claimed"),
                    false => HttpResponse::InternalServerError().json(format!("{:?}", error)),
                }
            }
            _ => HttpResponse::InternalServerError().json(format!("{:?}", error)),
        },
    }
}
#[get("/")]
async fn root() -> HttpResponse {
    HttpResponse::Ok().body("hi")
}

#[get("/login")]
async fn basic_auth(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    
    let user_name = credentials.user_id();
    let pass = credentials.password();

    match pass {
        None => HttpResponse::Unauthorized().json("Must provide user_name and password"),
        Some(pass) => {
            match sqlx::query_as::<_, DbUser>(
                "SELECT user_id, user_name,email, password FROM users WHERE user_name = $1",
            )
            .bind(user_name.to_string())
            .fetch_one(&state.db)
            .await
            {
                Ok(user) => {
                    let parsed_hash = PasswordHash::new(&user.password).unwrap();
                    let is_valid = Argon2::default()
                        .verify_password(pass.as_bytes(), &parsed_hash)
                        .is_ok();
                    if is_valid {
                        HttpResponse::Ok().json(TokenClaims::generate_access(user.user_id))
                    } else {
                        HttpResponse::Unauthorized().json("Incorrect user_name or password")
                    }
                }
                Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
            }
        }
    }
}
