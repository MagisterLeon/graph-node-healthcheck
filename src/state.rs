use serde::Serialize;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Mutex;
use rocket::State;
use crate::{ApiFacade, current_time_as_secs};

#[derive(Copy, Clone, Serialize, Debug)]
pub enum Health {
    UP,
    DOWN,
}

pub struct GlobalHealthcheckState {
    pub indexed_block_num: AtomicIsize,
    pub latest_block_num: AtomicIsize,
    pub time: AtomicIsize,
    pub health: Mutex<Health>,
}

impl GlobalHealthcheckState {
    pub fn new(indexed_block_num: i64, latest_block_num: i64, time: u64, health: Mutex<Health>) -> Self {
        Self {
            indexed_block_num: AtomicIsize::new(indexed_block_num as isize),
            latest_block_num: AtomicIsize::new(latest_block_num as isize),
            time: AtomicIsize::new(time as isize),
            health,
        }
    }
}


#[derive(Debug)]
pub struct HealthcheckState {
    pub indexed_block_num: i64,
    pub latest_block_num: i64,
    pub time: u64,
}

impl HealthcheckState {
    pub fn new() -> Self {
        Self {
            indexed_block_num: ApiFacade::get_indexed_block_number().expect("Got current indexed block num"),
            latest_block_num: ApiFacade::get_latest_block_number().expect("Got current latest block num"),
            time: current_time_as_secs(),
        }
    }
    pub fn from(indexed_block_num: i64, latest_block_num: i64) -> Self {
        Self {
            indexed_block_num,
            latest_block_num,
            time: current_time_as_secs(),
        }
    }

    pub fn from_global(global_state: &State<GlobalHealthcheckState>) -> Self {
        Self {
            indexed_block_num: global_state.indexed_block_num.load(Ordering::Relaxed) as i64,
            latest_block_num: global_state.latest_block_num.load(Ordering::Relaxed) as i64,
            time: global_state.time.load(Ordering::Relaxed) as u64,
        }
    }
}