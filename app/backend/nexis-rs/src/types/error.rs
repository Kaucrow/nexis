use thiserror::Error;

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
pub enum BadRequest {
    #[error("Role does not exist on user: {0}")]
    NonexistentRole(String),
}