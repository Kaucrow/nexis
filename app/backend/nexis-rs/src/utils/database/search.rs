use crate::prelude::*;
use serde_json::Value;
use types::{
    responses::ITEM_DETAILS_REG,
    error,
    mongodb::{
        SimpleItem,
        Tech,
        Item,
    }
};
use utils::database::items::get_simple_item;
use anyhow::Result;

static DB_COLLS: Lazy<Vec<&'static str>> = Lazy::new(|| vec![Tech::coll_name()]);

pub async fn get_item_details(
    db: &mongodb::Database, item_id: ObjectId
) -> Result<Value> {
    let item = get_simple_item(db, item_id).await?;

    let coll_name = item.coll;

    match ITEM_DETAILS_REG.get_item_details(db, &coll_name, item_id).await {
        Some(item) => {
            if DB_COLLS.contains(&coll_name.as_str()) {
                Ok(item.details(Some(db)).await?)
            } else {
                Ok(item.details(None).await?)
            }
        }
        None => bail!("An error was produced. Check the logs for more details.")
    }
}