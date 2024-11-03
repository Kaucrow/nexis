// src/routes/users/logout.rs
use crate::prelude::*;
use crate::responses;
use types::SSS_COOKIE_NAME;
use utils::get_sss_pub_token;

#[tracing::instrument(name = "Log out user", skip(req, redis_pool))]
#[actix_web::post("/logout")]
pub async fn log_out(
    req: HttpRequest,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing LOGOUT.");

    let sss_pub_token = match get_sss_pub_token(req) {
        Ok(token) => token,
        Err(e) => return HttpResponse::BadRequest().json(responses::Error::new(e))
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