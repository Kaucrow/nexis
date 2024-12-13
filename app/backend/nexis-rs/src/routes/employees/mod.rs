mod client_checkout;

use crate::prelude::*;

pub fn employees_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/employees")
            .service(client_checkout::employee_client_checkout)
    );
}