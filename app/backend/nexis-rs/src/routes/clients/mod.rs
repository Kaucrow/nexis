mod cart;

use actix_web::web;

pub fn client_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clients")
            .service(cart::get_cart_items)
            .service(cart::delete_cart_item)
            .service(cart::insert_cart_item)
    );
}