use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::healthcheck::{get_not_indexed_block_count};
use crate::{Config, HealthcheckState};

#[derive(Serialize)]
pub struct Block {
    number: i64,
}

#[get("/blocks")]
pub fn get_not_indexed_blocks(config: State<Config>) -> Json<Block> {
    let not_indexed_blocks = get_not_indexed_block_count(&config.api)
        .expect("Getting not indexed blocks");
    let response = Block {
        number: not_indexed_blocks
    };
    Json(response)
}
//
// #[get("/health")]
// pub fn healthcheck(config: State<Config>, healthcheck_state: State<HealthcheckState>) {
//     graph_healthcheck(&config.api, healthcheck_state).expect("Checking graph health");
// }
