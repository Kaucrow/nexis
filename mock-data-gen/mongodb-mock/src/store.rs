use crate::common::*;

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
            _ => doc! {}, // Default to an empty document
        };

        Payment {
            amount,
            payment_type: method_doc,
        } 
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSaleInfo {
    name: Option<String>,
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
pub struct DaySales {
    payment: Payment,
    client: ClientSaleInfo,
    item: Vec<ItemCode>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemCode {
    coll: String,
    _id: ObjectIdWrapper,
    lot: ObjectIdWrapper,
    code: ObjectIdWrapper,
}

fn process_item<T: SimpleItemTrait>(item: T, collection: String, items: &mut Vec<ItemCode>) {
    if let Some(lot) = item.get_lot() {
        if let Some(code) = lot.get_code() {
            items.push(
                ItemCode {
                    coll: collection,
                    _id: item.get_id().clone(),
                    lot: lot.get_id().clone(),
                    code: code.first().expect("").clone(),
                }
            )
        }
    }
}

impl DaySales {
    pub async fn dummy_with_rng<R: rand::Rng + ?Sized>(payment: Payment, client_info: ClientSaleInfo, store: &str, client: &Client, rng: &mut R) -> Result<Self, mongodb::error::Error> {
        let item_colls = match store {
            "clothes" => vec!["clothes"],
            "food" => vec!["food"],
            "library" => vec!["libraryItem"],
            "tech" => vec!["tech", "techCpu", "techGpu", "techKeyboard", "techOther"],
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

            let mut cursor = collection.aggregate(RND_ITEM_PIPELINE.clone()).await?;
            if let Some(res) = cursor.try_next().await? {
                if store == "food" {
                    let item: FoodItemSimple = mongodb::bson::from_document(res)?;
                    process_item(item, collection.name().to_string(), &mut items);
                } else {
                    let item: ItemSimple = mongodb::bson::from_document(res)?;
                    process_item(item, collection.name().to_string(), &mut items);
                }
            }
        }

        Ok(
            DaySales {
                payment,
                client: client_info,
                item: items,
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    owner: ObjectIdWrapper,
    #[serde(rename = "incomePercentage")]
    income_percentage: f64,
}

impl Owner {
    pub fn dummy_with_rng<R: rand::Rng + ?Sized>(owner: ObjectIdWrapper, rng: &mut R) -> Self {
        Owner {
            owner,
            income_percentage: format!("{:.2}", rng.gen_range(1.0..100.0)).parse().expect(""),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    _id: ObjectIdWrapper,
    name: String,
    num: u16,
    floor: u8,
    size: Size,
    #[serde(rename = "daySales")]
    day_sales: Vec<DaySales>,
    owner: Vec<Owner>,
    employee: Vec<ObjectIdWrapper>,
}

impl Store {
    pub async fn dummy_with_rng<R: rand::Rng + ?Sized>(
        store_type: &str,
        store_ids: &HashMap<&str, ObjectIdWrapper>,
        client: &Client,
        config: &Faker,
        rng: &mut R
    ) -> Result<Self, mongodb::error::Error> {
        let id = store_ids.get(store_type).expect(format!("Could not find store of type {store_type} in `store_ids`").as_str());

        let mut day_sales: Vec<DaySales> = Vec::new();

        for _ in 0..rng.gen_range(1..20) {
            day_sales.push(
                DaySales::dummy_with_rng(
                    Payment::dummy_with_rng(rng.gen_range(1.0..100.0), rng),
                    ClientSaleInfo::dummy_with_rng(ObjectIdWrapper::dummy_with_rng(config, rng), rng),
                    store_type,
                    client,
                    rng
                ).await?
            )
        }

        let set_owners: Vec<ObjectIdWrapper> =
            (0..rng.gen_range(1..3)).map(|_|
                ObjectIdWrapper::dummy_with_rng(config, rng)
            ).collect();

        let set_employees: Vec<ObjectIdWrapper> =
            (0..rng.gen_range(3..6)).map(|_|
                ObjectIdWrapper::dummy_with_rng(config, rng)
            ).collect();

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
                _id: id.clone(),
                name,
                num: rng.gen_range(100..200),
                floor: rng.gen_range(0..1),
                size: Size::dummy_with_rng(config, rng),
                day_sales,
                owner: set_owners.into_iter().map(|owner| Owner::dummy_with_rng(owner, rng)).collect(),
                employee: set_employees,
            }
        )
    }
}