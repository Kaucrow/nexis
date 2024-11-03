// src/routes/users/logout.rs
use crate::prelude::*;
use crate::responses;
use types::SSS_COOKIE_NAME;

#[tracing::instrument(name = "Log out user", skip(req, redis_pool))]
#[actix_web::post("/logout")]
pub async fn log_out(
    req: HttpRequest,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing LOGOUT.");
    let sss_pub_token =
        if let Some(sss_pub_cookie) = req.cookie(SSS_COOKIE_NAME) {
            sss_pub_cookie.value().to_string()
        } else {
            return HttpResponse::BadRequest().json(
                responses::Error { error: "Session cookie missing.".to_string() }
            );
        };

    match crate::utils::revoke_session_token(sss_pub_token, &redis_pool).await {
        Ok(_) => {},
        Err(e) if e.is::<types::error::Redis>() => return HttpResponse::InternalServerError().finish(),
        Err(e) => {
            tracing::error!(target: "backend", "An unexpected error occurred: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let clear_cookie = {
        let mut cookie = Cookie::build(SSS_COOKIE_NAME, "")
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