use mongodb_mock::{
    common::*,
    clothes::Clothes,
    store::Payment,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv::dotenv().ok();

    let mongodb_uri = std::env::var("MONGODB_URI").unwrap();

    let options = ClientOptions::parse(mongodb_uri).resolver_config(ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;

    let database = client.database("nexis");

    let my_coll: Collection<Clothes> = database.collection("clothes");

    let doc = Faker.fake::<Clothes>();

    //let res = my_coll.insert_one(doc).await?;

    let mut rng = rand::thread_rng();
    let pay = Payment::dummy_with_rng(64.0, &mut rng); 

    println!("{:#?}", pay);
    Ok(())
}