pub mod handlers;
pub mod types;
pub mod models;
pub mod services;
pub mod macros;

use actix_web::{web, App, HttpServer};
use crate::handlers::user::data::user_data;
use crate::handlers::user::login::user_login;
use crate::handlers::user::register::user_register;
use crate::services::auth_service::AuthService;
use crate::services::config_service::ConfigService;
use crate::services::db_service::DbService;
use crate::services::hash_service::HashService;
use crate::types::config_provider::JsonConfigProvider;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_provider = JsonConfigProvider::new();

    let config_service = web::Data::new(ConfigService::new(config_provider).await);
    let db_service = web::Data::new(DbService::new(&config_service).await);
    let hash_service = web::Data::new(HashService::new());
    let auth_service = web::Data::new(AuthService::new());

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default().allow_any_header().allow_any_method().allow_any_origin();
        App::new()
            .app_data(config_service.clone())
            .app_data(db_service.clone())
            .app_data(hash_service.clone())
            .app_data(auth_service.clone())
            .service(user_register)
            .service(user_login)
            .service(user_data)
            .wrap(cors)
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
