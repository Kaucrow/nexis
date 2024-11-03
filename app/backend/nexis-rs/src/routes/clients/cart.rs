use crate::prelude::*;
use crate::responses;
use crate::utils::database::get_client_cart_details;
use types::{ SSS_COOKIE_NAME, Role };

#[tracing::instrument(
    name = "Activating a new user",
    skip(db, redis_pool, req)
)]
#[actix_web::get("/cart-items")]
pub async fn get_cart_items(
    req: HttpRequest,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing client cart endpoint.");

    let sss_uuid_token =
        if let Some(sss_uuid_cookie) = req.cookie(SSS_COOKIE_NAME) {
            sss_uuid_cookie.value().to_string()
        } else {
            return HttpResponse::BadRequest().json(
                responses::Error { error: "Session cookie missing.".to_string() }
            );
        };

    match utils::verify_session_token(sss_uuid_token, &db, &redis_pool).await {
        Ok(session) => {
            if session.role != Role::Client {
                return HttpResponse::Unauthorized().json(responses::RoleRequired::new(Role::Client));
            }

            let uid= session.id;

            let cart = match get_client_cart_details(&db, uid).await {
                Ok(cart) => cart,
                Err(e) => {
                    tracing::error!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };

            HttpResponse::Ok().json(cart)
        } 
        Err(e) =>
            HttpResponse::Unauthorized().json(
                responses::Error { error: format!("Failed to verify session: {}", e) }
            )
    }
}