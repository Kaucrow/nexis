use crate::prelude::*;
use utils::verify_session;
use types::{ requests, responses, error, Role, auth::Session };

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
    let session = match verify_session(&req, &db, &redis_pool).await {
        Ok(session) => {
            session
        }
        Err(e) => {
            tracing::error!(target: "backend", "{:#?}", e);
            return HttpResponse::Unauthorized().json(responses::Error::detailed("Failed to verify session", e));
        }
    };

    if let Session::Admin(session) = session.data {
        if !session.stores.contains(&form.store) {
            return HttpResponse::Unauthorized().json(responses::Error::simple("You don't have the permission to modify this store's inventory"));
        }
    } else {
        return HttpResponse::Unauthorized().json(responses::RoleRequired::new(Role::Admin));
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