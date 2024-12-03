use crate::prelude::*;
use types::{ requests, responses, error, Role };
use handlers::checkout::{ client_checkout, client_cart_checkout };
use utils::verify_session;

#[tracing::instrument(
    name = "Accessing checkout endpoint",
    skip(req, db, redis_pool)
)]
#[actix_web::post("/checkout")]
pub async fn checkout(
    req: HttpRequest,
    details: web::Json<requests::clients::Checkout>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    match verify_session(&req, &db, &redis_pool).await {
        Ok(session) => {
            if session.role != Role::Client {
                return HttpResponse::Unauthorized().json(responses::RoleRequired::new(Role::Client));
            }

            let checkout_res = if details.cart {
                client_cart_checkout(&db, session.id, &details.items).await
            } else {
                client_checkout(&db, session.id, &details.items).await
            };

            match checkout_res {
                Ok(()) => HttpResponse::Ok().finish(),
                Err(e) => {
                    if let Some(e) = e.downcast_ref::<error::Mongodb>() {
                        match e {
                            &error::Mongodb::ItemSoldOut => {
                                tracing::error!(target: "backend", "{}", e);
                                HttpResponse::BadRequest().json(responses::Error::from_str(e.to_string()))
                            }
                            _ => unimplemented!()
                        }
                    } else {
                        HttpResponse::InternalServerError().json(responses::Error::simple("Checkout failed"))
                    }
                }
            }
        }
        Err(e) => {
            tracing::error!(target: "backend", "Failed to verify session: {}", e);
            HttpResponse::Unauthorized().json(responses::Error::simple("Failed to verify session"))
        }
    }
}