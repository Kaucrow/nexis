use types::constants::STORE_COLLS;

use crate::prelude::*;
use crate::types::mongodb::Item;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoleSelect {
    pub roles: Vec<String>,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub email: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<()>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemSuggestion {
    pub name: String,
    pub coll: String,
}

impl From<Item> for ItemSuggestion {
    fn from(item: Item) -> Self {
        ItemSuggestion {
            name: item.name,
            coll: item.coll,
        } 
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemResult {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    name: String,
    price: f64,
    store: String,
    coll: String,
}

impl From<Item> for ItemResult {
    fn from(item: Item) -> Self {
        let mut store = "Unknown";
        for (store_name, colls) in STORE_COLLS.iter() {
            if colls.contains(&item.coll.as_str()) {
                store = store_name;
                break;
            }
        };

        ItemResult {
            id: item.id,
            name: item.name,
            price: item.price,
            store: store.to_string(),
            coll: item.coll,
        } 
    }
}