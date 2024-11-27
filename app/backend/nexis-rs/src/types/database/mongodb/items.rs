use crate::prelude::*;
use super::{ IsCollection, constants::* };

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleItem {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    pub coll: String,
}

#[derive(Serialize, Debug)]
pub struct ItemSale {
    pub coll: String,
    pub item_id: ObjectId,
    pub lot_id: ObjectId,
    pub code: ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lot {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "enterDate")]
    pub enter_date: DateTime<Utc>,
    pub codes: Vec<ObjectId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    pub percentage: f64,
    pub name: String,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clothes {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    pub age: String,
    pub size: String,
    pub gender: String,
    pub colors: Vec<String>,
    #[serde(rename = "type")]
    pub clothes_type: String,
    pub brand: String,
    pub materials: Vec<Material>,
    pub lots: Vec<Lot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub isbn: String,
    #[serde(rename = "numPages")]
    pub num_pages: i32,
    pub authors: Vec<String>,
    pub publisher: String,
    pub edition: i32,
    pub audiences: Vec<String>,
    pub genres: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryItem {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book: Option<Box<Book>>,
    pub lots: Vec<Lot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Food {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    #[serde(rename = "pricePerKg", skip_serializing_if = "Option::is_none")]
    pub price_per_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "type")]
    pub food_type: String,
    pub lots: Vec<Lot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tech {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    pub brand: String,
    pub model: String,
    pub color: Vec<String>,
    #[serde(rename = "type")]
    pub tech_type: String,
    pub ram: i32,
    pub storage: i32,
    pub cpu: ObjectId,
    pub gpu: Option<ObjectId>,
    pub lots: Vec<Lot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemorySupported {
    #[serde(rename = "type")]
    pub memory_type: String,
    #[serde(rename = "maxSizeGb")]
    pub max_size_gb: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clock {
    #[serde(rename = "coreSpeedGhz")]
    pub core_speed_ghz: f64,
    #[serde(rename = "boostSpeedGhz")]
    pub boost_speed_ghz: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cpu {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    pub brand: String,
    pub model: String,
    pub arch: String,
    pub cores: i32,
    pub threads: i32,
    #[serde(rename = "socketType")]
    pub socket_type: String,
    #[serde(rename = "overclockSupp")]
    pub overclock_supp: bool,
    #[serde(rename = "soldSep")]
    pub sold_sep: bool,
    #[serde(rename = "memorySupp")]
    pub memory_supp: MemorySupported,
    pub clock: Clock,
    pub graphics: String,
    pub lots: Vec<Lot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    #[serde(rename = "type")]
    pub memory_type: String,
    #[serde(rename = "sizeGb")]
    pub size_gb: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gpu {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    pub brand: String,
    pub model: String,
    pub tdp: i32,
    pub ports: Vec<String>,
    pub memory: Memory,
    pub clock: Clock,
    pub lots: Vec<Lot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keyboard {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    pub brand: String,
    pub model: String,
    #[serde(rename = "type")]
    pub keyboard_type: String,
    #[serde(rename = "keySwitch")]
    pub key_switch: String,
    pub backlight: bool,
    pub wireless: bool,
    pub dimensions: Dimensions,
    #[serde(rename = "weightKg")]
    pub weight_kg: f64,
    pub lots: Vec<Lot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TechOther {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    pub lots: Vec<Lot>,
}

impl IsCollection for SimpleItem {
    fn coll_name() -> &'static str { ITEMS_COLL_NAME }
}

impl IsCollection for Clothes {
    fn coll_name() -> &'static str { CLOTHES_COLL_NAME }
}

impl IsCollection for Food {
    fn coll_name() -> &'static str { FOOD_COLL_NAME }
}

impl IsCollection for LibraryItem {
    fn coll_name() -> &'static str { LIBRARYITEMS_COLL_NAME }
}

impl IsCollection for Tech {
    fn coll_name() -> &'static str { TECHOTHER_COLL_NAME }
}

impl IsCollection for Gpu {
    fn coll_name() -> &'static str { GPUS_COLL_NAME }
}

impl IsCollection for Cpu {
    fn coll_name() -> &'static str { CPUS_COLL_NAME }
}

impl IsCollection for Keyboard {
    fn coll_name() -> &'static str { KEYBOARD_COLL_NAME }
}

impl IsCollection for TechOther {
    fn coll_name() -> &'static str { TECHOTHER_COLL_NAME }
}