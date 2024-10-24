use crate::common::*;
use std::collections::HashSet;
use once_cell::sync::Lazy;

static AUDIENCES: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "children", "preteens", "teens", "youngads", "adults"
]);

static GENRES: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "fantasy", "horror", "romance", "drama", "mystery", "scifi", "historical", "academic"
]);

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    isbn: String,
    #[serde(rename = "numPages")]
    num_pages: u32,
    author: Vec<String>,
    publisher: String,
    edition: u8,
    audience: Vec<String>,
    genre: Vec<String>,
}

impl Dummy<Faker> for Book {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let mut used_audiences: HashSet<&str> = HashSet::new();
        let mut used_genres: HashSet<&str> = HashSet::new();

        Book {
            isbn: Isbn().fake(),
            num_pages: rng.gen_range(3..1500),
            author: (0..rng.gen_range(1..3)).map(|_| Name().fake()).collect(),
            publisher: Word().fake(),
            edition: rng.gen_range(1..4),
            audience: (0..rng.gen_range(1..4)).filter_map(|_| {
                let audience = AUDIENCES.choose(rng).unwrap();
                if used_audiences.insert(audience) {
                    Some(audience.to_string())
                } else {
                    None
                }
            }).collect(),
            genre: (0..rng.gen_range(1..2)).filter_map(|_| {
                let genre = GENRES.choose(rng).unwrap();
                if used_genres.insert(genre) {
                    Some(genre.to_string())
                } else {
                    None
                }
            }).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryItem {
    _id: ObjectIdWrapper,
    name: String,
    price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    book: Option<Box<Book>>,
    lot: Vec<Lot>,
}

impl Dummy<Faker> for LibraryItem {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        LibraryItem {
            _id: ObjectIdWrapper::dummy_with_rng(config, rng),
            name: Word().fake(),
            price: format!("{:.2}", rng.gen_range(1.0..200.0)).parse().unwrap(),
            book:
                if rng.gen_bool(0.5) == true {
                    Some(Box::new(Book::dummy_with_rng(config, rng)))
                } else {
                    None
                },
            lot: (0..rng.gen_range(1..5)).map(|_| Lot::dummy_with_rng(config, rng)).collect(),
        } 
    }
}