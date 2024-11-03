use crate::prelude::*;
use crate::types::{ Role, mongodb::Item, constants::STORE_COLLS };

#[derive(Serialize)]
pub struct Success {
    message: String,
}

impl Success {
    pub fn new(message: &'static str) -> Self {
        Success {
            message: message.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Error {
    error: String,
}

impl Error {
    pub fn new(err_obj: anyhow::Error) -> Self {
        Error {
            error: err_obj.to_string(),
        }
    }

    pub fn simple(error: &'static str) -> Self {
        Error {
            error: error.to_string(),
        }
    }

    pub fn detailed(error: &'static str, err_obj: anyhow::Error) -> Self {
        Error {
            error: format!("{}: {}", error, err_obj),
        }
    }

    pub fn from_str(error: String) -> Self {
        Error {
            error,
        }
    }
}

#[derive(Serialize)]
pub struct RoleRequired {
    pub error: &'static str,
    #[serde(rename = "roleRequired")]
    pub role_required: Role,
}

impl RoleRequired {
    pub fn new(role: Role) -> Self {
        Self {
            error: "You do not have the role required to access this endpoint.",
            role_required: role,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoleSelect {
    #[serde(rename = "availableRoles")]
    pub available_roles: Vec<Role>,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub name: String,
    pub role: Role,
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