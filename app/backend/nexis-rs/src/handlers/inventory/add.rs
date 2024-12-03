use crate::prelude::*;
use anyhow::Result;
use std::{ fs, io::BufReader };
use csv::{ StringRecord, ReaderBuilder };
use async_trait::async_trait;
use chrono::Utc;
use utils::common::parse_to_utc_date;
use types::{
    requests,
    error,
    mongodb::{
        items::{ Material, Book, Lot, FoodLot },
        IsCollection,
        Clothes,
        LibraryItem,
        Food,
    }
};

#[derive(Debug, Copy, Clone)]
pub enum CsvType {
    Clothes,
    LibraryCommon,
    LibraryBook,
}

impl TryFrom<&str> for CsvType {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        match s {
            "clothes" => Ok(CsvType::Clothes),
            "libraryCommon" => Ok(CsvType::LibraryCommon),
            "libraryBook" => Ok(CsvType::LibraryBook),
            _ => bail!(error::Csv::UnsupportedTypeStr(s.to_string()))
        }
    }
}

impl std::fmt::Display for CsvType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clothes => f.write_str("clothes"),
            Self::LibraryCommon => f.write_str("libraryCommon"),
            Self::LibraryBook => f.write_str("libraryBook"),
        }
    }
}

pub async fn add_inventory(
    form: requests::UploadInventoryForm,
    db: &mongodb::Database,
) -> Result<()> {
    let csv_type_str = form.csv_type.0;

    let dir = format!("./inventory/{}/{}", form.store.0, &csv_type_str);
    fs::create_dir_all(&dir)?;

    let csv_type = CsvType::try_from(csv_type_str.as_str())?;

    let f = form.file;

    if f.content_type != Some(mime::TEXT_CSV) {
        tracing::error!("Invalid file type: {:?}", f.content_type);
        bail!("Only CSV files are allowed");
    }

    let path = format!("{}/{}", &dir, f.file_name.ok_or(anyhow!("File has no filename"))?);
    tracing::info!(target: "backend", "Saving inventory to {}", &path);

    f.file.persist(&path).unwrap();

    process_by_type(csv_type, db, &path).await?;

    Ok(())
}

struct CsvHeader(HashMap<String, usize>);

impl CsvHeader {
    pub fn get_idx(&self, field: &str) -> Result<usize> {
        Ok(*self.0.get(field).ok_or(anyhow!(error::Csv::InvalidHeader))?)
    }
}

trait CsvRecordExt {
    fn get_field(&self, field_name: &str, header: &CsvHeader) -> Result<&str>;
}

impl CsvRecordExt for StringRecord {
    fn get_field(&self, field_name: &str, header: &CsvHeader) -> Result<&str> {
        Ok(self.get(header.get_idx(field_name)?).unwrap())
    }
}

