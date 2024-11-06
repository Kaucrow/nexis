use crate::prelude::*;
use types::mongodb::{
    LibraryItem,
    Food,
};
use async_trait::async_trait;
use std::pin::Pin;
use serde_json::Value;
use futures_util::Future;
use anyhow::Result;

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
    async fn details(&self) -> Result<Value>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookDetails<'a> {
    pub isbn: &'a str,
    #[serde(rename = "numPages")]
    pub num_pages: i32,
    pub authors: Vec<&'a str>,
    pub publisher: &'a str,
    pub edition: i32,
    pub audience: Vec<&'a str>,
    pub genre: Vec<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LibraryItemDetails<'a> {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: &'a str,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book: Option<Box<BookDetails<'a>>>,
}

impl<'a> From<&'a LibraryItem> for LibraryItemDetails<'a> {
    fn from(item: &'a LibraryItem) -> Self {
        let book =
            if let Some(book) = &item.book {
                Some(Box::new(BookDetails {
                    isbn: &book.isbn,
                    num_pages: book.num_pages,
                    authors: book.authors.iter().map(|s| s.as_str()).collect(),
                    publisher: &book.publisher,
                    edition: book.edition,
                    audience: book.audience.iter().map(|s| s.as_str()).collect(),
                    genre: book.genre.iter().map(|s| s.as_str()).collect(),
                }))
            } else {
                None
            };

        LibraryItemDetails {
            id: item.id.to_hex(),
            name: &item.name,
            price: item.price,
            book,
        } 
    }
}

#[async_trait]
impl ItemDetails for LibraryItem {
    async fn details(&self) -> Result<Value> {
        match serde_json::to_value(LibraryItemDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct FoodDetails<'a> {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: &'a str,
    #[serde(rename = "pricePerKg", skip_serializing_if = "Option::is_none")]
    pub price_per_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "type")]
    pub food_type: &'a str,
}

impl<'a> From<&'a Food> for FoodDetails<'a> {
    fn from(item: &'a Food) -> Self {
        FoodDetails {
            id: item.id.to_hex(),
            name: &item.name,
            price_per_kg: item.price_per_kg,
            price: item.price,
            food_type: &item.food_type,
        } 
    }
}

#[async_trait]
impl ItemDetails for Food {
    async fn details(&self) -> Result<Value> {
        match serde_json::to_value(FoodDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
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