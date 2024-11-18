use crate::prelude::*;
use anyhow::Result;
use types::database::mongodb::ItemSale;
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
) {

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
                anyhow!("At least one of the requested items is sold out")
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

    tracing::info!(target: "backend", "HERE: {:#?}", store_sales);
    
    Ok(())
}