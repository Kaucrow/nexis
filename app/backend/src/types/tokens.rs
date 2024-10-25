use serde::{ Deserialize, Serialize };
use mongodb::bson::oid::ObjectId;

#[derive(Deserialize, Serialize)]
pub struct ConfirmationToken {
    pub user_id: ObjectId,
}