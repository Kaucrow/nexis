use crate::common::*;

#[derive(Debug, Serialize, Deserialize, Dummy)]
struct Material {
    percentage: f64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clothes {
    _id: ObjectIdWrapper,
    name: String,
    price: f64,
    gender: String,
    age: String,
    size: String,
    color: Vec<String>,
    #[serde(rename = "type")]
    clothes_type: String,
    brand: String,
    material: Vec<Material>,
    lot: Vec<Lot>,
}

impl Dummy<Faker> for Clothes {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let brands = vec![
            "Nike", "Adidas", "Puma", "Under Armour", "Reebok",
        ];

        let ages = vec![
            "Baby", "Child", "Teen", "Adult", "Elderly",
        ];

        let sizes = vec![
            "S", "M", "L", "XL", "2XL", "3XL",
        ];

        let genders = vec![
            "M", "F", "Other"
        ];

        let types = vec![
            "T-Shirt", "Shoes", "Shirt", "Socks",
        ];

        let color_count = rng.gen_range(1..3);
        let material_count = rng.gen_range(1..2);
        let lot_count = rng.gen_range(3..5);

        Clothes {
            _id: ObjectIdWrapper(ObjectId::new()),
            name: Word().fake(),
            price: format!("{:.2}", rng.gen_range(10.0..500.0)).parse().unwrap(),
            gender: genders.choose(rng).unwrap().to_string(),
            age: ages.choose(rng).unwrap().to_string(),
            size: sizes.choose(rng).unwrap().to_string(),
            color: (0..color_count).map(|_| crate::COLORS.choose(rng).unwrap().to_string()).collect(),
            clothes_type: types.choose(rng).unwrap().to_string(),
            brand: brands.choose(rng).unwrap().to_string(),
            material: (0..material_count).map(|_| Material { percentage: (1.0..100.0).fake(), name: Word().fake() }).collect(),
            lot: (0..lot_count).map(|_| Faker::fake::<Lot>(&Faker)).collect(),
        }
    }
}