pub mod users;
pub mod items;
pub mod constants;

pub use users::{ User, CartItem, Job };
pub use items::{
    SimpleItem,
    ItemSale,
    Clothes,
    Food,
    LibraryItem,
    Tech,
    TechOther,
    Cpu,
    Gpu,
    Keyboard,
};

pub trait IsCollection {
    fn coll_name() -> &'static str;
}