use crate::prelude::*;
use anyhow::Result;
use chrono::Utc;
use mongodb::bson::{ DateTime, to_bson };
use types::{
    error,
    database::mongodb::{ User, IsCollection, ItemSale },
};
use utils::{
    common::get_store_from_coll,
    database::items::{
        get_oldest_item_available,
        get_simple_item,
    },
};

pub async fn client_cart_checkout(
    db: &mongodb::Database,
    user_id: ObjectId,
    items: &Vec<String>,
) -> Result<()> {
    client_checkout(db, user_id, items).await?;

    let users_coll: Collection<Document> = db.collection(User::coll_name());

    users_coll.update_one(
        doc! { "_id": user_id },
        doc! { "$unset": { "client.cart": "" }}
    ).await?;

    Ok(())
}

pub async fn client_checkout(
    db: &mongodb::Database,
    user_id: ObjectId,
    items: &Vec<String>,
) -> Result<()> {
    let items: Result<Vec<ObjectId>, _> = items.iter().map(|s| ObjectId::parse_str(s)).collect();

    // Build a Vec containing the item IDs
    let items = match items {
        Ok(items) => items,
        Err(e) => {
            tracing::error!(target: "backend", "Malformed item id: {}", e);
            bail!("Invalid item id provided");
        }
    };

    // Build a Vec containing the data of each item that will be added to the stores' sales
    let sale_items = futures_util::stream::iter(items)
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
    })
    .buffer_unordered(10)   // Limit to 10 concurrent tasks
    .try_collect::<Vec<_>>() // Collect all successful results into a Vec
    .await?;    // Propagate the first error if any

    // Contains the sold items grouped by store
    let mut store_sales: HashMap<&'static str, Vec<ItemSale>> = HashMap::new();

    for item in sale_items {
        store_sales
            .entry(get_store_from_coll(&item.coll)?)
            .or_insert_with(Vec::new)
            .push(item);
    }

    // For each pair of store and sold items, spawn a new task
    futures_util::stream::iter(store_sales)
    .map(|(store_name, items)| {
        let db = db.clone();
        async move {
            // Process each sold item
            let item_results = futures_util::stream::iter(items)
                .map(|item| {
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
                            bail!("Item not found: {:#?}", item);
                        }

                        // Serialize the sale data
                        let serialized_sale = doc! {
                            "date": DateTime::from_system_time(Utc::now().into()),
                            "client": {
                                "user": user_id.clone(),
                            },
                            "item": to_bson(&item)?,
                        };

                        Ok::<_, anyhow::Error>(serialized_sale)
                    }
                })
                .buffer_unordered(10) // Limit concurrency to 10 tasks
                .try_collect::<Vec<_>>()
                .await?;

            // Push all serialized sales into the store's `weekSales`
            let stores_coll: Collection<Document> = db.collection("store");
            stores_coll.update_one(
                doc! { "name": store_name },
                doc! { "$push": { "weekSales": { "$each": item_results }}},
            ).await?;

            Ok::<_, anyhow::Error>(())
        }
    })
    .buffer_unordered(5) // Limit concurrency to 5 stores at a time
    .try_collect::<Vec<_>>()
    .await?;

    Ok(())
}