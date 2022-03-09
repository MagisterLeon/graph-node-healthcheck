use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::healthcheck::{graph_healthcheck};
use crate::{ApiFacade, GlobalHealthcheckState, Health};

#[derive(Serialize)]
pub struct NotIndexedBlocksResponse {
    number: i64,
}

#[derive(Serialize)]
pub struct HealthcheckResponse {
    health: Health,
}

#[get("/blocks")]
pub fn get_not_indexed_blocks() -> Json<NotIndexedBlocksResponse> {
    let number = ApiFacade::get_not_indexed_block_count()
        .expect("Got not indexed blocks count");
    let response = NotIndexedBlocksResponse {
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
