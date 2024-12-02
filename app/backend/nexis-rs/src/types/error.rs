use thiserror::Error;
use crate::handlers::inventory::add::CsvType;

#[derive(Debug, Error)]
pub enum Redis {
    #[error("Connection error: {0}")]
    ConnError(String),
    #[error("Session expired: {0}")]
    SessionExpired(String),
}

#[derive(Debug, Error)]
pub enum Mongodb {
    #[error("User repetition: {0}")]
    UserAlreadyExists(String),
    #[error("The client's cart is empty")]
    CartIsEmpty,
    #[error("The item is already in the client's cart")]
    CartAlreadyHasItem,
    #[error("The client's cart does not contain this item")]
    ItemNotInCart,
    #[error("Could not find the item")]
    SimpleItemNotFound,
    #[error("At least one of the requested items is sold out")]
    ItemSoldOut,
}

#[derive(Debug, Error)]
pub enum Csv {
    #[error("Unsupported csv type: {0}")]
    UnsupportedType(CsvType),
    #[error("Unsupported csv type: {0}")]
    UnsupportedTypeStr(String),
    #[error("Wrong number of fields. Expected {0} and found {1}")]
    WrongFieldNum(usize, usize),
    #[error("Failed to parse the csv on line {0}, field {1}")]
    ParseError(usize, usize),
    #[error("Missing {0} on line {1}")]
    MissingProperty(&'static str, usize),
    #[error("The csv has an invalid header")]
    InvalidHeader,
}

#[derive(Debug, Error)]
pub enum BadRequest {
    #[error("Role does not exist on user: {0}")]
    NonexistentRole(String),
}