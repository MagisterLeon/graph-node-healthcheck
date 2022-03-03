use std::error::Error;
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

use rocket::State;
use web3::types::U64;

use crate::{api, BlockApi, current_time_as_secs, HealthcheckState};
use crate::api::Api;
use async_trait::async_trait;


pub const HEALTHCHECK_INTERVAL: isize = 5;

pub fn graph_healthcheck(api: &dyn Api, healthcheck_state: State<HealthcheckState>) -> Result<(), String> {
    let last_checked_time = healthcheck_state.time.load(Ordering::Relaxed);
    let current_time = current_time_as_secs();

    let mut is_ok = healthcheck_state.is_ok.lock().unwrap();
    if current_time < (last_checked_time + HEALTHCHECK_INTERVAL) as u64 {
        return if *is_ok {
            Ok(())
        } else {
            Err("Blocks are not being indexed.".to_string())
        };
    }

    let not_indexed_blocks = healthcheck_state.not_indexed_blocks_count.load(Ordering::Relaxed);
    let current_not_indexed_blocks = get_not_indexed_block_count(api);

    healthcheck_state.not_indexed_blocks_count.store(current_not_indexed_blocks as isize, Ordering::Relaxed);
    healthcheck_state.time.store(current_time as isize, Ordering::Relaxed);

    if not_indexed_blocks > 0 && current_not_indexed_blocks >= not_indexed_blocks as i64 {
        *is_ok = false;
        return Err(format!("Blocks are not being indexed. Not indexed blocks count: {:?}", &current_not_indexed_blocks));
    }
    *is_ok = true;
    return Ok(());
}

pub fn get_not_indexed_block_count(api: &dyn Api) -> i64 {
    let indexed_block = get_indexed_block_number(api);
    let latest_block = get_latest_block_number(api);

    i64::try_from(latest_block - indexed_block).unwrap()
}

fn get_indexed_block_number(api: &dyn Api) -> i64 {
    let indexed_block = match api.get_indexed_block_num() {
        Ok(res) => res,
        Err(error) => panic!("Getting indexed block numer failed: {:?}", error)
    };
    println!("Indexed block: {:?}", &indexed_block);
    indexed_block
}

#[tokio::main]
async fn get_latest_block_number(api: &dyn Api) -> U64 {
    let latest_block = match api.get_latest_block_num().await {
        Ok(res) => res,
        Err(error) => panic!("Getting latest block numer failed: {:?}", error)
    };
    println!("Latest block: {:?}", &latest_block);
    latest_block
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicIsize;
    use super::*;

    #[test]
    fn healthcheck_ok_when_blocks_indexing_increased() {
        // given
        let api = TestApi::new(2, 2);
        let time = current_time_as_secs() - HEALTHCHECK_INTERVAL as u64;
        let healthcheck_state = HealthcheckState::new(0, time, Mutex::new(true));
        let rocket = rocket::ignite().manage(healthcheck_state);
        let state = State::from(&rocket).expect("managing `HealthcheckState`");

        // when
        let result = graph_healthcheck(&api, state);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn healthcheck_err_when_num_of_not_indexed_blocks_is_equal() {
        // given
        let api = TestApi::new(0, 2);
        let time = current_time_as_secs() - HEALTHCHECK_INTERVAL as u64;
        let healthcheck_state = HealthcheckState::new(2, time, Mutex::new(true));
        let rocket = rocket::ignite().manage(healthcheck_state);
        let state = State::from(&rocket).expect("managing `HealthcheckState`");

        // when
        let result = graph_healthcheck(&api, state);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn healthcheck_err_when_num_of_not_indexed_blocks_increased() {
        // given
        let api = TestApi::new(0, 3);
        let time = current_time_as_secs() - HEALTHCHECK_INTERVAL as u64;
        let healthcheck_state = HealthcheckState::new(2, time, Mutex::new(true));
        let rocket = rocket::ignite().manage(healthcheck_state);
        let state = State::from(&rocket).expect("managing `HealthcheckState`");

        // when
        let result = graph_healthcheck(&api, state);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn return_recent_ok_result_if_checked_before_healthcheck_interval() {
        // given
        let api = TestApi::new(0, 0);
        let time = current_time_as_secs();
        let healthcheck_state = HealthcheckState::new(0, time, Mutex::new(true));
        let rocket = rocket::ignite().manage(healthcheck_state);
        let state = State::from(&rocket).expect("managing `HealthcheckState`");

        // when
        let result = graph_healthcheck(&api, state);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn return_recent_err_result_if_checked_before_healthcheck_interval() {
        // given
        let api = TestApi::new(0, 0);
        let time = current_time_as_secs();
        let healthcheck_state = HealthcheckState::new(0, time, Mutex::new(false));
        let rocket = rocket::ignite().manage(healthcheck_state);
        let state = State::from(&rocket).expect("managing `HealthcheckState`");

        // when
        let result = graph_healthcheck(&api, state);

        // then
        assert!(result.is_err());
    }

    struct TestApi {
        indexed_block: i64,
        latest_block: i64,
    }

    impl TestApi {
        pub fn new(indexed_block: i64, latest_block: i64) -> Self {
            Self {
                indexed_block,
                latest_block,
            }
        }
    }

    #[async_trait]
    impl Api for TestApi {
        fn get_indexed_block_num(&self) -> Result<i64, Box<dyn Error>> {
            Ok(self.indexed_block)
        }

        async fn get_latest_block_num(&self) -> web3::Result<U64> {
            web3::Result::Ok(U64::from(self.latest_block))
        }
    }
}