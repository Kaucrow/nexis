use crate::prelude::*;
use anyhow::Result;
use chrono::{ DateTime, Utc };
use utils::database::users::get_client_cart;
use types::{
    responses,
    mongodb::{ SimpleItem, User, IsCollection },
};

#[tracing::instrument(
    name = "Getting client cart from DB with item details"
    skip(db, user_id),
)]
pub async fn get_client_cart_details(
    db: &mongodb::Database,
    user_id: ObjectId,
) -> Result<Vec<responses::CartItem>> {
    let user_cart = get_client_cart(db, user_id).await?;

    let client_doc = user_cart.get_document("client")?;
    let cart_items = if let Some(cart) = client_doc.get_array("cart").ok() {
        cart
    } else {
        bail!(types::error::Mongodb::CartIsEmpty);
    };

    let tasks: Vec<_> = cart_items.iter().map(|item| {
        let item_doc = item.as_document().unwrap().clone();
        let db = db.clone();

        tokio::task::spawn(async move {
            let item_id = item_doc.get_object_id("item")?;
            let collection_name = item_doc.get_str("coll")?.to_string();
            // TODO: Fix date_added not working
            //let date_added = item_doc.get_datetime("dateAdded")?.clone();

            let collection: Collection<Document> = db.collection(&collection_name);

            let item_details = collection
                .find_one(doc! { "_id": item_id })
                .await?
                .ok_or_else(|| anyhow!("Item not found in collection {}", collection_name))?;

            let in_stock = item_details
                .get_array("lot")
                .map_or(false, |lots| {
                    lots.iter().any(|lot| {
                        lot.as_document()
                            .and_then(|doc| doc.get_array("code").ok())
                            .map_or(false, |codes| !codes.is_empty())
                    })
                });

            let store = utils::get_store_from_coll(&collection_name)?;

            Ok(responses::CartItem {
                id: item_id.to_hex(),
                name: item_details.get_str("name")?.to_string(),
                price: item_details.get_f64("price").or_else(|_| item_details.get_f64("pricePerKg"))?,
                store: store.to_string(),
                in_stock,
            })
        })
    }).collect();

    let results = futures_util::future::try_join_all(tasks).await?; 

    Ok(results.into_iter().collect::<Result<Vec<_>>>()?)
}

#[tracing::instrument(
    name = "Deleting a client's cart item from DB",
    skip(db, user_id, item_id),
)]
pub async fn delete_client_cart_item(
    db: &mongodb::Database,
    user_id: ObjectId,
    item_id: ObjectId,
) -> Result<()> {
    let users_coll: Collection<User> = db.collection(User::coll_name());

    let update_result = users_coll.update_one(
        doc! { "_id": user_id },
        doc! { "$pull": { "client.cart": { "item": item_id }}},
    ).await?;

    if update_result.matched_count == 0 {
        bail!("User not found.");
    }

    if update_result.modified_count == 0 {
        bail!(types::error::Mongodb::ItemNotInCart);
    }

    Ok(())
}

#[tracing::instrument(
    name = "Inserting a client's cart item into DB",
    skip(db, user_id, item_id),
)]
pub async fn insert_client_cart_item(
    db: &mongodb::Database,
    user_id: ObjectId,
    item_id: ObjectId,
) -> Result<()> {
    let items_coll: Collection<SimpleItem> = db.collection(SimpleItem::coll_name());

    let item = items_coll.find_one(
        doc! { "_id": item_id }
    )
    .await?
    .ok_or_else(|| anyhow!("Item not found."))?;

    let users_coll: Collection<User> = db.collection(User::coll_name());

    let now: DateTime<Utc> = Utc::now();
    let date_added = bson::DateTime::from_millis(now.timestamp_millis());

    let update_result = users_coll.update_one(
        doc! { "_id": user_id, "client.cart.item": { "$ne": item.id }},
        doc! {
            "$push": {
                "client.cart": {
                    "item": item.id,
                    "coll": item.coll,
                    "dateAdded": date_added,
                }
            }
        },
    ).await?;

    if update_result.matched_count == 0 {
        bail!(types::error::Mongodb::CartAlreadyHasItem)
    }

    Ok(())
}