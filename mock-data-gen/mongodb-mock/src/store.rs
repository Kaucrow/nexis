use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    amount: f64,
    #[serde(rename = "type")]
    payment_type: Document,
}

impl Payment {
    pub fn dummy_with_rng<R: rand::Rng + ?Sized>(amount: f64, rng: &mut R) -> Self {
        let card_types = vec![
            "visa", "mastercard", "amex"
        ];

        let banks = vec![
            "bofa"
        ];

        let method_doc = match rng.gen_range(0..3) {
            0 => doc! { "cash": true },
            1 => {
                let card_type = card_types.choose(rng).unwrap();
                doc! { "card": card_type }
            },
            2 => {
                let bank = banks.choose(rng).unwrap();
                doc! {
                    "transfer": {
                        "bank": bank,
                        "refNum": rng.gen_range(1000..9999),
                    }
                }
            },
            _ => unimplemented!()
        };

        Payment {
            amount,
            payment_type: method_doc,
        } 
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSaleInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<ObjectIdWrapper>,
}

impl ClientSaleInfo {
    pub fn dummy_with_rng<R: rand::Rng + ?Sized>(client_id: ObjectIdWrapper, rng: &mut R) -> Self {
        match rng.gen_range(0..2) {
            0 => ClientSaleInfo {
                name: Some(Name().fake()),
                user: None,
            },
            1 => ClientSaleInfo {
                name: None,
                user: Some(client_id),
            },
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekSales {
    date: DateTimeWrapper,
    payment: Payment,
    client: ClientSaleInfo,
    items: Vec<ItemCode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ItemCode {
    coll: String,
    #[serde(rename = "itemId")]
    item_id: ObjectIdWrapper,
    #[serde(rename = "lotId")]
    lot_id: ObjectIdWrapper,
    code: ObjectIdWrapper,
}

fn process_item<T: SimpleItemTrait>(item: T, collection: String, items: &mut Vec<ItemCode>) {
    if let Some(lot) = item.get_lot() {
        if let Some(code) = lot.get_code() {
            items.push(
                ItemCode {
                    coll: collection,
                    item_id: item.get_id().clone(),
                    lot_id: lot.get_id().clone(),
                    code: code.first().expect("").clone(),
                }
            )
        }
    }
}

impl WeekSales {
    pub async fn dummy_with_rng<R: rand::Rng + ?Sized>(payment: Payment, client_info: ClientSaleInfo, store: &str, client: &Client, rng: &mut R) -> Result<Self, mongodb::error::Error> {
        let item_colls = match store {
            "clothes" => vec!["clothes"],
            "food" => vec!["food"],
            "library" => vec!["libraryItems"],
            "tech" => vec!["techs", "techCpus", "techGpus", "techKeyboards", "techOthers"],
            _ => unimplemented!()
        };

        let db = client.database("nexis");

        let mut items: Vec<ItemCode> = Vec::new();

        let max_iter = rng.gen_range(1..5);
        for _ in 0..max_iter {
            let collection  = {
                let collection = item_colls.choose(rng).unwrap();
                db.collection::<Document>(collection)
            };

            let mut cursor = collection.aggregate(get_rnd_item_pipeline(1)).await?;
            if let Some(res) = cursor.try_next().await? {
                if store == "food" {
                    let item: FoodItemSimple = bson::from_document(res)?;
                    process_item(item, collection.name().to_string(), &mut items);
                } else {
                    let item: ItemSimple = bson::from_document(res)?;
                    process_item(item, collection.name().to_string(), &mut items);
                }
            }
        }

        for item in items.clone() {
            let coll: Collection<Document> = db.collection(&item.coll);

            let filter = doc! {
                "_id": item.item_id.0,
                "lot._id": item.lot_id.0,
            };

            let update = doc! {
                "$pull": {
                    "lot.$.code": item.code.0,
                }
            };

            coll.update_one(filter, update).await?;
        };
        
        let date = {
            let naive_date = NaiveDate::from_ymd_opt(2024, 11, rng.gen_range(1..=7)).unwrap();
            let date = Utc.from_utc_datetime(&naive_date.and_hms_opt(rng.gen_range(0..24), 0, 0).expect(""));
            DateTimeWrapper(date)
        };

        Ok(
            WeekSales {
                date,
                payment,
                client: client_info,
                items,
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "userId")]
    user_id: ObjectIdWrapper,
    #[serde(rename = "incomePercentage")]
    income_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    #[serde(rename = "_id")]
    id: ObjectIdWrapper,
    name: String,
    num: u16,
    floor: u8,
    size: Size,
    #[serde(rename = "weekSales")]
    week_sales: Vec<WeekSales>,
    owners: Vec<Owner>,
    employees: Vec<ObjectIdWrapper>,
}

impl Store {
    pub async fn dummy_with_rng<R: rand::Rng + ?Sized>(
        store_type: &str,
        store_ids: &HashMap<&str, ObjectIdWrapper>,
        client: &Client,
        config: &Faker,
        rng: &mut R
    ) -> Result<Self, mongodb::error::Error> {
        let id = store_ids.get(store_type).expect(format!("Could not find store of type {store_type} in `store_ids`").as_str()).clone();

        let mut week_sales: Vec<WeekSales> = Vec::new();

        for _ in 0..rng.gen_range(1..20) {
            week_sales.push(
                WeekSales::dummy_with_rng(
                    Payment::dummy_with_rng(rng.gen_range(1.0..100.0), rng),
                    ClientSaleInfo::dummy_with_rng(ObjectIdWrapper::dummy_with_rng(config, rng), rng),
                    store_type,
                    client,
                    rng
                ).await?
            )
        }

        let db = client.database("nexis");
        let users_coll: Collection<Document> = db.collection("users");

        let owners: HashMap<ObjectIdWrapper, f64> = {
            let mut owners: HashMap<ObjectIdWrapper, f64> = HashMap::new();
            let owners_amt = rng.gen_range(1..=3);

            let mut cursor = users_coll.aggregate(vec![
                doc! { "$match": { "admin": { "$exists": 1 }}},
                doc! { "$sample": { "size": owners_amt as i32}},
            ]).await?;
                
            let mut owner_ids: Vec<ObjectIdWrapper> = Vec::new();
            while let Some(res) = cursor.try_next().await? {
                if let Some(Bson::ObjectId(oid)) = res.get("_id") {
                    owner_ids.push(ObjectIdWrapper(*oid));
                }
            }

            let mut remaining_percentage = 100.0;
            for i in 0..owners_amt {
                let percentage = if i == owners_amt - 1 {
                    remaining_percentage.round_to_2()
                } else {
                    let rnd_percentage = rng.gen_range(1.0..remaining_percentage / (owners_amt - i) as f64).round_to_2();
                    remaining_percentage -= rnd_percentage;
                    rnd_percentage
                };

                owners.insert(owner_ids[i].clone(), percentage);
            }

            owners
        };

        let employees: Vec<ObjectIdWrapper> = {
            let mut employees: Vec<ObjectIdWrapper> = Vec::new();
            let employees_amt = rng.gen_range(3..=8);

            let mut cursor = users_coll.aggregate(vec![
                doc!{ "$match": { "employee.schedule": { "$elemMatch": { "store": id.0 }}}},
                doc!{ "$sample": { "size": employees_amt }}
            ]).await?;

            while let Some(res) = cursor.try_next().await? {
                if let Some(Bson::ObjectId(oid)) = res.get("_id") {
                    employees.push(ObjectIdWrapper(*oid));
                }
            }

            employees
        };

        let name = String::from(
            match store_type {
                "clothes" => "vesti",
                "food" => "savoro",
                "library" => "readon",
                "tech" => "cyberion",
                _ => unimplemented!()
            }
        );

        Ok(
            Store {
                id,
                name,
                num: rng.gen_range(100..200),
                floor: rng.gen_range(0..1),
                size: Size::dummy_with_rng(config, rng),
                week_sales,
                owners: owners.into_iter().map(|(user_id, percentage)|
                    Owner {
                        user_id,
                        income_percentage: percentage
                    }
                ).collect(),
                employees,
            }
        )
    }
}