async fn get_csv_items<T>(
    csv_path: &str,
    csv_type: CsvType,
) -> Result<Vec<Box<T>>>
where
    T: Csv,
{
    let file = fs::File::open(csv_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(reader);

    let header: CsvHeader = CsvHeader(
        csv_reader
        .headers()
        .map_err(|_| anyhow!(error::Csv::InvalidHeader))?
        .iter()
        .enumerate()
        .map(|(i, field)| (field.to_string(), i))
        .collect());

    let mut items = Vec::new();

    for (i, result) in csv_reader.records().enumerate() {
        let record = result?;

        let item = T::from_csv_record(&header, record, i, &csv_type).await?;
        items.push(item);
    }

    Ok(items)
}

async fn process_by_type(
    csv_type: CsvType,
    db: &mongodb::Database,
    csv_path: &str,
) -> Result<()> {
    match csv_type {
        CsvType::Clothes => {
            let items = get_csv_items::<Clothes>(csv_path, csv_type).await?;
            for item in items { item.mongodb_insert(db).await? };
        }
        CsvType::LibraryCommon | CsvType::LibraryBook => {
            let items = get_csv_items::<LibraryItem>(csv_path, csv_type).await?;
            for item in items { item.mongodb_insert(db).await? };
        }
    };

    Ok(())
}

#[async_trait]
trait Csv: IsCollection + Sized + Send + Sync + serde::Serialize {
    fn expected_fields(csv_type: &CsvType) -> Result<usize>;

    async fn from_csv_record(header: &CsvHeader, r: csv::StringRecord, line: usize, csv_type: &CsvType) -> Result<Box<Self>>;

    async fn mongodb_insert(&self, db: &mongodb::Database) -> Result<()> {
        let coll: Collection<Self> = db.collection(Self::coll_name());
        coll.insert_one(self).await?;
        Ok(())
    }
}

trait CsvParseExt {
    fn parse_ext<T: std::str::FromStr>(&self, line: usize, column: usize) -> Result<T>;
}

impl CsvParseExt for str {
    fn parse_ext<T: std::str::FromStr>(&self, line: usize, column: usize) -> Result<T> {
        self.parse::<T>().map_err(|_| anyhow!(error::Csv::ParseError(line, column))) 
    }
}

fn build_lot(codes: &str) -> Lot {
    let mut lot = Lot {
        id: ObjectId::new(),
        enter_date: Utc::now(),
        codes: Vec::new(),
    };

    for code in codes.split(',') {
        lot.codes.push(ObjectId::new())
    }

    lot
}

#[async_trait]
impl Csv for Clothes {
    fn expected_fields(_: &CsvType) -> Result<usize> { Ok(10) }

    async fn from_csv_record(header: &CsvHeader, r: csv::StringRecord, line: usize, csv_type: &CsvType) -> Result<Box<Self>> {
        if r.len() != Self::expected_fields(csv_type)? {
            bail!(error::Csv::WrongFieldNum(Self::expected_fields(csv_type)?, r.len(), line));
        }

        let colors: Vec<String> = r.get_field("colors", header)?.split(',').map(|color| color.to_string()).collect();

        let materials: Result<Vec<Material>> = r.get_field("materials", header)?.split(',').map(|material| {
            let mut parts = material.splitn(2, ':');
            let material_type = parts.next().ok_or(error::Csv::MissingProperty("material type", line))?;
            let material_percentage = parts.next().ok_or(error::Csv::MissingProperty("material percentage", line))?;
            Ok(Material {
                name: material_type.to_string(),
                percentage: material_percentage.parse_ext(line, 8)?,
            })
        })
        .collect();
        let materials = materials?;

        let lot = build_lot(r.get_field("codes", header)?);
        
        Ok(Box::new(Self {
            id: ObjectId::new(),
            name: r.get_field("name", header)?.to_string(),
            price: r.get_field("price", header)?.parse_ext(line, 1)?,
            age: r.get_field("age", header)?.to_string(),
            size: r.get_field("size", header)?.to_string(),
            gender: r.get_field("gender", header)?.to_string(),
            brand: r.get_field("brand", header)?.to_string(),
            colors,
            clothes_type: r.get_field("clothes_type", header)?.to_string(),
            materials,
            lots: vec![lot],
        }))
    }
}

#[async_trait]
impl Csv for LibraryItem {
    fn expected_fields(csv_type: &CsvType) -> Result<usize> {
        match csv_type {
            CsvType::LibraryCommon => Ok(9),
            CsvType::LibraryBook => Ok(9),
            _ => bail!(error::Csv::UnsupportedType(*csv_type)),
        }
    }

    async fn from_csv_record(header: &CsvHeader, r: csv::StringRecord, line: usize, csv_type: &CsvType) -> Result<Box<Self>> {
        if r.len() != Self::expected_fields(csv_type)? {
            bail!(error::Csv::WrongFieldNum(Self::expected_fields(csv_type)?, r.len(), line));
        }

        match csv_type {
            CsvType::LibraryCommon => {
                let lot = build_lot(r.get_field("codes", header)?);

                Ok(Box::new(Self {
                    id: ObjectId::new(),
                    name: r.get_field("name", header)?.to_string(),
                    price: r.get_field("price", header)?.parse_ext(line, 1)?,
                    book: None,
                    lots: vec![lot],
                }))
            }
            CsvType::LibraryBook => {
                let lot = build_lot(r.get_field("codes", header)?);

                let authors: Vec<String> = r.get_field("authors", header)?.split(',').map(|author| author.to_string()).collect();
                let audiences: Vec<String> = r.get_field("audiences", header)?.split(',').map(|audience| audience.to_string()).collect();
                let genres: Vec<String> = r.get_field("genres", header)?.split(',').map(|genre| genre.to_string()).collect();

                Ok(Box::new(Self {
                    id: ObjectId::new(),
                    name: r.get_field("name", header)?.to_string(),
                    price: r.get_field("price", header)?.parse_ext(line, 1)?,
                    book: Some(Box::new(Book {
                        isbn: r.get_field("isbn", header)?.to_string(),
                        num_pages: r.get_field("num_pages", header)?.parse_ext(line, 3)?,
                        authors,
                        publisher: r.get_field("publisher", header)?.to_string(),
                        edition: r.get_field("edition", header)?.parse_ext(line, 6)?,
                        audiences,
                        genres,
                    })),
                    lots: vec![lot],
                }))
            }
            _ => bail!(error::Csv::UnsupportedType(*csv_type))
        }
    }
}

#[async_trait]
impl Csv for Food {
    fn expected_fields(_: &CsvType) -> Result<usize> { Ok(5) }

    async fn from_csv_record(header: &CsvHeader, r: csv::StringRecord, line: usize, csv_type: &CsvType) -> Result<Box<Self>> {
        if r.len() != Self::expected_fields(csv_type)? {
            bail!(error::Csv::WrongFieldNum(Self::expected_fields(csv_type)?, r.len(), line));
        }

        let price: Option<f64> = 
            match r.get_field("price", header)? {
                field if field.is_empty() => None,
                field => Some(field.parse::<f64>().map_err(|e| anyhow!(e))?),
            };

        let price_per_kg: Option<f64> = 
            match r.get_field("price_per_kg", header)? {
                field if field.is_empty() => None,
                field => Some(field.parse::<f64>().map_err(|e| anyhow!(e))?),
            };

        let lot: FoodLot = {
            let mut lot = FoodLot {
                id: ObjectId::new(),
                enter_date: Utc::now(),
                expiry: parse_to_utc_date(r.get_field("expiry", header)?)?,
                codes: Vec::new(),
            };

            for code in r.get_field("codes", header)?.split(',') {
                lot.codes.push(ObjectId::new())
            }

            lot
        };

        Ok(Box::new(Self {
            id: ObjectId::new(),
            name: r.get_field("name", header)?.to_string(),
            price,
            price_per_kg,
            food_type: r.get_field("type", header)?.to_string(),
            lots: vec![lot],
        }))
    }
}