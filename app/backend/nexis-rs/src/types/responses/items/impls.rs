use crate::prelude::*;
use super::*;
use types::mongodb::{
    Item,
    Clothes,
    LibraryItem,
    Food,
    Tech,
    Gpu,
    Cpu,
    Keyboard,
    TechOther,
};
use async_trait::async_trait;
use std::pin::Pin;
use serde_json::Value;
use serde::de::DeserializeOwned;
use futures_util::Future;
use anyhow::Result;

const ITEM_DE_ERR: &'static str = "An error was produced while retrieving the item: ";

pub static ITEM_DETAILS_REG: Lazy<ItemDetailsRegistry> = Lazy::new(|| {
    let mut registry = ItemDetailsRegistry::new();

    registry.add_fetcher("libraryItem", get_item_fetcher::<LibraryItem>());
    registry.add_fetcher("food", get_item_fetcher::<Food>());

    registry
});

type ItemFetcher = Arc<dyn Fn(Arc<mongodb::Database>, ObjectId) -> Pin<Box<dyn Future<Output = Option<Box<dyn ItemDetails + Send>>> + Send>> + Send + Sync>;

pub struct ItemDetailsRegistry {
    fetchers: HashMap<String, ItemFetcher>,
}

impl ItemDetailsRegistry {
    fn new() -> Self {
        ItemDetailsRegistry {
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

fn get_item_fetcher<T>() -> ItemFetcher
where
    T: ItemDetails + Item + DeserializeOwned + Send + Sync + Unpin + 'static,
{
    Arc::new(|db: Arc<mongodb::Database>, item_id: ObjectId| {
        Box::pin(async move {
            let coll_name = T::coll_name();
            let coll: Collection<T> = db.collection(coll_name);

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
pub trait ItemDetails: Send + Sync
{
    async fn details(&self) -> Result<Value>;
}

impl<'a> From<&'a Clothes> for ClothesDetails<'a> {
    fn from(item: &'a Clothes) -> Self {
        ClothesDetails {
            id: item.id.to_hex(),
            name: &item.name,
            price: item.price,
            age: &item.age,
            size: &item.size,
            color: item.color.iter().map(|s| s.as_str()).collect(),
            clothes_type: &item.clothes_type,
            brand: &item.brand,
            materials: item.materials.iter().map(|material| MaterialDetails {
                name: &material.name,
                percentage: material.percentage,
            }).collect(),
        }
    }
}

#[async_trait]
impl ItemDetails for Clothes {
    async fn details(&self) -> Result<Value> {
        match serde_json::to_value(ClothesDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
    }
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