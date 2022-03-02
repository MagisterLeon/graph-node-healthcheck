use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::healthcheck::{get_not_indexed_block_count, graph_healthcheck};
use crate::{Config, HealthcheckState};

#[derive(Serialize)]
pub struct Block {
    number: i64,
}

#[get("/")]
pub fn get_not_indexed_blocks(config: State<Config>) -> Json<Block> {
    let not_indexed_blocks = get_not_indexed_block_count(&config.api);
    let response = Block {
        number: not_indexed_blocks
    };
    Json(response)
}

#[get("/healthcheck")]
pub fn healthcheck(config: State<Config>, healthcheck_state: State<HealthcheckState>) {
    graph_healthcheck(&config.api, healthcheck_state);
}
