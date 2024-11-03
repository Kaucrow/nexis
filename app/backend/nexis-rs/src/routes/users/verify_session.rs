// src/routes/users/logout.rs
use crate::prelude::*;
use crate::{ responses, types::error };

#[tracing::instrument(name = "Verify user session", skip(redis_pool, db, req))]
#[actix_web::get("/verify-session")]
pub async fn verify_session(
    req: HttpRequest,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>
) -> HttpResponse {
    tracing::info!(target: "backend", "Verifying session.");

    let sss_uuid_token =
        if let Some(sss_uuid_cookie) = req.cookie("session_uuid") {
            sss_uuid_cookie.value().to_string()
        } else {
            return HttpResponse::BadRequest().json(responses::Error::simple("Session cookie missing."));
        };

    match utils::verify_session_token(sss_uuid_token, &db, &redis_pool).await {
        Ok(_) =>
            HttpResponse::Ok().finish(),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<error::Redis>() {
                match e {
                    error::Redis::SessionExpired(e) => {
                        let clear_cookie = {
                            let mut cookie = Cookie::build("session_uuid", "")
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
                    _ => unimplemented!("Unimplemented redis error.")
                }
            } else {
                HttpResponse::Unauthorized().json(responses::Error::detailed("Failed to verify session", e))
            }
        }
    }
}