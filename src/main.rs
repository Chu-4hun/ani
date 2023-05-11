pub mod controllers;
pub mod models;
mod repo;
pub mod token;
mod validators;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use controllers::{
    auth::{basic_auth, create_user, generate_access},
    releases_controller::{get_popular_releases, get_episodes, get_episode_by_id, get_by_episode_id},
    user_interactions::{change_friend_status, get_friend_requests}, user_controller::{search_profile, get_profile}, history_controller::{get_user_history, insert_user_history},
};
use controllers::{
    releases_controller::{get_release, search_releases},
    user_interactions::send_friend_request,
    user_controller::edit_profile,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use validators::{validator_acces, validator_refresh};

pub struct AppState {
    db: Pool<Postgres>,
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
                    web::scope("/watch")
                        .wrap(bearer_middleware_access.clone())
                        .service(search_releases)
                        .service(get_release)
                        .service(get_episodes)
                        .service(get_episode_by_id)
                        .service(get_by_episode_id)
                        .service(get_popular_releases),
                )
                .service(
                    web::scope("/profile")
                        .wrap(bearer_middleware_access.clone())
                        .service(edit_profile)
                        .service(get_profile)
                        .service(search_profile),
                )
                .service(
                    web::scope("/history")
                        .wrap(bearer_middleware_access.clone())
                        .service(get_user_history)
                        .service(insert_user_history),
                )
                .service(
                    web::scope("/interact")
                        .wrap(bearer_middleware_access.clone())
                        .service(get_friend_requests)
                        .service(change_friend_status)
                        .service(send_friend_request),
                ),
        )
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
