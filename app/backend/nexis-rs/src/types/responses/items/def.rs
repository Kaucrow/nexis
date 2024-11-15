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

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialDetails<'a> {
    pub percentage: f64,
    pub name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClothesDetails<'a> {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: &'a str,
    pub price: f64,
    pub age: &'a str,
    pub size: &'a str,
    pub color: Vec<&'a str>,
    #[serde(rename = "type")]
    pub clothes_type: &'a str,
    pub brand: &'a str,
    pub materials: Vec<MaterialDetails<'a>>, 
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
pub struct LibraryItemDetails<'a> {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: &'a str,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book: Option<Box<BookDetails<'a>>>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TechDetails<'a> {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: &'a str,
    pub price: f64,
    pub brand: &'a str,
    pub model: &'a str,
    pub color: Vec<&'a str>,
    #[serde(rename = "type")]
    pub tech_type: &'a str,
    pub memory: i32,
    pub cpu: CpuDetailsOwned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<GpuDetailsOwned>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemorySupportedDetails<'a> {
    #[serde(rename = "type")]
    pub memory_type: &'a str,
    #[serde(rename = "maxSizeGb")]
    pub max_size_gb: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemorySupportedDetailsOwned {
    #[serde(rename = "type")]
    pub memory_type: String,
    #[serde(rename = "maxSizeGb")]
    pub max_size_gb: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ClockDetails {
    #[serde(rename = "coreSpeedGhz")]
    pub core_speed_ghz: f64,
    #[serde(rename = "boostSpeedGhz")]
    pub boost_speed_ghz: f64,
}

#[derive(Serialize, Debug)]
pub struct CpuDetails<'a> {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: &'a str,
    pub price: f64,
    pub brand: &'a str,
    pub model: &'a str,
    pub arch: &'a str,
    pub cores: i32,
    pub threads: i32,
    #[serde(rename = "socketType")]
    pub socket_type: &'a str,
    #[serde(rename = "overclockSupp")]
    pub overclock_supp: bool,
    pub memory_supp: MemorySupportedDetails<'a>,
    pub clock: ClockDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuDetailsOwned {
    #[serde(rename = "_id")]
    pub id: String,
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
    pub memory_supp: MemorySupportedDetailsOwned,
    pub clock: ClockDetails,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryDetails<'a> {
    #[serde(rename = "type")]
    pub memory_type: &'a str,
    #[serde(rename = "sizeGb")]
    pub size_gb: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryDetailsOwned {
    #[serde(rename = "type")]
    pub memory_type: String,
    #[serde(rename = "sizeGb")]
    pub size_gb: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GpuDetails<'a> {
    #[serde(rename = "_id")]
    pub id: String, 
    pub name: &'a str,
    pub price: f64,
    pub brand: &'a str,
    pub model: &'a str,
    pub tdp: i32,
    pub ports: Vec<&'a str>,
    pub dedicated: bool,
    pub memory: MemoryDetails<'a>,
    pub clock: ClockDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GpuDetailsOwned {
    #[serde(rename = "_id")]
    pub id: String, 
    pub name: String,
    pub price: f64,
    pub brand: String,
    pub model: String,
    pub tdp: i32,
    pub ports: Vec<String>,
    pub dedicated: bool,
    pub memory: MemoryDetailsOwned,
    pub clock: ClockDetails,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DimensionsDetails {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardDetails<'a> {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: &'a str,
    pub price: f64,
    pub brand: &'a str,
    pub model: &'a str,
    #[serde(rename = "type")]
    pub keyboard_type: &'a str,
    #[serde(rename = "keySwitch")]
    pub key_switch: &'a str,
    pub backlight: bool,
    pub wireless: bool,
    pub dimensions: DimensionsDetails,
    #[serde(rename = "weightKg")]
    pub weight_kg: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TechOtherDetails<'a> {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: &'a str,
    pub price: f64,
}