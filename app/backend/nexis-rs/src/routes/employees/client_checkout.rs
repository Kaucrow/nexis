use crate::prelude::*;
use types::{ requests, responses, Role };
use utils::verify_session;
use handlers::employees;

#[tracing::instrument(
    name = "Accessing online checkout endpoint",
    skip(req, db, redis_pool)
)]
#[actix_web::post("/checkout")]
pub async fn employee_client_checkout(
    req: HttpRequest,
    details: web::Json<requests::employees::Checkout>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    match verify_session(&req, &db, &redis_pool).await {
        Ok(session) => {
            if session.role != Role::Employee {
                return HttpResponse::Unauthorized().json(responses::RoleRequired::new(Role::Employee));
            }
        }
        Err(e) => {
            tracing::error!(target: "backend", "{:#?}", e);
            return HttpResponse::Unauthorized().json(responses::Error::detailed("Failed to verify session: {}", e));
        }
    };

    match employees::checkout::client_checkout(&db, &details.0).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!(target: "backend", "{:#?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}