use crate::prelude::*;
use chrono::{ Duration, Datelike, Timelike, Weekday };
use once_cell::sync::Lazy;
use serde::Serializer;

static GENDERS: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "male", "female", "other"
]);

#[derive(Debug, Serialize, Deserialize)]
struct CartItem {
    #[serde(rename = "dateAdded")]
    date_added: DateTimeWrapper,
    coll: String,
    item: ObjectIdWrapper,
}

impl CartItem {
    fn dummy_with_rng<R: Rng + ?Sized>(coll: String, item: ObjectIdWrapper, config: &Faker, rng: &mut R) -> Self {
        CartItem {
            date_added: DateTimeWrapper::dummy_with_rng(config, rng),
            coll,
            item,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    stars: u8,
    title: String,
    comment: String,
    coll: String,
    item: ObjectIdWrapper, 
}

impl Review {
    fn dummy_with_rng<R: Rng + ?Sized>(coll: String, item: ObjectIdWrapper, _config: &Faker, rng: &mut R) -> Self {
        let title: Vec<String> = Words(1..8).fake();
        let comment: Vec<String> = Words(8..30).fake();

        Review {
            stars: rng.gen_range(1..=5),
            title: title.join(" "),
            comment: comment.join(" "),
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
    interests: Vec<ObjectIdWrapper>,
    cart: Option<Box<Vec<CartItem>>>,
    reviews: Option<Box<Vec<Review>>>,
}

impl Client {
    async fn dummy_with_rng<R: Rng + ?Sized>(store_ids: &Vec<ObjectIdWrapper>, client: &mongodb::Client, config: &Faker, rng: &mut R) -> Self {
        let rnd_stores: Vec<ObjectIdWrapper> = {
            let mut used_stores: HashSet<ObjectIdWrapper> = HashSet::new();

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
                        ITEM_COLLS.to_vec(),
                ).await;

                item_coll.insert(item.get_id().clone(), coll);
            };

            Some(Box::new(
                item_coll.into_iter().map(|(item, coll)| CartItem::dummy_with_rng(coll, item, config, rng)).collect()
            ))
        } else {
            None
        };

        let reviews = if rng.gen_bool(0.8) {
            let reviews_amt = rng.gen_range(1..=5);
            let mut used_items: HashSet<ObjectIdWrapper> = HashSet::new();
            let mut reviews: Vec<Review> = Vec::new();
            for _ in 0..reviews_amt {
                let (item, coll) = get_rnd_item_simple(
                    rng,
                    client,
                    ITEM_COLLS.to_vec()
                ).await;

                if used_items.insert(item.get_id().clone()) {
                    reviews.push(
                        Review::dummy_with_rng(coll, item.get_id().clone(), config, rng)
                    );
                }
            }
            Some(Box::new(reviews))
        } else {
            None
        };

        Client {
            age: rng.gen_range(16..=70),
            gender: GENDERS.choose(rng).unwrap().to_string(),
            phone_num: CellNumber().fake(),
            interests: rnd_stores,
            cart,
            reviews,
        }
    }
}

fn format_datetime(dt: DateTime<Utc>) -> String {
    let weekday = match dt.weekday() {
        Weekday::Mon => "Mon",
        Weekday::Tue => "Tue",
        Weekday::Wed => "Wed",
        Weekday::Thu => "Thu",
        Weekday::Fri => "Fri",
        Weekday::Sat => "Sat",
        Weekday::Sun => "Sun",
    };
    let hour = dt.hour();
    let (hour, period) = if hour < 12 {
        (hour, "AM")
    } else {
        (if hour == 12 { hour } else { hour - 12 }, "PM")
    };

    format!("{} {}:{} {} UTC", weekday, hour, dt.minute(), period)
}

#[derive(Debug, Deserialize)]
struct Schedule {
    enter: DateTimeWrapper,
    exit: DateTimeWrapper,
    #[serde(rename = "enterDate", skip_serializing_if = "Option::is_none")]
    enter_date: Option<DateTimeWrapper>,
    #[serde(rename = "exitDate", skip_serializing_if = "Option::is_none")]
    exit_date: Option<DateTimeWrapper>,
    store: ObjectIdWrapper,
    job: ObjectIdWrapper,
}

impl Serialize for Schedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Schedule", 6)?;

        state.serialize_field("enter", &format_datetime(self.enter.0))?;
        state.serialize_field("exit", &format_datetime(self.exit.0))?;
        if self.enter_date.is_some() {
            state.serialize_field("enterDate", &self.enter_date)?;
        }
        if self.exit_date.is_some() {
            state.serialize_field("exitDate", &self.exit_date)?;
        }
        state.serialize_field("store", &self.store)?;
        state.serialize_field("job", &self.job)?;

        state.end()
    }
}

