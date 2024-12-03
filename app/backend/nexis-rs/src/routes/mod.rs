mod health;
mod debug;
mod search;
mod users;
mod clients;
mod admins;

pub use health::health_check;
pub use users::auth_routes_config;
pub use debug::debug_routes_config;
pub use search::{ search_suggestions, search_items, search_item_details };
pub use clients::clients_routes_config;
pub use admins::admins_routes_config;