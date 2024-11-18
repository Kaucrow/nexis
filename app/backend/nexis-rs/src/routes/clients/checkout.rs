use crate::prelude::*;
use types::requests;
use utils::database::checkout::client_checkout;
use utils::database::items::{
    AvailableItem,
    get_oldest_item_available,
};

#[tracing::instrument(
    name = "Getting client's cart items",
    skip(db, redis_pool)
)]
#[actix_web::post("/checkout")]
pub async fn checkout(
    details: web::Json<requests::clients::Checkout>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    if details.cart {
        HttpResponse::Ok().finish()
    } else {
        match client_checkout(&db, ObjectId::new(), &details.items).await {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(e) => {
                tracing::error!(target: "backend", "{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}