fn get_rnd_storejob_pipeline() -> Vec<Document> {
    vec![
        doc! { "$sample": { "size": 1 }},
        doc! {"$addFields": {
                "rndStore": doc! {
                    "$arrayElemAt": [
                        "$stores",
                        {
                            "$floor": {
                                "$multiply": [ { "$rand": {}}, { "$size": "$stores" }]
                            }
                        }
                    ]
                }
            }
        },
        doc! {"$project": {
                "_id": 1,
                "rndStore": 1
            }
        }
    ]
}

impl Schedule {
    async fn dummy_with_rng<R: Rng + ?Sized>(min_datetime: &mut Option<DateTime<Utc>>, client: &mongodb::Client, _config: &Faker, rng: &mut R) -> Self {
        let shift_duration = Duration::hours(12);
        let enter =
            if let Some(date) = min_datetime {
                date.clone() + Duration::hours(rng.gen_range(12..=24))
            } else {
                let naive_date = NaiveDate::from_ymd_opt(2024, 10, 21).unwrap();
                Utc.from_utc_datetime(&naive_date.and_hms_opt(rng.gen_range(0..24), 0, 0).expect(""))
            };

        let exit = enter + shift_duration;
        *min_datetime = Some(exit);

        let db = client.database("nexis");
        let jobs_coll: Collection<Document> = db.collection("storeJobs");

        let mut cursor = jobs_coll.aggregate(get_rnd_storejob_pipeline()).await.expect("");
        
        let (job, store) =
            if let Ok(Some(res)) = cursor.try_next().await {
                let job = if let Some(Bson::ObjectId(oid)) = res.get("_id") {
                    ObjectIdWrapper(*oid)
                } else { panic!() };

                let store = if let Some(Bson::ObjectId(oid)) = res.get("rndStore") {
                    ObjectIdWrapper(*oid)
                } else { panic!() };

                ( job, store )
            } else {
                panic!()
            };

        Schedule {
            enter: DateTimeWrapper(enter),
            exit: DateTimeWrapper(exit),
            enter_date: None,
            exit_date: None,
            store,
            job,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    age: u8,
    gender: String,
    #[serde(rename = "phoneNum")]
    phone_num: String,
    schedule: Vec<Schedule>,
}

impl Employee {
    async fn dummy_with_rng<R: Rng + ?Sized>(client: &mongodb::Client, config: &Faker, rng: &mut R) -> Self {
        let mut min_datetime = None;
        let mut schedule: Vec<Schedule> = Vec::new();

        for _ in 0..4 {
            schedule.push(Schedule::dummy_with_rng(&mut min_datetime, client, config, rng).await)
        }

        Employee {
            age: rng.gen_range(18..=70),
            gender: GENDERS.choose(rng).unwrap().to_string(),
            phone_num: CellNumber().fake(),
            schedule,
        }   
    }
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
    #[serde(rename = "isActive")]
    is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<Box<Client>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    employee: Option<Box<Employee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin: Option<Box<Admin>>,
}

#[derive(Debug)]
pub struct UserDetails {
    pub email: String,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl User {
    pub async fn dummy_with_rng<R: Rng + ?Sized>(
        store_ids: &Vec<ObjectIdWrapper>,
        client: &mongodb::Client,
        config: &Faker,
        rng: &mut R
    ) -> Self {
        let (client, employee, admin) = match rng.gen_range(0..3) {
            0 => {
                (
                    Some(Box::new(Client::dummy_with_rng(store_ids, client, config, rng).await)),
                    None,
                    None
                )
            }
            1 => {
                (
                    None,
                    Some(Box::new(Employee::dummy_with_rng(client, config, rng).await)),
                    None,
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
            is_active: rng.gen_bool(0.8),
            client,
            employee,
            admin, 
        }
    }

    pub async fn custom<R: Rng + ?Sized>(
        roles: Vec<&str>,
        details: &UserDetails,
        store_ids: &Vec<ObjectIdWrapper>,
        mongo_client: &mongodb::Client,
        config: &Faker,
        rng: &mut R
    ) -> Self {
        let mut client: Option<Box<Client>> = None;
        let mut employee: Option<Box<Employee>> = None;
        let mut admin: Option<Box<Admin>> = None;

        for role in roles {
            match role {
                "client" =>
                    client = Some(Box::new(Client::dummy_with_rng(store_ids, mongo_client, config, rng).await)),
                "employee" =>
                    employee = Some(Box::new(Employee::dummy_with_rng(mongo_client, config, rng).await)),
                "admin" =>
                    admin = Some(Box::new(Admin {})),
                _ => unimplemented!("Unknown role")
            }
        }

        User {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            email: details.email.clone(),
            username: details.username.clone(),
            password: hash(details.password.as_bytes()).await,
            name: details.name.clone(),
            is_active: true,
            client,
            employee,
            admin,
        }
    }
}