use actix_web::web;

use crate::{auth, gmud, user};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(auth::configure_routes)
            .configure(gmud::configure_routes)
            .configure(user::configure_routes),
    );
}
