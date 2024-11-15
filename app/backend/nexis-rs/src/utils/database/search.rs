use crate::prelude::*;
use serde_json::Value;
use types::{ responses::ITEM_DETAILS_REG, mongodb::SimpleItem };
use anyhow::Result;

pub async fn get_item_details(
    db: &mongodb::Database, item_id: ObjectId
) -> Result<Value> {
    let items_coll: Collection<SimpleItem> = db.collection("items");
    
    let item = items_coll.find_one( doc! { "_id": item_id }).await?.ok_or_else(|| anyhow!("Failed to find the simple item."))?;

    let coll_name = item.coll;

    match ITEM_DETAILS_REG.get_item_details(db, &coll_name, item_id).await {
        Some(item) => match coll_name.as_str() {
            "tech" => Ok(item.details(Some(db)).await?),
            _ => Ok(item.details(None).await?),
        }
        None => bail!("An error was produced. Check the logs for more details.")
    }
}