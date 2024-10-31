use types::responses::{ ItemSuggestion, ItemResult };

use crate::prelude::*;
use crate::types::{ mongodb::Item, responses };

#[derive(Deserialize, Debug)]
pub struct SearchSuggestionParams {
    input: String,
}

#[tracing::instrument(
    name = "Getting search suggestions",
    skip(db)
)]
#[actix_web::get("/search-suggestions")]
pub async fn search_suggestions(
    parameters: web::Query<SearchSuggestionParams>,
    db: web::Data<mongodb::Database>,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing search suggestions.");

    const MAX_SUGGEST: i32 = 8;

    let items_coll: Collection<Item> = db.collection("items");

    let input = &parameters.input;

    let mut cursor = items_coll.aggregate(vec![
        doc! { "$match": { "$text": { "$search": input, "$caseSensitive": false }}},
        doc! { "$sample": { "size": MAX_SUGGEST }},
    ])
    .await.unwrap();

    let mut suggestions: Vec<responses::ItemSuggestion> = Vec::new();
    let mut added_ids: HashSet<ObjectId> = HashSet::new();

    while let Ok(Some(doc)) = cursor.try_next().await {
        let item: Item = bson::from_document(doc).unwrap();
        if added_ids.insert(item.id) {
            suggestions.push(ItemSuggestion::from(item))
        }
    }

    if suggestions.len() < MAX_SUGGEST as usize {
        let max_suggest_rem: i32 = MAX_SUGGEST - suggestions.len() as i32;

        let mut cursor = items_coll.aggregate(vec![
            doc! { "$match": { "name": { "$regex": input, "$options": "i" }}},
            doc! { "$sample": { "size": max_suggest_rem }}
        ])
        .await.unwrap();

        while let Ok(Some(doc)) = cursor.try_next().await {
            let item: Item = bson::from_document(doc).unwrap();
            if added_ids.insert(item.id) {
                suggestions.push(ItemSuggestion::from(item))
            }
        }
    }

    HttpResponse::Ok().json(suggestions)
}

#[derive(Deserialize, Debug)]
pub struct SearchParams {
    input: String,
    page: i32,
    #[serde(rename = "min-price", skip_serializing_if = "Option::is_none")]
    min_price: Option<i32>,
    #[serde(rename = "max-price", skip_serializing_if = "Option::is_none")]
    max_price: Option<i32>,
}

#[tracing::instrument(
    name = "Getting search results",
    skip(db)
)]
#[actix_web::get("/search")]
pub async fn search(
    parameters: web::Query<SearchParams>,
    db: web::Data<mongodb::Database>,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing search.");

    const MAX_RESULTS: i32 = 15;

    let input = &parameters.input;
    let page = &parameters.page;
    let min_price: Option<i32> = parameters.min_price;
    let max_price: Option<i32> = parameters.max_price;
    let skip = page * MAX_RESULTS;

    let search_aggregate = build_search_pipeline(input, min_price, max_price, skip, MAX_RESULTS, true).await;
        
    let items_coll: Collection<Item> = db.collection("items");

    let mut results: Vec<ItemResult> = Vec::new();
    let mut added_ids: HashSet<ObjectId> = HashSet::new();

    let mut cursor = items_coll.aggregate(search_aggregate).await.expect("Item aggregate failed");
    while let Ok(Some(doc)) = cursor.try_next().await {
        let item: Item = bson::from_document(doc).unwrap();
        if added_ids.insert(item.id) {
            results.push(ItemResult::from(item));
        }
    }

    if results.len() < MAX_RESULTS as usize {
        let search_aggregate = build_search_pipeline(input, min_price, max_price, skip, MAX_RESULTS, false).await;

        let mut cursor = items_coll.aggregate(search_aggregate).await.expect("Item aggregate failed");
        while let Ok(Some(doc)) = cursor.try_next().await {
            let item: Item = bson::from_document(doc).unwrap();
            if added_ids.insert(item.id) {
                results.push(ItemResult::from(item));
            }
        }

    }

    HttpResponse::Ok().json(results)
}

async fn build_search_pipeline(
    input: &str,
    min_price: Option<i32>,
    max_price: Option<i32>,
    skip: i32,
    limit: i32,
    use_text_search: bool
) -> Vec<Document> {
    let price_match = match (min_price, max_price) {
        (Some(min), Some(max)) => doc! { "price": { "$gte": min, "$lte": max }},
        (Some(min), None) => doc! { "price": { "$gte": min }},
        (None, Some(max)) => doc! { "price": { "$lte": max }},
        _ => doc! {}
    };

    let mut text_or_regex_match = if use_text_search {
        doc! { "$text": { "$search": input, "$caseSensitive": false }}
    } else {
        doc! { "name": { "$regex": input, "$options": "i" }}
    };

    text_or_regex_match.extend(price_match);

    let pipeline = vec![
        doc! { "$match": text_or_regex_match },
        doc! { "$skip": skip },
        doc! { "$limit": limit },
    ];

    tracing::debug!(target: "backend", "Search pipeline: {:#?}", pipeline);

    pipeline
}