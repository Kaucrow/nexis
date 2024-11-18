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

    let items = match items {
        Ok(items) => items,
        Err(e) => {
            tracing::error!(target: "backend", "Malformed item id: {}", e);
            bail!("Invalid item id provided");
        }
    };

    let mut sale_items: Vec<ItemSale> = Vec::new();

    for item in items {
        let simple_item = get_simple_item(db, item).await?;
        let available_item = get_oldest_item_available(db, &simple_item)
            .await?
            .ok_or(
                anyhow!(error::Mongodb::ItemSoldOut)
            )?;
        
        sale_items.push(ItemSale {
            coll: simple_item.coll,
            item_id: available_item.id,
            lot_id: available_item.lot_id,
            code: available_item.code
        })
    }

    let mut store_sales: HashMap<&'static str, Vec<ItemSale>> = HashMap::new();

    for item in sale_items {
        store_sales
            .entry(get_store_from_coll(&item.coll)?)
            .or_insert_with(Vec::new)
            .push(item);
    }

    for (store_name, items) in store_sales {
        for item in &items {
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
                doc! { "$pull": { "lots.$.codes": item.code }}
            ).await?;

            if res.matched_count == 0 {
                bail!("Item not found: {:#?}", item);
            }
        }

        let serialized_sale: Document = doc! {
            "test": true,
            "date": DateTime::from_system_time(Utc::now().into()),
            "client": {
                "user": user_id,
            },
            "items": to_bson(&items)?,
        };

        let stores_coll: Collection<Document> = db.collection("store");
        stores_coll.update_one(
            doc! { "name": store_name },
            doc! { "$push": { "weekSales": serialized_sale }},
        ).await?;
    }

    Ok(())
}