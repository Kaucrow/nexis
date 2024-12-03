pub mod users;
pub mod items;

pub use users::{ User, CartItem };
pub use items::{
    Item,
    SimpleItem,
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