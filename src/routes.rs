use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::healthcheck::{graph_healthcheck};
use crate::{GlobalHealthcheckState, Health};
use crate::block::{get_not_indexed_block_count};

#[derive(Serialize)]
pub struct Block {
    number: i64,
}

#[derive(Serialize)]
pub struct HealthcheckResponse {
    health: Health,
}

#[get("/blocks")]
pub fn get_not_indexed_blocks() -> Json<Block> {
    let number = get_not_indexed_block_count()
        .expect("Got not indexed blocks count");
    let response = Block {
        number
    };
    Json(response)
}

#[get("/health")]
pub fn healthcheck(global_state: State<GlobalHealthcheckState>) -> Json<HealthcheckResponse> {
    let health = graph_healthcheck(global_state);
    let response = HealthcheckResponse {
        health
    };
    return Json(response);
}
