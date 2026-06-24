pub mod dtos;
pub mod handlers;
pub mod models;
pub mod services;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/gmuds")
            .service(handlers::create_gmud)
            .service(handlers::list_gmuds)
            .service(handlers::get_gmud_by_id)
            .service(handlers::update_gmud)
            .service(handlers::delete_gmud),
    );
}

