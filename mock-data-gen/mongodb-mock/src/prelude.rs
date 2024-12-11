pub use fake::{
    { Dummy, Fake, Faker, Rng },
    faker::{
        lorem::en::{ Word, Words },
        name::en::Name,
        barcode::en::Isbn,
        internet::en::{ FreeEmail, Username, Password },
        phone_number::en::CellNumber,
    },
};
pub use mongodb::{
    bson::{ self, Bson, doc, Document, oid::ObjectId },
    options::{ ClientOptions, ResolverConfig, ServerApi, ServerApiVersion, IndexOptions },
    Client,
    Collection,
    IndexModel,
};
pub use chrono::{ DateTime, TimeZone, Utc, NaiveDate };
pub use serde::{ Serialize, Deserialize };
pub use rand::prelude::SliceRandom;
pub use futures_util::stream::{ self, StreamExt, TryStreamExt };
pub use std::collections::{ HashMap, HashSet };

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHasher, SaltString
    },
    Argon2
};
use once_cell::sync::Lazy;

pub static ITEM_COLLS: Lazy<Vec<&'static str>> = Lazy::new(|| vec![
    "clothes", "food", "libraryItems", "techCpus", "techGpus", "techs", "techOthers", "techKeyboards"
]);

pub static COLORS: Lazy<Vec<&'static str>> = Lazy::new(|| vec![
    "red", "green", "blue", "yellow", "orange", "teal", "purple", "pink", "white", "black", "brown"
]);

pub static STORES: Lazy<Vec<&'static str>> = Lazy::new(|| vec![
    "vesti", "readon", "savoro", "cyberion"
]);

