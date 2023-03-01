use crate::{token::TokenClaims, AppState};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::{basic::BasicAuth, bearer::BearerAuth};

use crate::models::user::*;
use sqlx;

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

    if !user_is_unique(&user.login, &user.email, &state).await.unwrap() {
        return HttpResponse::BadRequest().json("username or email already claimed");
    }

    //TODO move this user model
    match sqlx::query_as::<_, UserNoPassword>(
        "
        INSERT INTO users (login, password, email)
        VALUES ($1, $2, $3)
        RETURNING id, login;
        ",
    )
    .bind(user.login)
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
async fn generate_access(credentials: BearerAuth) -> HttpResponse {
    let calims = TokenClaims::get_token_claims(credentials.token()).unwrap();
    HttpResponse::Ok().json(TokenClaims::generate_access(calims.id))
}

#[get("/login")]
async fn basic_auth(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let login = credentials.user_id();
    let pass = credentials.password();

    match pass {
        None => HttpResponse::Unauthorized().json("Must provide user_name and password"),
        Some(pass) => match get_user_by_name(login, state).await {
            Ok(user) => {
                let parsed_hash = PasswordHash::new(&user.password).unwrap();
                let is_valid = Argon2::default()
                    .verify_password(pass.as_bytes(), &parsed_hash)
                    .is_ok();
                if is_valid {
                    HttpResponse::Ok().json(TokenClaims::generate_refresh(user.id))
                } else {
                    HttpResponse::Unauthorized().json("Incorrect user_name or password")
                }
            }
            Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
        },
    }
}
