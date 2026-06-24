use std::io;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, get, http::header, middleware::Logger, web::Data};
use mongodb::Database;
use serde_json::json;

mod auth;
mod config;
mod errors;
mod gmud;
mod routes;
mod shared;
mod user;

use config::{AppConfig, db::init_database};
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

fn is_allowed_origin(origin: &str, configured_origins: &[String]) -> bool {
    configured_origins.iter().any(|allowed| allowed == origin)
        || origin.contains("://localhost")
        || origin.contains("://127.0.0.1")
        || origin.ends_with(".sslip.io")
}

fn build_cors(config: &AppConfig) -> Cors {
    let configured_origins = config.cors_allowed_origins.clone();

    Cors::default()
        .supports_credentials()
        .allowed_origin_fn(move |origin, _request_head| {
            origin
                .to_str()
                .map(|origin| is_allowed_origin(origin, &configured_origins))
                .unwrap_or(false)
        })
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .max_age(3600)
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
    let cors_config = config.clone();

    log::info!("API GMUD em http://{bind_address}");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(build_cors(&cors_config))
            .service(health)
            .configure(routes::configure_routes)
    })
    .bind(bind_address)?
    .run()
    .await
}
