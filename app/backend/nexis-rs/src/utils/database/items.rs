use crate::prelude::*;
use anyhow::Result;
use serde::de::DeserializeOwned;
use async_trait::async_trait;
use types::{
    error,
    mongodb::{
        items::Lot,
        SimpleItem,
    },
};

#[tracing::instrument(
    name = "Getting simple item",
    skip(db)
)]
pub async fn get_simple_item(
    db: &mongodb::Database,
    item_id: ObjectId,
) -> Result<SimpleItem> {
    let items_coll: Collection<SimpleItem> = db.collection("items");
    let item = items_coll.find_one( doc! { "_id": item_id }).await?.ok_or_else(|| anyhow!(error::Mongodb::SimpleItemNotFound))?;

    Ok(item)
}

#[derive(Debug)]
pub struct AvailableItem {
    pub id: ObjectId,
    pub lot_id: ObjectId,
    pub code: ObjectId,
    pub remaining: i32,
}

#[tracing::instrument(
    name = "Getting oldest item available",
    skip(db)
)]
pub async fn get_oldest_item_available(
    db: &mongodb::Database,
    item: &SimpleItem,
) -> Result<Option<AvailableItem>> {
    let coll_name = &item.coll;

    let item_coll: Collection<Document> = db.collection(coll_name);

    let mut item_cursor = item_coll
        .find(
            doc! { "_id": item.id },
        )
        .projection(doc! { "lots": 1 })
        .await?;

    if let Some(Ok(item_doc)) = item_cursor.next().await {
        let lots = item_doc.get("lots").ok_or(anyhow!("Item does not have `lots` field"))?;
        let lots_value = serde_json::to_value(lots)?;
        let mut lots: Vec<Lot> = serde_json::from_value(lots_value)?;

        lots.sort_by(|a, b| a.enter_date.cmp(&b.enter_date));

        let lot = lots.iter().find(|lot| !lot.codes.is_empty());

        if let Some(lot) = lot {
            let code = lot.codes.first().unwrap().to_owned();
            let remaining = lots.iter().map(|lot| lot.codes.iter().count() as i32).sum();
            Ok(Some(AvailableItem {
                id: item.id,
                remaining,
                lot_id: lot.id,
                code,
            }))
        } else {
            Ok(None)
        }
    } else {
        bail!("Item not found")
    }
}