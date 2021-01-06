use actix_web::{web};

pub mod user_routes;
pub mod auth_routes;
pub mod group_routes;
pub mod job_routes;
pub mod test_routes;

pub fn init(cfg: &mut web::ServiceConfig) {
    user_routes::init(cfg);
    auth_routes::init(cfg);
    group_routes::init(cfg);
    job_routes::init(cfg);
    test_routes::init(cfg);
}
