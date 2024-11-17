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
use anyhow::Result;

static DB_COLLS: Lazy<Vec<&'static str>> = Lazy::new(|| vec![Tech::coll_name()]);

pub async fn get_item_details(
    db: &mongodb::Database, item_id: ObjectId
) -> Result<Value> {
    let items_coll: Collection<SimpleItem> = db.collection("items");
    
    let item = items_coll.find_one( doc! { "_id": item_id }).await?.ok_or_else(|| anyhow!(error::Mongodb::SimpleItemNotFound))?;

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