use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub price: f64,
    pub coll: String,
}