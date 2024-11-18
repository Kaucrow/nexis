use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Checkout {
    pub cart: bool,
    pub items: Vec<String>, 
}