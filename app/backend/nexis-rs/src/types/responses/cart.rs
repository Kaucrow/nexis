use crate::prelude::*;

#[derive(Serialize)]
pub struct CartItem {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub store: String,
    #[serde(rename = "inStock")]
    pub in_stock: bool,
}

#[derive(Serialize)]
pub struct Cart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<Box<Vec<CartItem>>>,
}