use mongodb_mock::{
    common::*,
    clothes::Clothes,
    store::Store,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv::dotenv().ok();

    let mongodb_uri = std::env::var("MONGODB_URI").unwrap();

    let options = ClientOptions::parse(mongodb_uri).resolver_config(ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;

    let db = client.database("nexis");

    let collections = db.list_collection_names().await?;

    for collection in collections {
        println!("Dropping collection: {}", &collection);
        db.collection::<Document>(&collection)
            .drop()
            .await?;
    }

    let mut rng = rand::thread_rng();

    let clothes_coll: Collection<Clothes> = db.collection("clothes");
    let clothes: Vec<Clothes> = (0..50).map(|_| Faker.fake::<Clothes>()).collect();
    let res = clothes_coll.insert_many(clothes).await?;

    let stores_coll: Collection<Store>  = db.collection("store");
    let store: Store = Store::dummy_with_rng("clothes", &client, &fake::Faker, &mut rng).await?;
    let res = stores_coll.insert_one(store).await?;

    //let pay = Payment::dummy_with_rng(64.0, &mut rng);

    //println!("{:#?}", pay);
    Ok(())
}