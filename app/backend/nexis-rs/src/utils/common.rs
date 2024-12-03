use crate::prelude::*;
use crate::types::STORE_COLLS;
use anyhow::Result;
use chrono::{ DateTime, TimeZone, NaiveDate, Utc };

pub fn get_store_from_coll(coll: &str) -> Result<&'static str> {
    for (store, colls) in STORE_COLLS.iter() {
        if colls.contains(&coll) {
            return Ok(store);
        }
    }

    bail!("Could not find a store that had `{}` as a collection", coll);
}

pub fn parse_to_utc_date(date_str: &str) -> Result<DateTime<Utc>> {
    // Parse the input date string in "DD-MM-YYYY" format
    let parsed_date = NaiveDate::parse_from_str(date_str, "%d-%m-%Y")?;

    // Create a datetime at midnight in UTC
    let naive_datetime = parsed_date.and_hms_opt(0, 0, 0).ok_or(anyhow!(""))?;

    // Convert NaiveDateTime to DateTime<Utc>
    let utc_datetime = Utc.from_utc_datetime(&naive_datetime);

    Ok(utc_datetime)
}