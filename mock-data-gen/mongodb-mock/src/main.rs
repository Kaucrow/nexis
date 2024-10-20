use mongodb_mock::{
    common::*,
    clothes::Clothes,
    store::Store,
    food::Food,
    library::LibraryItem,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv::dotenv().ok();

    let mongodb_uri = std::env::var("MONGODB_URI").unwrap();

    let options = ClientOptions::parse(mongodb_uri).resolver_config(ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;

    let db = client.database("nexis");

    let collections = db.list_collection_names().await?;

    println!("======== Dropping collections ========");
    for collection in collections {
        println!("- Dropped: {}", &collection);
        db.collection::<Document>(&collection)
            .drop()
            .await?;
    }

    println!("======== Inserting collections ========");
    let mut rng = rand::thread_rng();

    let clothes_coll: Collection<Clothes> = db.collection("clothes");
    let clothes: Vec<Clothes> = (0..50).map(|_| Faker.fake::<Clothes>()).collect();
    clothes_coll.insert_many(clothes).await?;
    println!("- Inserted: clothes");

    let food_coll: Collection<Food> = db.collection("food");
    let food: Vec<Food> = (0..50).map(|_| Faker.fake::<Food>()).collect();
    food_coll.insert_many(food).await?;
    println!("- Inserted: food");
    
    let library_item_coll: Collection<LibraryItem> = db.collection("libraryItem");
    let library_items: Vec<LibraryItem> = (0..50).map(|_| Faker.fake::<LibraryItem>()).collect();
    library_item_coll.insert_many(library_items).await?;
    println!("- Inserted: library items");

    let stores_coll: Collection<Store> = db.collection("store");

    let store: Store = Store::dummy_with_rng("clothes", &client, &fake::Faker, &mut rng).await?;
    stores_coll.insert_one(store).await?;
    println!("- Inserted: clothes store");
    
    let store: Store = Store::dummy_with_rng("food", &client, &fake::Faker, &mut rng).await?;
    stores_coll.insert_one(store).await?;
    println!("- Inserted: food store");

    let store: Store = Store::dummy_with_rng("library", &client, &fake::Faker, &mut rng).await?;
    stores_coll.insert_one(store).await?;
    println!("- Inserted: library store");

    Ok(())
}