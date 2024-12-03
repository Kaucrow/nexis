use crate::prelude::*;
use actix_multipart::form::{
    text::Text,
    tempfile::TempFile,
};

#[derive(Debug, MultipartForm)]
pub struct UploadInventoryForm {
    pub file: TempFile,
    pub store: Text<String>,
    #[multipart(rename = "csvType")]
    pub csv_type: Text<String>,
}