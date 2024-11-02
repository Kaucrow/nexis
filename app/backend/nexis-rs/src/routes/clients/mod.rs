mod cart;

use actix_web::web;

pub fn client_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clients")
            .service(cart::get_cart_items)
    );
}