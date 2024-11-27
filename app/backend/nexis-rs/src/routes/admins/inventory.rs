use crate::prelude::*;
use types::{ requests, responses, error };

#[tracing::instrument(
    name = "Accessing admin's inventory upload endpoint",
    skip(db, redis_pool, form),
)]
#[actix_web::post("/inventory")]
pub async fn add_inventory(
    MultipartForm(form): MultipartForm<requests::UploadInventoryForm>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    match handlers::inventory::add::add_inventory(form, &db).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!(target: "backend", "Could not add inventory: {}", e.to_string());
            if let Ok(csv_err) = e.downcast::<error::Csv>() {
                HttpResponse::BadRequest().json(responses::Error::new(csv_err.into()))
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}