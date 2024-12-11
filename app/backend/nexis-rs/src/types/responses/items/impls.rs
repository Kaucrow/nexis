use crate::prelude::*;
use super::*;
use types::mongodb::{
    IsCollection,
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

    registry.add_fetcher(Clothes::coll_name(), get_item_fetcher::<Clothes>());
    registry.add_fetcher(LibraryItem::coll_name(), get_item_fetcher::<LibraryItem>());
    registry.add_fetcher(Food::coll_name(), get_item_fetcher::<Food>());
    registry.add_fetcher(Cpu::coll_name(), get_item_fetcher::<Cpu>());
    registry.add_fetcher(Gpu::coll_name(), get_item_fetcher::<Gpu>());
    registry.add_fetcher(Keyboard::coll_name(), get_item_fetcher::<Keyboard>());
    registry.add_fetcher(TechOther::coll_name(), get_item_fetcher::<TechOther>());
    registry.add_fetcher(Tech::coll_name(), get_item_fetcher::<Tech>());

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
        coll_name: &str,
        item_id: ObjectId,
    ) -> Option<Box<dyn ItemDetails + Send>> {
        let fetcher = self.fetchers.get(coll_name);

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
    T: ItemDetails + IsCollection + DeserializeOwned + Send + Sync + Unpin + 'static,
{
    Arc::new(|db: Arc<mongodb::Database>, item_id: ObjectId| {
        Box::pin(async move {
            let coll_name = T::coll_name();
            let coll: Collection<T> = db.collection(coll_name);

            let item =
                match coll.find_one(doc! { "_id": item_id }).await {
                    Ok(item) => item?,
                    Err(e) => {
                        tracing::error!(target: "backend", "{}{}", ITEM_DE_ERR, e);
                        return None;
                    }
                };

            Some(Box::new(item) as Box<dyn ItemDetails + Send>)
        })
    })
}

#[async_trait]
pub trait ItemDetails: Send + Sync
{
    async fn details(&self, db: Option<&mongodb::Database>) -> Result<Value>;
}

impl<'a> From<&'a Clothes> for ClothesDetails<'a> {
    fn from(item: &'a Clothes) -> Self {
        Self {
            id: item.id.to_hex(),
            store: &item.store,
            name: &item.name,
            price: item.price,
            age: &item.age,
            size: &item.size,
            gender: &item.gender,
            colors: item.colors.iter().map(|s| s.as_str()).collect(),
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
    async fn details(&self, _: Option<&mongodb::Database>) -> Result<Value> {
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
                    audiences: book.audiences.iter().map(|s| s.as_str()).collect(),
                    genres: book.genres.iter().map(|s| s.as_str()).collect(),
                }))
            } else {
                None
            };

        Self {
            id: item.id.to_hex(),
            store: &item.store,
            name: &item.name,
            price: item.price,
            book,
        } 
    }
}

#[async_trait]
impl ItemDetails for LibraryItem {
    async fn details(&self, _: Option<&mongodb::Database>) -> Result<Value> {
        match serde_json::to_value(LibraryItemDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
    }
}

impl<'a> From<&'a Food> for FoodDetails<'a> {
    fn from(item: &'a Food) -> Self {
        Self {
            id: item.id.to_hex(),
            store: &item.store,
            name: &item.name,
            price_per_kg: item.price_per_kg,
            price: item.price,
            food_type: &item.food_type,
        } 
    }
}

#[async_trait]
impl ItemDetails for Food {
    async fn details(&self, _: Option<&mongodb::Database>) -> Result<Value> {
        match serde_json::to_value(FoodDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
    }
}

impl<'a> From<&'a Cpu> for CpuDetails<'a> {
    fn from(item: &'a Cpu) -> Self {
        Self {
            id: item.id.to_hex(),
            store: &item.store,
            name: &item.name,
            price: item.price,
            brand: &item.brand,
            model: &item.model,
            arch: &item.arch,
            cores: item.cores,
            threads: item.threads,
            socket_type: &item.socket_type,
            overclock_supp: item.overclock_supp,
            memory_supp: MemorySupportedDetails {
                memory_type: &item.memory_supp.memory_type,
                max_size_gb: item.memory_supp.max_size_gb,
            },
            clock: ClockDetails {
                core_speed_ghz: item.clock.core_speed_ghz,
                boost_speed_ghz: item.clock.boost_speed_ghz,
            },
            graphics: &item.graphics,
        }
    }
}

#[async_trait]
impl ItemDetails for Cpu {
    async fn details(&self, _: Option<&mongodb::Database>) -> Result<Value> {
        match serde_json::to_value(CpuDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
    }
}

impl<'a> From<&'a Gpu> for GpuDetails<'a> {
    fn from(item: &'a Gpu) -> Self {
        Self {
            id: item.id.to_hex(),
            store: &item.store,
            name: &item.name,
            price: item.price,
            brand: &item.brand,
            tdp: item.tdp,
            model: &item.model,
            ports: item.ports.iter().map(|s| s.as_str()).collect(),
            memory: MemoryDetails {
                memory_type: &item.memory.memory_type,
                size_gb: item.memory.size_gb,
            },
            clock: ClockDetails {
                core_speed_ghz: item.clock.core_speed_ghz,
                boost_speed_ghz: item.clock.boost_speed_ghz,
            }
        }
    }
}

#[async_trait]
impl ItemDetails for Gpu {
    async fn details(&self, _: Option<&mongodb::Database>) -> Result<Value> {
        match serde_json::to_value(GpuDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
    }
}

impl<'a> From<&'a Keyboard> for KeyboardDetails<'a> {
    fn from(item: &'a Keyboard) -> Self {
        Self {
            id: item.id.to_hex(),
            store: &item.store,
            name: &item.name,
            price: item.price,
            brand: &item.brand,
            model: &item.model,
            keyboard_type: &item.keyboard_type,
            key_switch: &item.key_switch,
            backlight: item.backlight,
            wireless: item.wireless,
            dimensions: DimensionsDetails {
                length: item.dimensions.length,
                width: item.dimensions.width,
                height: item.dimensions.height,
            },
            weight_kg: item.weight_kg,
        }
    }
}

#[async_trait]
impl ItemDetails for Keyboard {
    async fn details(&self, _: Option<&mongodb::Database>) -> Result<Value> {
        match serde_json::to_value(KeyboardDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
    }
}

impl<'a> From<&'a TechOther> for TechOtherDetails<'a> {
    fn from(item: &'a TechOther) -> Self {
        Self {
            id: item.id.to_hex(),
            store: &item.store,
            name: &item.name,
            price: item.price,
        }
    }
}

#[async_trait]
impl ItemDetails for TechOther {
    async fn details(&self, _: Option<&mongodb::Database>) -> Result<Value> {
        match serde_json::to_value(TechOtherDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
    }
}

impl<'a> From<&'a Tech> for TechDetails<'a> {
    fn from(item: &'a Tech) -> Self {
        Self {
            id: item.id.to_hex(),
            store: &item.store,
            name: &item.name,
            price: item.price,
            brand: &item.brand,
            model: &item.model,
            color: &item.color,
            tech_type: &item.tech_type,
            ram: item.ram,
            storage: item.storage, 
            cpu: &item.cpu,
            gpu: item.gpu.as_deref(),
        } 
    }
}

#[async_trait]
impl ItemDetails for Tech {
    async fn details(&self, _: Option<&mongodb::Database>) -> Result<Value> {
        match serde_json::to_value(TechDetails::from(self)) {
            Ok(value) => Ok(value),
            Err(e) => bail!(e),
        }
    }
}