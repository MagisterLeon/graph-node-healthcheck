#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicIsize};
use std::sync::Mutex;
use web3::types::RewardType::Block;
use healthcheck::get_not_indexed_block_count;
use api::{BlockApi};
use time::current_time_as_secs;
use crate::healthcheck::{get_indexed_block_number, get_latest_block_number};

mod routes;
mod healthcheck;
mod time;
mod api;
mod errors;

pub struct Config {
    api: BlockApi,
}

pub struct HealthcheckState {
    indexed_block_num: AtomicIsize,
    latest_block_num: AtomicIsize,
    time: AtomicIsize,
    is_ok: Mutex<bool>,
}

impl HealthcheckState {
    pub fn new(indexed_block_num: i64, latest_block_num: i64, time: u64, is_ok: Mutex<bool>) -> Self {
        Self {
            indexed_block_num: AtomicIsize::new(indexed_block_num as isize),
            latest_block_num: AtomicIsize::new(latest_block_num as isize),
            time: AtomicIsize::new(time as isize),
            is_ok,
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    let api = BlockApi {};

    let indexed_block_num = get_indexed_block_number(&api)
        .expect("Getting indexed block number");
    let latest_block_num = get_latest_block_number(&api)
        .expect("Getting latest block number");
    let time = current_time_as_secs();

    rocket::ignite()
        .manage(Config {
            api
        })
        .manage(HealthcheckState::new(
            latest_block_num,
            latest_block_num,
            time,
            Mutex::new(true)
        ))
        .mount("/", routes![
            routes::get_not_indexed_blocks,
            // routes::healthcheck
        ])
        .launch();
}