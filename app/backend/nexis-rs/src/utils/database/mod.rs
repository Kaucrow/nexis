pub mod users;
pub mod search;
pub mod checkout;
pub mod items;

pub use users::{
    insert_created_user_into_db,
    get_client_cart_details,
    delete_client_cart_item,
    insert_client_cart_item,
    get_user,
};

use crate::prelude::*;
use anyhow::Result;

pub async fn get_redis_conn(redis_pool: &deadpool_redis::Pool) -> Result<deadpool_redis::Connection> {
    match redis_pool.get().await {
        Ok(redis_conn) => 
            Ok(redis_conn),
        Err(e) => {
            tracing::error!(target: "redis", "Redis connection could not be established: {}", e);
            Err(anyhow!("Redis connection could not be established."))
        }
    }
}