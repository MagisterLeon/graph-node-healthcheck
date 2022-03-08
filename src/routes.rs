use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::healthcheck::{CurrentHealthcheckState, graph_healthcheck};
use crate::{current_time_as_secs, HealthcheckState};
use crate::block::{get_indexed_block_number, get_latest_block_number, get_not_indexed_block_count};

#[derive(Serialize)]
pub struct Block {
    number: i64,
}

#[get("/blocks")]
pub fn get_not_indexed_blocks() -> Json<Block> {
    let not_indexed_blocks = get_not_indexed_block_count()
        .expect("Got not indexed blocks count");
    let response = Block {
        number: not_indexed_blocks
    };
    Json(response)
}

#[get("/health")]
pub fn healthcheck(previous_state: State<HealthcheckState>) {

    let current_state = CurrentHealthcheckState {
        latest_block_num: get_latest_block_number()
            .expect("Got current latest block num"),
        indexed_block_num: get_indexed_block_number()
            .expect("Got current indexed block num"),
        time: current_time_as_secs()
    };

    graph_healthcheck(previous_state).unwrap();
}
