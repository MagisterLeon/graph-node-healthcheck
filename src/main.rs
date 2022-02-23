#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::Serialize;

use node::get_block_number;

use crate::graph::get_indexed_block_number;

mod node;
mod graph;

#[derive(Serialize)]
struct Block {
    number: u64,
}

#[get("/")]
fn get_not_indexed_blocks() -> Json<Block> {
    let indexed_block = match get_indexed_block_number() {
        Ok(res) => res,
        Err(error) => panic!("Getting indexed block numer failed: {:?}", error),
    };

    let latest_block = match get_block_number() {
        Ok(res) => res,
        Err(error) => panic!("Getting latest block numer failed: {:?}", error)
    };
    println!("Indexed block: {:?}", &indexed_block);
    println!("Latest block: {:?}", &latest_block);

    let not_indexed_blocks = u64::try_from(latest_block - indexed_block).unwrap();
    let response = Block {
        number: not_indexed_blocks
    };

    Json(response)
}

fn main() {
    dotenv::dotenv().ok();
    rocket::ignite().mount("/", routes![get_not_indexed_blocks]).launch();
}