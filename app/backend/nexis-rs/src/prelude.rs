pub use mongodb::{
    self,
    Collection,
    options::{ ClientOptions, ResolverConfig },
    bson::{ self, doc, Document, oid::ObjectId }
};
pub use deadpool_redis::{ self, redis };
pub use serde::{ Serialize, Deserialize };
pub use pasetors::{
    claims::{ Claims, ClaimsValidationRules },
    keys::SymmetricKey,
    token::UntrustedToken,
    local,
    version4::V4,
    Local
};
pub use sqlx::{ PgPool, Row, postgres::PgRow };
pub use actix_web::{
    HttpResponse,
    HttpRequest,
    http,
    web,
    cookie::Cookie
};
pub use anyhow::{ anyhow, bail };
pub use uuid::Uuid;
pub use once_cell::sync::Lazy;
pub use futures_util::TryStreamExt;
pub use std::collections::{ HashMap, HashSet };
pub use crate::{ types, utils };

pub static ITEM_COLLS: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "clothes", "food", "libraryItem", "tech", "techCpu", "techGpu", "techKeyboard", "techOther"
]);