use crate::common::*;
use once_cell::sync::Lazy;

static GENDERS: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "male", "female", "other"
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

pub static JOBS: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "cashier", "bagger", "janitor", "stock clerk", "IT"
]);

impl Job {
    pub fn dummy_with_rng<R: Rng + ?Sized>(name: &str, store_ids: &Vec<ObjectIdWrapper>, client: &mongodb::Client, config: &Faker, rng: &mut R) -> Self {
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

#[derive(Debug, Serialize, Deserialize)]
struct Cart {
    #[serde(rename = "dateAdded")]
    date_added: DateTimeWrapper,
    coll: String,
    item: ObjectIdWrapper,
}

impl Cart {
    fn dummy_with_rng<R: Rng + ?Sized>(coll: String, item: ObjectIdWrapper, config: &Faker, rng: &mut R) -> Self {
        Cart {
            date_added: DateTimeWrapper::dummy_with_rng(config, rng),
            coll,
            item,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Client {
    age: u8,
    gender: String,
    #[serde(rename = "phoneNum")]
    phone_num: String,
    interest: Vec<ObjectIdWrapper>,
    cart: Option<Box<Vec<Cart>>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Schedule {
    enter: String,
    exit: String,
    #[serde(rename = "enterDate", skip_serializing_if = "Option::is_none")]
    enter_date: Option<DateTimeWrapper>,
    #[serde(rename = "exitDate", skip_serializing_if = "Option::is_none")]
    exit_date: Option<DateTimeWrapper>,
    store: ObjectIdWrapper,
    job: ObjectIdWrapper,
}

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    age: u8,
    gender: String,
    #[serde(rename = "phoneNum")]
    phone_num: String,
    schedule: Vec<Schedule>,
}

#[derive(Debug, Serialize, Deserialize, Dummy)]
struct Admin {}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    _id: ObjectIdWrapper,
    email: String,
    username: String,
    password: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<Box<Client>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    employee: Option<Box<Employee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin: Option<Box<Admin>>,
}

fn get_rnd_store_pipeline(store_amt: i64) -> Vec<Document> {
    vec![
        doc! { "$addFields": { "random": {"$rand": {} }}},
        doc! { "$sort": { "random": 1 }},
        doc! { "$limit": store_amt },
        doc! { "$project": { "_id": 1 }},
    ]
}

impl User {
    pub async fn dummy_with_rng<R: Rng + ?Sized>(
        store_ids: &HashMap<&str, ObjectIdWrapper>,
        client: &mongodb::Client,
        config: &Faker,
        rng: &mut R
    ) -> Self {
        let db = client.database("nexis");

        let (client, employee, admin) = match rng.gen_range(0..3) {
            0 => {
                /*
                let stores_coll: Collection<Document> = db.collection("store");
                let mut cursor = stores_coll.aggregate(get_rnd_store_pipeline(rng.gen_range(1..=4))).await.expect("");
                let mut rnd_stores: Vec<ObjectIdWrapper> = Vec::new();
                while let Ok(Some(res)) = cursor.try_next().await {
                    if let Some(Bson::ObjectId(oid)) = res.get("_id") {
                        rnd_stores.push(ObjectIdWrapper(oid.clone()));
                    } else {
                        panic!("Expected `_id` key for store");
                    }
                }
                */
                let rnd_stores: Vec<ObjectIdWrapper> = {
                    let mut used_stores: HashSet<ObjectIdWrapper> = HashSet::new();
                    let store_ids: Vec<ObjectIdWrapper> = store_ids.values().map(|val| val.clone()).collect();

                    (0..rng.gen_range(1..4)).filter_map(|_| {
                        let store = store_ids.choose(rng).unwrap().clone();
                        if used_stores.insert(store.clone()) {
                            Some(store)
                        } else {
                            None
                        }
                    }).collect()
                };
                
                let cart = if rng.gen_bool(0.5) {
                    let item_amt = rng.gen_range(1..=3);
                    let mut item_coll: HashMap<ObjectIdWrapper, String> = HashMap::new();
                    for _ in 0..=item_amt {
                        let(item, coll) = get_rnd_item_simple(
                                rng,
                                client,
                                vec!["clothes", "food", "libraryItem", "tech", "techCpu", "techGpu", "techKeyboard", "techOther"]
                        ).await;

                        item_coll.insert(item.get_id().clone(), coll);
                    };

                    Some(Box::new(
                        item_coll.into_iter().map(|(item, coll)| Cart::dummy_with_rng(coll, item, config, rng)).collect()
                    ))
                } else {
                    None
                };

                (
                    Some(Box::new(Client {
                        age: rng.gen_range(16..=70),
                        gender: GENDERS.choose(rng).unwrap().to_string(),
                        phone_num: CellNumber().fake(),
                        interest: rnd_stores,
                        cart,
                    })),
                    None,
                    None
                ) 
            }
            1 => {
                //todo!("employee gen")
                (
                    None,
                    None,
                    Some(Box::new(Admin {}))
                )
            }
            2 => {
                (
                    None,
                    None,
                    Some(Box::new(Admin {})),
                )
            }
            _ => unimplemented!()
        };

        User {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            email: FreeEmail().fake(),
            username: Username().fake(),
            password: Password(8..9).fake(),
            name: Name().fake(),
            client,
            employee,
            admin, 
        } 
    }
}