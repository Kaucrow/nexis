// src/routes/users/logout.rs
use crate::prelude::*;
use crate::types::ErrorResponse;

#[tracing::instrument(name = "Verify user session", skip(redis_pool, req))]
#[actix_web::get("/verify-session")]
pub async fn verify_session(
    req: HttpRequest,
    redis_pool: web::Data<deadpool_redis::Pool>
) -> HttpResponse {
    tracing::info!(target: "backend", "Verifying session");

    let sss_uuid_token =
        if let Some(sss_uuid_cookie) = req.cookie("session_uuid") {
            sss_uuid_cookie.value().to_string()
        } else {
            return HttpResponse::BadRequest().json(
                ErrorResponse { error: "Session cookie missing.".to_string() }
            );
        };

    match crate::utils::verify_session_token(sss_uuid_token, &redis_pool).await {
        Ok(_) =>
            HttpResponse::Ok().finish(),
        Err(e) =>
            HttpResponse::Unauthorized().json(
                ErrorResponse { error: format!("Failed to verify session: {}", e) }
            )
    }
}