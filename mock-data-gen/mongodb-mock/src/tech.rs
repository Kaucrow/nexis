use crate::prelude::*;
use std::collections::{ HashMap, HashSet };
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
            max_size_gb: 2u8.pow(rng.gen_range(3..=7)),
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

impl Dummy<Faker> for MemDetails {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        MemDetails {
            mem_type: vec!["ddr3", "ddr4", "ddr5"].choose(rng).unwrap().to_string(),
            size_gb: 2u8.pow(rng.gen_range(0..=3)),
        }
    }
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
        let core_speed_ghz = (rng.gen_range(0.8..=4.0) as f64).round_to_2();
        let boost_speed_ghz = ((core_speed_ghz + 0.4) as f64).round_to_2();

        Clock {
            core_speed_ghz,
            boost_speed_ghz,
        } 
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu {
    _id: ObjectIdWrapper,
    name: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    warranty: Option<String>,
    #[serde(rename = "memorySupp")]
    memory_supp: MemSupp,
    clock: Clock,
    graphics: String,
    lots: Vec<Lot>,
}

impl Dummy<Faker> for Cpu {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let brand = CPU_BRAND_MODEL.keys().choose(rng).unwrap();
        let model = CPU_BRAND_MODEL[brand].choose(rng).unwrap();

        let warranty = if rng.gen_bool(0.5) {
            Some(WARRANTIES.choose(rng).unwrap().to_string())
        } else {
            None
        };

        Cpu {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            name: format!("{} {}", brand, model),
            price: (rng.gen_range(50.0..=200.0) as f64).round_to_2(),
            brand: brand.to_string(),
            model: model.to_string(),
            arch: vec!["x86", "x64"].choose(rng).unwrap().to_string(),
            cores: 2u8.pow(rng.gen_range(0..=3)),
            threads: rng.gen_range(1..=3),
            socket_type: CPU_SOCKETS.choose(rng).unwrap().to_string(),
            overclock_supp: rng.gen_bool(0.5),
            warranty,
            memory_supp: MemSupp::dummy_with_rng(config, rng),
            clock: Clock::dummy_with_rng(config, rng),
            graphics: Word().fake(),
            lots: (0..rng.gen_range(1..=5)).map(|_| Lot::dummy_with_rng(config, rng)).collect(),
        } 
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gpu {
    _id: ObjectIdWrapper,
    name: String,
    price: f64,
    brand: String,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cuda_cores: Option<u16>,
    tdp: u16,
    ports: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warranty: Option<String>,
    memory: MemDetails,
    clock: Clock,
    lots: Vec<Lot>,
}

impl Dummy<Faker> for Gpu {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let brand = GPU_BRAND_MODEL.keys().choose(rng).unwrap();
        let model = GPU_BRAND_MODEL[brand].choose(rng).unwrap();

        let cuda_cores = {
            if rng.gen_bool(0.5) {
                Some(rng.gen_range(1..=24_576))
            } else {
                None
            }
        };

        let ports = {
            let amt = rng.gen_range(1..=2);
            let mut used: HashSet<&str> = HashSet::new();
            (0..amt).filter_map(|_| {
                let port = GPU_PORTS.choose(rng).unwrap();
                if used.insert(port) {
                    Some(port.to_string())
                } else {
                    None
                }
            }).collect()
        };

        let warranty = {
            if rng.gen_bool(0.5) {
                Some(WARRANTIES.choose(rng).unwrap().to_string())
            } else {
                None
            }
        };

