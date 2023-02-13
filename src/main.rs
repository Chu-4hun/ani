pub mod controllers;
pub mod models;
pub mod token;
mod validators;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::{
    middleware::HttpAuthentication,
};
use controllers::auth::{basic_auth, create_user, root};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use validators::{ validator_refresh, validator_acces};

pub struct AppState { db: Pool<Postgres>,}



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
        let bearer_middleware_refresh = HttpAuthentication::bearer(validator_refresh);
        let bearer_middleware_access = HttpAuthentication::bearer(validator_acces);
        App::new()
            .service(
                web::scope("api/v1/auth")
                    .app_data(Data::new(AppState { db: pool.clone() }))
                    .service(basic_auth)
                    .service(create_user),
            )
            .service(
                web::scope("api/v1")
                    .app_data(Data::new(AppState { db: pool.clone() }))
                    .wrap(bearer_middleware_access) // .service(create_article),
                    .service(root),
            )
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
