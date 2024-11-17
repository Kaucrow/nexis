use crate::prelude::*;
use crate::responses;
use crate::utils::{
    get_sss_pub_token,
    verify_session_token,
    database::{
        get_client_cart_details,
        delete_client_cart_item,
        insert_client_cart_item,
    },
};
use types::Role;

#[tracing::instrument(
    name = "Getting client's cart items",
    skip(db, redis_pool, req)
)]
#[actix_web::get("/cart")]
pub async fn get_cart_items(
    req: HttpRequest,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing client cart endpoint.");

    let sss_pub_token = match get_sss_pub_token(req) {
        Ok(token) => token,
        Err(e) => return HttpResponse::BadRequest().json(responses::Error::new(e))
    };

    match verify_session_token(sss_pub_token, &db, &redis_pool).await {
        Ok(session) => {
            if session.role != Role::Client {
                return HttpResponse::Unauthorized().json(responses::RoleRequired::new(Role::Client));
            }

            let uid= session.id;

            let cart: Vec<responses::CartItem> = match get_client_cart_details(&db, uid).await {
                Ok(cart) => cart,
                Err(e) => {
                    if let Some(e) = e.downcast_ref::<types::error::Mongodb>() {
                        if let types::error::Mongodb::CartIsEmpty = e {
                            return HttpResponse::Ok().json(Vec::<()>::new());
                        } else {
                            unimplemented!()
                        }
                    } else {
                        tracing::error!("{}", e);
                        return HttpResponse::InternalServerError().finish();
                    }
                }
            };

            HttpResponse::Ok().json(cart)
        }
        Err(e) => {
            HttpResponse::Unauthorized().json(responses::Error::detailed("Failed to verify session", e))
        }
    }
}

#[derive(Debug, Deserialize)]
struct CartItemUpdateParams {
    #[serde(rename = "item")]
    pub item_id: String,
}

#[tracing::instrument(
    name = "Deleting a client's cart item",
    skip(db, redis_pool, req, params),
    fields(item_id = %params.item_id)
)]
#[actix_web::delete("/cart")]
pub async fn delete_cart_item(
    params: web::Query<CartItemUpdateParams>,
    req: HttpRequest,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing client cart item delete endpoint.");

    let sss_pub_token = match get_sss_pub_token(req) {
        Ok(token) => token,
        Err(e) => return HttpResponse::BadRequest().json(responses::Error::new(e))
    };

    match verify_session_token(sss_pub_token, &db, &redis_pool).await {
        Ok(session) => {
            if session.role != Role::Client {
                return HttpResponse::Unauthorized().json(responses::RoleRequired::new(Role::Client));
            }

            let uid = session.id;
            let item_id = match ObjectId::parse_str(&params.item_id) {
                Ok(oid) => oid,
                Err(_) => return HttpResponse::BadRequest().json(responses::Error::simple("Malformed item id."))
            };

            match delete_client_cart_item(&db, uid, item_id).await {
                Ok(()) => HttpResponse::Ok().finish(),
                Err(e) => {
                    if let Some(e) = e.downcast_ref::<types::error::Mongodb>() {
                        if let types::error::Mongodb::ItemNotInCart = e {
                            return HttpResponse::BadRequest().json(responses::Error::simple("The item was not not found in the cart."))
                        } else {
                            unimplemented!()
                        }
                    } else {
                        tracing::error!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
        }
        Err(e) => {
            HttpResponse::Unauthorized().json(responses::Error::detailed("Failed to verify session", e))
        }
    }
}

#[tracing::instrument(
    name = "Inserting an item into the client's cart",
    skip(db, redis_pool, req, params),
    fields(item_id = %params.item_id)
)]
#[actix_web::post("/cart")]
pub async fn insert_cart_item(
    params: web::Query<CartItemUpdateParams>,
    req: HttpRequest,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing client cart item insert endpoint.");

    let sss_pub_token = match get_sss_pub_token(req) {
        Ok(token) => token,
        Err(e) => return HttpResponse::BadRequest().json(responses::Error::new(e))
    };

    match verify_session_token(sss_pub_token, &db, &redis_pool).await {
        Ok(session) => {
            if session.role != Role::Client {
                return HttpResponse::Unauthorized().json(responses::RoleRequired::new(Role::Client));
            }

            let uid = session.id;
            let item_id = match ObjectId::parse_str(&params.item_id) {
                Ok(oid) => oid,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            match insert_client_cart_item(&db, uid, item_id).await {
                Ok(()) => HttpResponse::Ok().finish(),
                Err(e) => {
                    if let Some(e) = e.downcast_ref::<types::error::Mongodb>() {
                        if let types::error::Mongodb::CartAlreadyHasItem = e {
                            HttpResponse::BadRequest().json(responses::Error::simple("The cart already has this item."))
                        } else {
                            unimplemented!()
                        }
                    } else {
                        tracing::error!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
        }
        Err(e) => {
            HttpResponse::Unauthorized().json(responses::Error::detailed("Failed to verify session", e))
        }
    }
}