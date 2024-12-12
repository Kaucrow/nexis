mod cart;
mod checkout;
mod register;

use actix_web::web;

pub fn clients_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clients")
            .service(register::register_client)
            .service(cart::get_cart_items)
            .service(cart::delete_cart_item)
            .service(cart::insert_cart_item)
            .service(checkout::checkout)
    );
}