        Gpu {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            name: format!("{} {}", brand, model),
            price: (rng.gen_range(50.0..=300.0) as f64).round_to_2(),
            brand: brand.to_string(),
            model: model.to_string(),
            cuda_cores,
            tdp: rng.gen_range(10..=700),
            ports,
            warranty,
            memory: MemDetails::dummy_with_rng(config, rng),
            clock: Clock::dummy_with_rng(config, rng),
            lots: (0..rng.gen_range(1..=5)).map(|_| Lot::dummy_with_rng(config, rng)).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tech {
    _id: ObjectIdWrapper,
    name: String,
    price: f64,
    brand: String,
    model: String,
    color: String,
    #[serde(rename = "type")]
    tech_type: String,
    ram: u16,
    storage: u16,
    cpu: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu: Option<String>,
    lots: Vec<Lot>,
}

impl Tech {
    pub fn dummy_with_rng<R: rand::Rng + ?Sized>(cpu: ObjectIdWrapper, gpu: Option<ObjectIdWrapper>, config: &Faker, rng: &mut R) -> Self {
        let brand: String = Word().fake();
        let model: String = Word().fake();

        let cpu_brand = CPU_BRAND_MODEL.keys().choose(rng).unwrap();
        let cpu_model = CPU_BRAND_MODEL[cpu_brand].choose(rng).unwrap();

        let gpu = if rng.gen_bool(0.5) {
            let gpu_brand = GPU_BRAND_MODEL.keys().choose(rng).unwrap();
            let gpu_model = GPU_BRAND_MODEL[gpu_brand].choose(rng).unwrap();
            Some(format!("{} {}", gpu_brand, gpu_model))
        } else {
            None
        };

        Tech {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            name: format!("{} {}", brand, model),
            price: (rng.gen_range(80.0..=1200.0) as f64).round_to_2(),
            brand,
            model,
            color: COLORS.choose(rng).unwrap().to_string(),
            tech_type: TECH_TYPES.choose(rng).unwrap().to_string(),
            ram: 2u16.pow(rng.gen_range(0..=6)),
            storage: 2u16.pow(rng.gen_range(3..=10)),
            cpu: format!("{} {}", cpu_brand, cpu_model),
            gpu,
            lots: (0..rng.gen_range(1..=5)).map(|_| Lot::dummy_with_rng(config, rng)).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keyboard {
    _id: ObjectIdWrapper,
    name: String,
    price: f64,
    brand: String,
    model: String,
    #[serde(rename = "type")]
    keyboard_type: String,
    #[serde(rename = "keySwitch")]
    key_switch: String,
    backlight: bool,
    wireless: bool,
    dimensions: Size,
    #[serde(rename = "weightKg")]
    weight_kg: f64,
    lots: Vec<Lot>
}

pub static KEYB_TYPES: Lazy<Vec<&'static str>> = Lazy::new(|| vec![
    "mechanical", "membrane", "chiclet"
]);

pub static KEYSW_TYPES: Lazy<Vec<&'static str>> = Lazy::new(|| vec![
    "linear", "tactile", "clicky"
]);

impl Dummy<Faker> for Keyboard {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let brand: String = Word().fake();
        let model: String = Word().fake();

        Keyboard {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            name: format!("{} {}", brand, model),
            price: (rng.gen_range(10.0..300.0) as f64).round_to_2(),
            brand,
            model,
            keyboard_type: KEYB_TYPES.choose(rng).unwrap().to_string(),
            key_switch: KEYSW_TYPES.choose(rng).unwrap().to_string(),
            backlight: rng.gen_bool(0.5),
            wireless: rng.gen_bool(0.5),
            dimensions: Size::dummy_with_rng(config, rng),
            weight_kg: (rng.gen_range(0.5..=1.5) as f64).round_to_2(),
            lots: (1..=rng.gen_range(1..5)).map(|_| Lot::dummy_with_rng(config, rng)).collect(),
        } 
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechOther {
    _id: ObjectIdWrapper,
    name: String,
    price: f64,
    lots: Vec<Lot>,
}

impl Dummy<Faker> for TechOther {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        TechOther {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            name: Word().fake(),
            price: (rng.gen_range(0.5..100.0) as f64),
            lots: (0..rng.gen_range(1..5)).map(|_| Lot::dummy_with_rng(config, rng)).collect(),
        }
    }
}