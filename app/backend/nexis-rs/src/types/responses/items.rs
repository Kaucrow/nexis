use crate::prelude::*;
use types::mongodb::{
    LibraryItem,
    Food,
};
use async_trait::async_trait;
use std::pin::Pin;
use serde_json::Value;
use futures_util::Future;

const ITEM_DE_ERR: &'static str = "An error was produced while retrieving the item: ";

pub static ITEM_REGISTRY: Lazy<ItemRegistry> = Lazy::new(|| {
    let mut registry = ItemRegistry::new();

    registry.add_fetcher("libraryItem", get_library_item_fetcher());
    registry.add_fetcher("food", get_food_item_fetcher());

    registry
});

type ItemFetcher = Arc<dyn Fn(Arc<mongodb::Database>, ObjectId) -> Pin<Box<dyn Future<Output = Option<Box<dyn ItemDetails + Send>>> + Send>> + Send + Sync>;

pub struct ItemRegistry {
    fetchers: HashMap<String, ItemFetcher>,
}

impl ItemRegistry {
    fn new() -> Self {
        ItemRegistry {
            fetchers: HashMap::new(),
        }
    }

    fn add_fetcher(&mut self, coll_name: &str, fetcher: ItemFetcher) {
        self.fetchers.insert(coll_name.to_string(), fetcher);
    }

    pub async fn get_item_details(
        &self,
        db: &mongodb::Database,
        coll_name: String,
        item_id: ObjectId,
    ) -> Option<Box<dyn ItemDetails + Send>> {
        let fetcher = self.fetchers.get(&coll_name);

        match fetcher {
            Some(fetcher) => fetcher(Arc::new(db.clone()), item_id).await,
            None => {
                tracing::error!(target: "backend", "No fetcher found for collection: `{}`", coll_name);
                None
            }
        }
    }
}

#[async_trait]
pub trait ItemDetails: Send + Sync {
    async fn details(&self) -> Value;
}

#[async_trait]
impl ItemDetails for LibraryItem {
    async fn details(&self) -> Value {
        let mut item_json = serde_json::json!({
            "id": self.id.to_string(),
            "name": self.name,
            "price": self.price,
        });

        if let Some(book) = &self.book {
            item_json["book"] = serde_json::json!(book);
        }

        item_json
    }
}

fn get_library_item_fetcher() -> ItemFetcher {
    Arc::new(|db: Arc<mongodb::Database>, item_id: ObjectId| {
        Box::pin(async move {
            let coll: Collection<LibraryItem> = db.collection("libraryItem");

            let library_item =
                match coll.find_one(doc! { "_id": item_id }).await {
                    Ok(item) => item?,
                    Err(e) => {
                        tracing::error!(target: "backend", "{}{}", ITEM_DE_ERR, e);
                        return None;
                    }
                };

            Some(Box::new(library_item) as Box<dyn ItemDetails + Send>)
        })
    })
}

#[async_trait]
impl ItemDetails for Food {
    async fn details(&self) -> Value {
        let mut item_json = serde_json::json!({
            "id": self.id.to_string(),
            "name": self.name,
            "type": self.food_type,
        });

        if let Some(price) = &self.price {
            item_json["price"] = serde_json::json!(price);
        }

        if let Some(price_per_kg) = &self.price_per_kg {
            item_json["pricePerKg"] = serde_json::json!(price_per_kg);
        }

        item_json
    }
}

fn get_food_item_fetcher() -> ItemFetcher {
    Arc::new(|db: Arc<mongodb::Database>, item_id: ObjectId| {
        Box::pin(async move {
            let coll: Collection<Food> = db.collection("food");

            let food_item =
                match coll.find_one(doc! { "_id": item_id }).await {
                    Ok(item) => item?,
                    Err(e) => {
                        tracing::error!(target: "backend", "{}{}", ITEM_DE_ERR, e);
                        return None;
                    }
                };

            Some(Box::new(food_item) as Box<dyn ItemDetails + Send>)
        })
    })
}