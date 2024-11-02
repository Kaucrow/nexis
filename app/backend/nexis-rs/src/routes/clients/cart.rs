use crate::prelude::*;
use types::{ responses, SSS_COOKIE_NAME };

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
                responses::ErrorResponse { error: "Session cookie missing.".to_string() }
            );
        };

    match utils::verify_session_token(sss_uuid_token, &db, &redis_pool).await {
        Ok(session) => {
            HttpResponse::Ok().json({
                responses::SuccessResponse { message: format!("ID: {:#?}, ROLE: {:#?}, SESSION DATA: {:#?}", session.id, session.role, session.data) }
            })
        } 
        Err(e) =>
            HttpResponse::Unauthorized().json(
                responses::ErrorResponse { error: format!("Failed to verify session: {}", e) }
            )
    }
}