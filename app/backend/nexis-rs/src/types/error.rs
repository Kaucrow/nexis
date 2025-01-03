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
    #[error("User repetition: {0} ")]
    UserAlreadyExists(String)
}