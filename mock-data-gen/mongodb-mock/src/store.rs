use crate::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    length: f64,
    width: f64,
    height: f64, 
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSaleInfo {
    coll: String,
    lot: ObjectIdWrapper,
    item: ObjectIdWrapper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DaySales {
    payment: Payment,
    client: ClientSaleInfo,
    item: Vec<ItemSaleInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    coll: String,
    owner: Vec<Document>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    coll: String,
    employee: Vec<ObjectIdWrapper>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    _id: ObjectIdWrapper,
    name: String,
    num: u16,
    floor: u8,
    size: Size,
    #[serde(rename = "daySales")]
    day_sales: DaySales,
    owner: Owner,
    employee: Employee,
}