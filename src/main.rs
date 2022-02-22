#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

use node::get_block_number;

mod node;
mod subgraph;

#[get("/")]
async fn get_not_indexed_blocks() {
    let block_number = get_block_number().await;
    println!("Block number: {:?}", &block_number.unwrap());
}

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    rocket::build().mount("/", routes![get_not_indexed_blocks])
}