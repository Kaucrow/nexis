use fake::{
    {Dummy, Fake, Faker},
    faker::name::en::Name,
};
use mongodb::{bson::{doc, Document}, options::{ClientOptions, ResolverConfig, ServerApi, ServerApiVersion}, Client};
use mongodb::Collection;
use uuid::Uuid;
use dotenv;

#[derive(Debug, Dummy)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    #[dummy(faker = "Name()")]
    customer: String,
    id: Uuid,
    paid: bool,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv::dotenv().ok();

    let mongodb_uri = std::env::var("MONGODB_URI").unwrap();

    let options = ClientOptions::parse(mongodb_uri).resolver_config(ResolverConfig::cloudflare()).await?;
    // Create a new client and connect to the server
    let client = Client::with_options(options)?;

    // Get a handle on the movies collection
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    // Find a movie based on the title value
    let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }).await?;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);

    Ok(())
}