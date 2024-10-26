// src/routes/users/logout.rs
use crate::prelude::*;

#[tracing::instrument(name = "Log out user", skip(req, redis_pool))]
#[actix_web::post("/logout")]
pub async fn log_out(
    req: HttpRequest,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing LOGOUT.");
    let sss_uuid_token =
        if let Some(sss_uuid_cookie) = req.cookie("session_uuid") {
            sss_uuid_cookie.value().to_string()
        } else {
            return HttpResponse::BadRequest().json(
                crate::types::ErrorResponse { error: "Session cookie missing.".to_string() }
            );
        };

    crate::utils::revoke_session_token(sss_uuid_token, &redis_pool).await.expect("Failed to revoke the session token");

    let clear_cookie = {
        let mut cookie = Cookie::build("session_uuid", "")
        .path("/")
        .http_only(true)
        .finish();
        cookie.make_removal();
        cookie
    };

    HttpResponse::Ok()
        .cookie(clear_cookie)
        .finish()
}