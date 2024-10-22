pub use fake::{
    { Dummy, Fake, Faker, Rng },
    faker::lorem::en::Word,
    faker::name::en::Name,
    faker::barcode::en::Isbn,
};
pub use mongodb::{
    bson::{ doc, Document, oid::ObjectId },
    options::{ ClientOptions, ResolverConfig, ServerApi, ServerApiVersion },
    Client,
    Collection
};
pub use chrono::{ DateTime, TimeZone, Utc, NaiveDate };
pub use serde::{ Serialize, Deserialize };
pub use rand::prelude::SliceRandom;
pub use futures_util::stream::TryStreamExt;

use once_cell::sync::Lazy;

pub static COLORS: Lazy<Vec<&'static str>> = Lazy::new(|| vec![
    "red", "green", "blue", "yellow", "orange", "teal", "purple", "pink", "white", "black", "brown"
]);

pub static RND_ITEM_PIPELINE: Lazy<Vec<Document>> = Lazy::new(|| vec![
    doc! { "$addFields": { "random": {"$rand": {} }}},
    doc! { "$sort": { "random": 1 }},
    doc! { "$limit": 1 },
    doc! { "$project": { "_id": 1, "lot": 1 }},
]);

pub fn get_rnd_item_pipeline(item_amt: i64) -> Vec<Document> {
    vec![
        doc! { "$addFields": { "random": {"$rand": {} }}},
        doc! { "$sort": { "random": 1 }},
        doc! { "$limit": item_amt },
        doc! { "$project": { "_id": 1, "lot": 1 }},
    ]
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
    pub lot: Vec<Lot>,
}

impl Dummy<Faker> for Size {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        Size {
            length: rng.gen_range(1.0..50.0),
            width: rng.gen_range(1.0..50.0),
            height: rng.gen_range(1.0..50.0),
        } 
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    length: f64,
    width: f64,
    height: f64, 
}



#[derive(Debug, Serialize, Deserialize, Clone)]
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
        if let Some(_) = self.code.first() {
            Some(&self.code)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lot {
    pub _id: ObjectIdWrapper,
    enter_date: DateTimeWrapper,
    pub code: Vec<ObjectIdWrapper>,
}

impl Dummy<Faker> for Lot {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let code_amt = rng.gen_range(1..36);

        Lot {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            enter_date: DateTimeWrapper::dummy_with_rng(config, rng),
            code: (0..code_amt).map(|_| ObjectIdWrapper::dummy_with_rng(config, rng)).collect(),
        }
    }
}