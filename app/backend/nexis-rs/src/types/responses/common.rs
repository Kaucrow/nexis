use crate::prelude::*;

#[derive(Serialize)]
pub struct Success {
    message: String,
}

impl Success {
    pub fn new(message: &'static str) -> Self {
        Success {
            message: message.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Error {
    error: String,
}

impl Error {
    pub fn new(err_obj: anyhow::Error) -> Self {
        Error {
            error: err_obj.to_string(),
        }
    }

    pub fn simple(error: &'static str) -> Self {
        Error {
            error: error.to_string(),
        }
    }

    pub fn detailed(error: &'static str, err_obj: anyhow::Error) -> Self {
        Error {
            error: format!("{}: {}", error, err_obj),
        }
    }

    pub fn from_str(error: String) -> Self {
        Error {
            error,
        }
    }
}
