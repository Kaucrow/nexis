use thiserror::Error;

#[derive(Debug, Error)]
pub enum Redis {
    #[error("Connection error: {0}")]
    ConnError(String)
}