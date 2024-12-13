use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Checkout {
    pub items: Vec<String>,
    #[serde(rename = "clientName")]
    pub client_name: String,
    pub store: String,
}