mod redis;

use actix_web::web;

pub fn debug_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/debug")
        .service(redis::get_redis_session)
        .service(redis::get_redis_roleselect)
    );
}