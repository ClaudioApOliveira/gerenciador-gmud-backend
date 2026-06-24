pub mod dtos;
pub mod handlers;
pub mod models;
pub mod services;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(handlers::create_user)
            .service(handlers::list_users),
    );
}

