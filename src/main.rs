pub mod controllers;
pub mod token;
mod validators;
pub mod entity;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use controllers::{auth::{basic_auth, generate_access, create_user}};
use controllers::user_interactions::send_friend_request;
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sqlx::postgres::PgPoolOptions;
use validators::{validator_acces, validator_refresh};

pub struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(50);
    let db = Database::connect(opt).await.expect("Error building a connection pool");

    HttpServer::new(move || {
        let bearer_middleware_refresh = HttpAuthentication::bearer(validator_refresh);
        let bearer_middleware_access = HttpAuthentication::bearer(validator_acces);
        App::new().service(
            web::scope("api/v1")
                .app_data(Data::new(AppState { db: db }))
                .service(web::scope("/auth").service(basic_auth).service(create_user))
                .service(
                    web::scope("/access")
                        .wrap(bearer_middleware_refresh)
                        .service(generate_access),
                )
                .service(
                    web::scope("/interact")
                        .wrap(bearer_middleware_access)
                        // .service(get_friend_requests)
                        .service(send_friend_request),
                ),
        )
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
