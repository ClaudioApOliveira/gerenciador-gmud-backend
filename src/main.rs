use std::io;

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer};
use mongodb::Database;
use serde_json::json;

mod auth;
mod config;
mod errors;
mod gmud;
mod routes;
mod shared;
mod user;

use config::{db::init_database, AppConfig};
use errors::api_error::ApiError;
use shared::api_response::ok;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub config: AppConfig,
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(ok("servico ativo", json!({ "status": "ok" })))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let config = AppConfig::from_env().map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Erro ao carregar configuracao: {err}"),
        )
    })?;

    let db = init_database(&config)
        .await
        .map_err(|err: ApiError| io::Error::other(err.to_string()))?;

    let state = AppState {
        db,
        config: config.clone(),
    };
    let bind_address = config.bind_address();

    log::info!("API GMUD em http://{bind_address}");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(health)
            .configure(routes::configure_routes)
    })
        .bind(bind_address)?
        .run()
        .await
}
