use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Food {
    _id: ObjectIdWrapper,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
    #[serde(rename = "pricePerKg", skip_serializing_if = "Option::is_none")]
    price_per_kg: Option<f64>,
    #[serde(rename = "type")]
    food_type: String,
    lots: Vec<FoodLot>,
}

impl Dummy<Faker> for Food {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let types = vec![
            "meat", "poultry", "cheese", "drink", "can", "vegetable", "fruit", "cereal", "grain"
        ];

        let food_type = String::from(*types.choose(rng).unwrap());

        let (price, price_per_kg) = match food_type.as_str() {
            "drink" | "can" | "cereal"
                => (Some(rng.gen_range(1.0..50.0)), None),
            "meat" | "poultry" | "cheese" | "vegetable" | "fruit" | "grain"
                => (None, Some(rng.gen_range(1.0..100.0))),
            _ => unimplemented!()
        };

        Food {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            name: Word().fake(),
            price,
            price_per_kg,
            food_type,
            lots: (0..10).map(|_| FoodLot::dummy_with_rng(config, rng)).collect(),
        } 
    }
}