// src/routes/users/logout.rs
use crate::prelude::*;
use types::{ self, responses, SSS_COOKIE_NAME };

#[tracing::instrument(name = "Verify user session", skip(redis_pool, db, req))]
#[actix_web::get("/verify-session")]
pub async fn verify_session(
    req: HttpRequest,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>
) -> HttpResponse {
    tracing::info!(target: "backend", "Verifying session.");

    match utils::verify_session(&req, &db, &redis_pool).await {
        Ok(_) =>
            HttpResponse::Ok().finish(),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<types::error::Redis>() {
                match e {
                    types::error::Redis::SessionExpired(e) => {
                        let clear_cookie = {
                            let mut cookie = Cookie::build(SSS_COOKIE_NAME, "")
                                .path("/")
                                .http_only(true)
                                .finish();
                            cookie.make_removal();
                            cookie
                        };

                        HttpResponse::Unauthorized()
                        .cookie(clear_cookie)
                        .json(responses::Error::from_str(e.to_string()))
                    }
                    _ => unimplemented!()
                }
            } else {
                HttpResponse::Unauthorized().json(responses::Error::detailed("Failed to verify session", e))
            }
        }
    }
}