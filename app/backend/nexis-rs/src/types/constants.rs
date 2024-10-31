use crate::prelude::*;

pub const USER_ID_KEY: &str = "user_id";
pub const USER_EMAIL_KEY: &str = "user_email";

pub const STORE_COLLS: Lazy<HashMap<&str, Vec<&str>>> = Lazy::new(|| HashMap::from([
    ("cyberion", vec![ "tech", "techCpu", "techGpu", "techKeyboard", "techOther" ]),
    ("savoro", vec![ "food" ]),
    ("savoro", vec![ "clothes" ]),
    ("vesti", vec![ "libraryItem" ]),
]));