use std::error::Error;
use crate::api::{get_indexed_block_num, get_latest_block_num};

pub fn get_not_indexed_block_count() -> Result<i64, Box<dyn Error>> {
    let indexed_block = get_indexed_block_number()?;
    let latest_block = get_latest_block_number()?;
    Ok(latest_block - indexed_block)
}

pub fn get_indexed_block_number() -> Result<i64, Box<dyn Error>> {
    let indexed_block = get_indexed_block_num()?;
    println!("Indexed block: {}", &indexed_block);
    Ok(indexed_block)
}

pub fn get_latest_block_number() -> Result<i64, Box<dyn Error>> {
    let latest_block = get_latest_block_num()?;
    println!("Latest block: {}", &latest_block);
    Ok(latest_block)
}
