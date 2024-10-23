use std::{iter::zip, mem::uninitialized};

use mongodb_mock::{
    common::*,
    clothes::Clothes,
    store::Store,
    food::Food,
    library::LibraryItem,
    tech::{ Cpu, Gpu, Tech, Keyboard, TechOther },
    user::User,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv::dotenv().ok();

    let mongodb_uri = 
        if let Ok(uri) = std::env::var("MONGODB_URI") { uri }
        else { panic!("Make sure that `.env` file exists and contains the `MONGODB_URI` env variable.") };

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

    let cpu_coll: Collection<Cpu> = db.collection("techCpu");
    let cpus: Vec<Cpu> = (0..50).map(|_| Faker.fake::<Cpu>()).collect();
    cpu_coll.insert_many(cpus).await?;
    println!("- Inserted: CPUs");

    let gpu_coll: Collection<Gpu> = db.collection("techGpu");
    let gpus: Vec<Gpu> = (0..50).map(|_| Faker.fake::<Gpu>()).collect();
    gpu_coll.insert_many(gpus).await?;
    println!("- Inserted: GPUs");

    let cpu_coll: Collection<Document> = db.collection("techCpu");
    let mut cursor = cpu_coll.aggregate(get_rnd_item_pipeline(50)).await?;
    let mut rnd_cpus: Vec<ItemSimple> = Vec::new();
    loop {
        if let Some(res) = cursor.try_next().await? {
            rnd_cpus.push(mongodb::bson::from_document::<ItemSimple>(res)?);
        } else {
            break;
        }
    }

    let gpu_coll: Collection<Document> = db.collection("techGpu");
    let mut cursor = gpu_coll.aggregate(get_rnd_item_pipeline(50)).await?;
    let mut rnd_gpus: Vec<ItemSimple> = Vec::new();
    loop {
        if let Some(res) = cursor.try_next().await? {
            rnd_gpus.push(mongodb::bson::from_document::<ItemSimple>(res)?);
        } else {
            break;
        }
    }
    
    let rnd_cpus_gpus = zip(rnd_cpus, rnd_gpus);
    
    let tech_coll: Collection<Tech> = db.collection("tech");
    let techs: Vec<Tech> =
        rnd_cpus_gpus.map(|(cpu, gpu)| {
            let gpu =
                if rng.gen_bool(0.5) { Some(gpu._id) }
                else { None };

            Tech::dummy_with_rng(cpu._id, gpu, &Faker, &mut rng)
        }).collect();
    tech_coll.insert_many(techs).await?;
    println!("- Inserted: techs");

    let keyb_coll: Collection<Keyboard> = db.collection("techKeyboard");
    let keybs: Vec<Keyboard> = (0..=50).map(|_| Keyboard::dummy_with_rng(&Faker, &mut rng)).collect();
    keyb_coll.insert_many(keybs).await?;
    println!("- Inserted: keyboards");

    let tech_other_coll: Collection<TechOther> = db.collection("techOther");
    let tech_others: Vec<TechOther> = (0..=50).map(|_| TechOther::dummy_with_rng(&Faker, &mut rng)).collect();
    tech_other_coll.insert_many(tech_others).await?;
    println!("- Inserted: other techs");

    let store_ids: HashMap<&str, ObjectIdWrapper> = (0..4).map(|i| {
        match i {
            0 => ("clothes", ObjectIdWrapper::dummy_with_rng(&Faker, &mut rng)),
            1 => ("food", ObjectIdWrapper::dummy_with_rng(&Faker, &mut rng)),
            2 => ("library", ObjectIdWrapper::dummy_with_rng(&Faker, &mut rng)),
            3 => ("tech", ObjectIdWrapper::dummy_with_rng(&Faker, &mut rng)),
            _ => unimplemented!(),
        }
    }).collect();

    let users_coll: Collection<User> = db.collection("user");
    let mut users: Vec<User> = Vec::new();
    for _ in 0..50 {
        users.push(User::dummy_with_rng(&store_ids, &client, &Faker, &mut rng).await);
    }
    users_coll.insert_many(users).await?;
    println!("- Inserted: users");

    let stores_coll: Collection<Store> = db.collection("store");

    let store: Store = Store::dummy_with_rng("clothes", &store_ids, &client, &fake::Faker, &mut rng).await?;
    stores_coll.insert_one(store).await?;
    println!("- Inserted: clothes store");
    
    let store: Store = Store::dummy_with_rng("food", &store_ids, &client, &fake::Faker, &mut rng).await?;
    stores_coll.insert_one(store).await?;
    println!("- Inserted: food store");

    let store: Store = Store::dummy_with_rng("library", &store_ids, &client, &fake::Faker, &mut rng).await?;
    stores_coll.insert_one(store).await?;
    println!("- Inserted: library store");

    Ok(())
}