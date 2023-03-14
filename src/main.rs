pub mod controllers;
pub mod models;
pub mod schema;
pub mod token;
mod validators;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use controllers::user_interactions::send_friend_request;
use controllers::{
    auth::{basic_auth, create_user, generate_access},
    user_interactions::get_friend_requests,
};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use r2d2_postgres::r2d2;
use validators::{validator_acces, validator_refresh};

type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct AppState {
    db: ConnectionPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        let bearer_middleware_refresh = HttpAuthentication::bearer(validator_refresh);
        let bearer_middleware_access = HttpAuthentication::bearer(validator_acces);
        App::new().service(
            web::scope("api/v1")
                .app_data(Data::new(AppState { db: pool.clone() }))
                .service(web::scope("/auth").service(basic_auth).service(create_user))
                .service(
                    web::scope("/access")
                        .wrap(bearer_middleware_refresh)
                        .service(generate_access),
                )
                .service(
                    web::scope("/interact")
                        .wrap(bearer_middleware_access)
                        .service(get_friend_requests)
                        .service(send_friend_request),
                ),
        )
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
