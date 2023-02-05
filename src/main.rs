pub mod auth;
pub mod models;

use actix_web::{
    dev::ServiceRequest,
    web::{self, Data},
    App, Error, HttpMessage, HttpServer,
};
use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};

use auth::{create_user, root, basic_auth};
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id: i32,
}
pub struct AppState {
    db: Pool<Postgres>,
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(validator);
        App::new()
        // .service(
            // web::scope("api/v1/")
                .app_data(Data::new(AppState { db: pool.clone() }))
                .service(basic_auth)
                .service(create_user)
                .service(root)
        // )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
