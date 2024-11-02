mod health;
mod debug;
mod common;
mod users;
mod clients;

pub use health::health_check;
pub use users::auth_routes_config;
pub use clients::client_routes_config;
pub use debug::debug_routes_config;
pub use common::{ search_suggestions, search };