pub fn get_rnd_item_pipeline(item_amt: i64) -> Vec<Document> {
    vec![
        doc! { "$sample": { "size": item_amt }},
        doc! { "$match": { "lots": { "$elemMatch": { "codes": { "$ne": [] }}}}},
        doc! { "$project": { "_id": 1, "lots": 1 }},
    ]
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SimpleItem {
    Regular(ItemSimple),
    Food(FoodItemSimple),
}

impl SimpleItem {
    pub fn get_id(&self) -> &ObjectIdWrapper {
        match self {
            SimpleItem::Regular(item) => item.get_id(),
            SimpleItem::Food(item) => item.get_id(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodLot {
    _id: ObjectIdWrapper,
    #[serde(rename = "enterDate")]
    enter_date: DateTimeWrapper,
    expiry: DateTimeWrapper,
    codes: Vec<ObjectIdWrapper>
}

impl LotTrait for FoodLot {
    fn get_id(&self) -> &ObjectIdWrapper {
        &self._id
    }

    fn get_code(&self) -> Option<&Vec<ObjectIdWrapper>> {
        if let Some(_) = self.codes.first() {
            Some(&self.codes)
        } else {
            None
        }
    }
}

impl Dummy<Faker> for FoodLot {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let enter_date = DateTimeWrapper::dummy_with_rng(config, rng);
        let expiry = DateTimeWrapper(enter_date.0.checked_add_days(chrono::Days::new(7)).expect(""));

        FoodLot {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            enter_date,
            expiry,
            codes: (0..10).map(|_| ObjectIdWrapper::dummy_with_rng(config, rng)).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodItemSimple {
    _id: ObjectIdWrapper,
    lots: Vec<FoodLot>,
}

pub trait SimpleItemTrait {
    type LotType: LotTrait;

    fn get_id(&self) -> &ObjectIdWrapper;
    fn get_lot(&self) -> Option<&Self::LotType>;
}

impl SimpleItemTrait for ItemSimple {
    type LotType = Lot;

    fn get_id(&self) -> &ObjectIdWrapper {
        &self._id
    }

    fn get_lot(&self) -> Option<&Self::LotType> {
        self.lots.first()
    }
}

impl SimpleItemTrait for FoodItemSimple {
    type LotType = FoodLot;

    fn get_id(&self) -> &ObjectIdWrapper {
        &self._id
    }

    fn get_lot(&self) -> Option<&Self::LotType> {
        self.lots.first()
    }
}

pub async fn get_rnd_item_simple<R: Rng + ?Sized>(rng: &mut R, client: &Client, colls: Vec<&str>) -> (SimpleItem, String) {
    let db = client.database("nexis");
    let coll_name = colls.choose(rng).unwrap();
    let coll: Collection<Document> = db.collection(coll_name);
    let mut cursor = coll.aggregate(get_rnd_item_pipeline(1)).await.expect("");

    if coll_name == &"food" {
        let item: FoodItemSimple = {
            if let Ok(Some(res)) = cursor.try_next().await {
                bson::from_document(res).unwrap()
            } else { panic!("Err getting simple item in collection `{}`", coll_name) }
        };
        (SimpleItem::Food(item), coll_name.to_string())
    } else {
        let item: ItemSimple = {
            if let Ok(Some(res)) = cursor.try_next().await {
                bson::from_document(res).unwrap()
            } else { panic!("Err getting simple item in collection `{}`", coll_name) }
        };
        (SimpleItem::Regular(item), coll_name.to_string())
    }
}

pub trait RoundTo2 {
    fn round_to_2(self) -> f64;
}

impl RoundTo2 for f64 {
    fn round_to_2(self) -> f64 {
        (self * 100.0).round() / 100.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSimple {
    pub _id: ObjectIdWrapper,
    pub lots: Vec<Lot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    length: f64,
    width: f64,
    height: f64, 
}

impl Dummy<Faker> for Size {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        Size {
            length: (rng.gen_range(1.0..50.0) as f64).round_to_2(),
            width: (rng.gen_range(1.0..50.0) as f64).round_to_2(),
            height: (rng.gen_range(1.0..50.0) as f64).round_to_2(),
        } 
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ObjectIdWrapper(pub ObjectId);

impl Dummy<Faker> for ObjectIdWrapper {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, _rng: &mut R) -> Self {
        ObjectIdWrapper(ObjectId::new())  // Generate a new ObjectId
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateTimeWrapper(pub DateTime<Utc>);

impl Dummy<Faker> for DateTimeWrapper {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let year = rng.gen_range(2000..2024);
        let month = rng.gen_range(1..13);
        let day = rng.gen_range(1..29); // Simplified to avoid invalid dates
        let hour = rng.gen_range(0..24);
        let min = rng.gen_range(0..60);
        let sec = rng.gen_range(0..60);

        let naive_date = NaiveDate::from_ymd_opt(year, month, day).expect("");

        DateTimeWrapper(Utc.from_utc_datetime(&naive_date.and_hms_opt(hour, min, sec).expect("")))
    }
}

pub trait LotTrait {
    fn get_id(&self) -> &ObjectIdWrapper;
    fn get_code(&self) -> Option<&Vec<ObjectIdWrapper>>;
}

impl LotTrait for Lot {
    fn get_id(&self) -> &ObjectIdWrapper {
        &self._id
    }

    fn get_code(&self) -> Option<&Vec<ObjectIdWrapper>> {
        if let Some(_) = self.codes.first() {
            Some(&self.codes)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lot {
    pub _id: ObjectIdWrapper,
    #[serde(rename = "enterDate")]
    enter_date: DateTimeWrapper,
    pub codes: Vec<ObjectIdWrapper>,
}

impl Dummy<Faker> for Lot {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let code_amt = rng.gen_range(1..36);

        Lot {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            enter_date: DateTimeWrapper::dummy_with_rng(config, rng),
            codes: (0..code_amt).map(|_| ObjectIdWrapper::dummy_with_rng(config, rng)).collect(),
        }
    }
}

pub async fn hash(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password, &salt)
        .expect("Unable to hash password.")
        .to_string()
}