use crate::prelude::*;
use crate::types::STORE_COLLS;
use anyhow::Result;

pub fn get_store_from_coll(coll: &str) -> Result<&'static str> {
    for (store, colls) in STORE_COLLS.iter() {
        if colls.contains(&coll) {
            return Ok(store);
        }
    }

    bail!("Could not find a store that had `{}` as a collection", coll);
}