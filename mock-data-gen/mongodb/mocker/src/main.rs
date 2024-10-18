use fake::{Dummy, Fake, Faker};
use fake::faker::name::en::Name;
use rand::rngs::StdRng;
use rand::SeedableRng;
use uuid::Uuid;

#[derive(Debug, Dummy)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    #[dummy(faker = "Name()")]
    customer: String,
    id: Uuid,
    paid: bool,
}

fn main() {
    println!("Hello, world!");

    let f: Foo = Faker.fake();
    println!("{:#?}", f);
    let g: Foo = Faker.fake();
    println!("{:#?}", g);
    let h: Foo = Faker.fake();
    println!("{:#?}", h);
}