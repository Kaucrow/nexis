mod inventory;

use actix_web::web;

pub fn admins_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admins")
            .service(inventory::add_inventory)
    );
}