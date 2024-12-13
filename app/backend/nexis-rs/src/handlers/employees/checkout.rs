use crate::prelude::*;
use anyhow::Result;
use bson::DateTime;
use types::{
    mongodb::ItemSale,
    error,
    employees,
};
use utils::database::items::{
    get_oldest_item_available,
    get_simple_item,
};

pub async fn client_checkout(
    db: &mongodb::Database,
    details: &employees::Checkout,
) -> Result<()> {
    let items: Result<Vec<ObjectId>, _> = details.items.iter().map(|s| ObjectId::parse_str(s)).collect();

    // Build a Vec containing the item IDs
    let items = match items {
        Ok(items) => items,
        Err(e) => {
            tracing::error!(target: "backend", "Malformed item id: {}", e);
            bail!("Invalid item id provided");
        }
    };

    // Build a Vec containing the data of each item that will be added to the stores' sales
    let sale_items = futures_util::stream::iter(items.into_iter()
    .map(|item| {
        let db = db.clone();
        async move {
            let simple_item = get_simple_item(&db, item).await?;
            let available_item = get_oldest_item_available(&db, &simple_item)
                .await?
                .ok_or(anyhow!(error::Mongodb::ItemSoldOut))?;
            
            Ok::<_, anyhow::Error>(ItemSale {
                coll: simple_item.coll,
                item_id: available_item.id,
                lot_id: available_item.lot_id,
                code: available_item.code,
            })
        }
    }))
    .buffer_unordered(10)   // Limit to 10 concurrent tasks
    .try_collect::<Vec<_>>()    // Collect all successful results into a Vec
    .await?;    // Propagate the first error if any

    // Delete each sold item from its collection
    futures_util::stream::iter(sale_items.iter().map(|item| {
        let db = db.clone();
        async move {
            // Removing the item's code from its collection
            let coll: Collection<Document> = db.collection(&item.coll);
            let res = coll.update_one(
                doc! {
                    "_id": item.item_id,
                    "lots": {
                        "$elemMatch": {
                            "_id": item.lot_id,
                            "codes": item.code,
                        }
                    }
                },
                doc! { "$pull": { "lots.$.codes": item.code }},
            ).await?;

            // Is true if either the item or its code was not found on its collection
            if res.matched_count == 0 {
                return Err::<(), anyhow::Error>(anyhow!(format!("Item not found: {:#?}", item)));
            }
                
            Ok::<(), anyhow::Error>(())
        }
    }))
    .buffer_unordered(10)
    .try_collect::<Vec<()>>()
    .await?;

    // Push all serialized sales into the store's `weekSales`
    let stores_coll: Collection<Document> = db.collection("store");
    stores_coll.update_one(
        doc! { "name": details.store.clone() },
        doc! { "$push": { "weekSales": {
            "date": DateTime::from_system_time(Utc::now().into()),
            "payment": {},
            "client": {
                "name": &details.client_name,
            },
            "items": bson::to_bson(&sale_items)?
        }}},
    ).await?;

    Ok(())
}