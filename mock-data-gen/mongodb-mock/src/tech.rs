use crate::common::*;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use rand::prelude::IteratorRandom;

static CPU_BRAND_MODEL: Lazy<HashMap<&str, Vec<&str>>> = Lazy::new(|| HashMap::from(
    [
        ("nvidia", vec!["gtx1650", "gtx580", "gtx4080ti", "gtx3060"]),
        ("amd", vec!["rx7900", "rx6700xt", "rx6800"]),
        ("intel", vec!["a770", "a750", "a380"])
    ]
));

static CPU_SOCKETS: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "lga1151", "lga3647", "lga1200", "lga1700"
]);

static GPU_BRAND_MODEL: Lazy<HashMap<&str, Vec<&str>>> = Lazy::new(|| HashMap::from(
    [
        ("nvidia", vec!["gtx1650", "gtx580", "gtx4080ti", "gtx3060"]),
        ("amd", vec!["rx7900", "rx6700xt", "rx6800"]),
        ("intel", vec!["a770", "a750", "a380"])
    ]
));

static GPU_PORTS: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "dp", "dvi", "vga", "hdmi"
]);

static TECH_TYPES: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "pc", "laptop", "tablet", "phone"
]);

static WARRANTIES: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "1day", "1week", "1month", "3month", "1year"
]);

#[derive(Debug, Serialize, Deserialize)]
struct MemSupp {
    #[serde(rename = "type")]
    mem_type: String,
    #[serde(rename = "maxSizeGb")]
    max_size_gb: u8,
}

impl Dummy<Faker> for MemSupp {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        MemSupp {
            mem_type: vec!["ddr3", "ddr4", "ddr5"].choose(rng).unwrap().to_string(),
            max_size_gb: 2u8.pow(rng.gen_range(3..8)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MemDetails {
    #[serde(rename = "type")]
    mem_type: String,
    #[serde(rename = "sizeGb")]
    size_gb: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Clock {
    #[serde(rename = "coreSpeedGhz")]
    core_speed_ghz: f64,
    #[serde(rename = "boostSpeedGhz")]
    boost_speed_ghz: f64,
}

impl Dummy<Faker> for Clock {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let core_speed_ghz = format!("{:.2}", rng.gen_range(0.8..4.0)).parse().unwrap();
        let boost_speed_ghz = format!("{:.2}", (core_speed_ghz + 0.4)).parse().unwrap();

        Clock {
            core_speed_ghz,
            boost_speed_ghz,
        } 
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu {
    _id: ObjectIdWrapper,
    price: f64,
    brand: String,
    model: String,
    arch: String,
    cores: u8,
    threads: u8,
    #[serde(rename = "socketType")]
    socket_type: String,
    #[serde(rename = "overclockSupp")]
    overclock_supp: bool,
    #[serde(rename = "soldSep")]
    sold_sep: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    warranty: Option<String>,
    #[serde(rename = "memorySupp")]
    memory_supp: MemSupp,
    clock: Clock,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu: Option<Gpu>,
    lot: Vec<Lot>,
}

impl Dummy<Faker> for Cpu {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let brand = CPU_BRAND_MODEL.keys().choose(rng).unwrap();
        let model = CPU_BRAND_MODEL.get(brand).unwrap().choose(rng).unwrap();

        Cpu {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            price: rng.gen_range(50.0..200.0),
            brand: brand.to_string(),
            model: model.to_string(),
            arch: vec!["x86", "x64"].choose(rng).unwrap().to_string(),
            cores: 2u8.pow(rng.gen_range(0..4)),
            threads: rng.gen_range(1..4),
            socket_type: CPU_SOCKETS.choose(rng).unwrap().to_string(),
            overclock_supp: rng.gen_bool(0.5),
            sold_sep: rng.gen_bool(0.5),
            warranty:
                if rng.gen_bool(0.5) == true {
                    Some(WARRANTIES.choose(rng).unwrap().to_string())
                } else {
                    None
                },
            memory_supp: MemSupp::dummy_with_rng(config, rng),
            clock: Clock::dummy_with_rng(config, rng),
            gpu: None,
            lot: (0..rng.gen_range(1..5)).map(|_| Lot::dummy_with_rng(config, rng)).collect(),
        } 
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Gpu {
    _id: ObjectIdWrapper,
    price: f64,
    brand: String,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cuda_cores: Option<u16>,
    tdp: u8,
    ports: Vec<String>,
    dedicated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    warranty: Option<String>,
    memory: MemDetails,
    clock: Clock,
    lot: Vec<Lot>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tech {
    _id: ObjectIdWrapper,
    price: f64,
    brand: String,
    model: String,
    color: Vec<String>,
    #[serde(rename = "type")]
    tech_type: String,
    memory: u16,
    cpu: ObjectIdWrapper,
    gpu: Option<ObjectIdWrapper>,
    lot: Vec<Lot>,
}