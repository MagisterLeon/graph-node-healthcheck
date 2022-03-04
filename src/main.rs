#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicIsize};
use std::sync::Mutex;
use healthcheck::get_not_indexed_block_count;
use api::{BlockApi};
use time::current_time_as_secs;

mod routes;
mod healthcheck;
mod time;
mod api;
mod errors;

pub struct Config {
    api: BlockApi,
}

pub struct HealthcheckState {
    not_indexed_blocks_count: AtomicIsize,
    time: AtomicIsize,
    is_ok: Mutex<bool>,
}

impl HealthcheckState {
    pub fn new(not_indexed_blocks_count: i64, time: u64, is_ok: Mutex<bool>) -> Self {
        Self {
            not_indexed_blocks_count: AtomicIsize::new(not_indexed_blocks_count as isize),
            time: AtomicIsize::new(time as isize),
            is_ok,
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    let api = BlockApi {};

    let not_indexed_blocks_count = get_not_indexed_block_count(&api)
        .expect("Getting not indexed blocks on startup");
    let time = current_time_as_secs();

    rocket::ignite()
        .manage(Config {
            api
        })
        .manage(HealthcheckState::new(not_indexed_blocks_count, time, Mutex::new(true)),
        )
        .mount("/", routes![
            routes::get_not_indexed_blocks,
            routes::healthcheck
        ])
        .launch();
}