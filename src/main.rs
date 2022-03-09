#![feature(proc_macro_hygiene, decl_macro)]
#![feature(is_some_with)]

#[macro_use]
extern crate rocket;
extern crate core;

use std::sync::Mutex;
use time::current_time_as_secs;
use crate::api::ApiFacade;
use crate::state::{GlobalHealthcheckState, Health};

mod routes;
mod healthcheck;
mod time;
mod api;
mod state;

fn main() {
    dotenv::dotenv().ok();

    let indexed_block_num = ApiFacade::get_indexed_block_number()
        .expect("Getting indexed block number");
    let latest_block_num = ApiFacade::get_latest_block_number()
        .expect("Getting latest block number");
    let time = current_time_as_secs();

    rocket::ignite()
        .manage(GlobalHealthcheckState::new(
            indexed_block_num,
            latest_block_num,
            time,
            Mutex::new(Health::UP)
        ))
        .mount("/", routes![
            routes::get_not_indexed_blocks,
            routes::healthcheck
        ])
        .launch();
}