use crate::prelude::*;
use crate::types::{ mongodb::SimpleItem, constants::STORE_COLLS };

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemSuggestion {
    pub name: String,
    pub coll: String,
}

impl From<SimpleItem> for ItemSuggestion {
    fn from(item: SimpleItem) -> Self {
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

impl From<SimpleItem> for ItemResult {
    fn from(item: SimpleItem) -> Self {
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