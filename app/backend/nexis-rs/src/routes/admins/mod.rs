mod inventory;
mod register_employee;
mod jobs;

use crate::prelude::*;
use types::{ auth::{ Session, AdminSession }, responses, Role };
use actix_web::web;

pub fn admins_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admins")
            .service(inventory::add_inventory)
            .service(register_employee::register_employee)
            .service(jobs::get::get_jobs)
    );
}

pub async fn verify_session(
    req: &HttpRequest,
    db: &mongodb::Database,
    redis_pool: &deadpool_redis::Pool,
) -> Result<AdminSession, HttpResponse> {
    let session = match utils::verify_session(&req, &db, &redis_pool).await {
        Ok(session) => {
            session
        }
        Err(e) => {
            tracing::error!(target: "backend", "{:#?}", e);
            return Err(HttpResponse::Unauthorized().json(responses::Error::detailed("Failed to verify session", e)));
        }
    };

    let admin_data = if let Session::Admin(session) = session.data {
        session
    } else {
        return Err(HttpResponse::Unauthorized().json(responses::RoleRequired::new(Role::Admin)));
    };

    Ok(admin_data)
}