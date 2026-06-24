pub mod dtos;
pub mod extractor;
pub mod handlers;
pub mod services;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(handlers::login)
            .service(handlers::refresh)
            .service(handlers::logout)
            .service(handlers::me),
    );
}

