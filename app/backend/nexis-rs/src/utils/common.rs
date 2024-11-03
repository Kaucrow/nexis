use crate::prelude::*;
use crate::types::STORE_COLLS;
use anyhow::Result;

pub fn get_store_from_coll(coll: &str) -> Result<String> {
    for (store, colls) in STORE_COLLS.iter() {
        if colls.contains(&coll) {
            return Ok(store.to_string());
        }
    }

    bail!("Could not find a store that had `{}` as a collection", coll);
}