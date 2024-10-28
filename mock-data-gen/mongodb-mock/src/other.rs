use crate::common::*;
use once_cell::sync::Lazy;

pub static JOBS: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "cashier", "bagger", "janitor", "stock clerk", "IT"
]);

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    _id: ObjectIdWrapper,
    name: String,
    #[serde(rename = "payPerHour", skip_serializing_if = "Option::is_none")]
    pay_per_hour: Option<f64>,
    #[serde(rename = "payPerWeek", skip_serializing_if = "Option::is_none")]
    pay_per_week: Option<f64>,
    stores: Vec<ObjectIdWrapper>,
}

impl Job {
    pub fn dummy_with_rng<R: Rng + ?Sized>(name: &str, store_ids: &Vec<ObjectIdWrapper>, _client: &mongodb::Client, config: &Faker, rng: &mut R) -> Self {
        let (pay_per_hour, pay_per_week) =
            if rng.gen_bool(0.5) {
                (Some((rng.gen_range(5.0..=50.0) as f64).round_to_2()), None)    
            } else {
                (None, Some((rng.gen_range(200.0..=1000.0) as f64).round_to_2()))
            };

        let stores: Vec<ObjectIdWrapper> = {
            let mut used_stores: HashSet<ObjectIdWrapper> = HashSet::new();
                (0..4).filter_map(|_| {
                let store = store_ids[rng.gen_range(0..4)].clone();
                if used_stores.insert(store.clone()) { Some(store) }
                else { None }
            }).collect()
        };

        Job {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            name: name.to_string(),
            pay_per_hour,
            pay_per_week,
            stores,
        }
    }
}