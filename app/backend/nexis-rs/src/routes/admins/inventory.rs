use crate::prelude::*;
use super::verify_session;
use types::{ requests, responses, error };

#[tracing::instrument(
    name = "Accessing admin's inventory upload endpoint",
    skip(db, redis_pool, form),
)]
#[actix_web::post("/inventory")]
pub async fn add_inventory(
    req: HttpRequest,
    MultipartForm(form): MultipartForm<requests::UploadInventoryForm>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    let admin_session = match verify_session(&req, &db, &redis_pool).await {
        Ok(session) => session,
        Err(e) => {
            return e;
        }
    };

    if admin_session.stores.contains(&form.store) {
        return HttpResponse::Unauthorized().json(responses::Error::simple("You don't have the permissions required to modify this store's inventory"));
    }

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