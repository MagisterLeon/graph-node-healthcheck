use std::error::Error;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

use rocket::State;

use crate::{current_time_as_secs, HealthcheckState};
use async_trait::async_trait;
use crate::errors::HealthcheckError;


pub const HEALTHCHECK_INTERVAL: isize = 5;

pub struct CurrentHealthcheckState {
    pub indexed_block_num: i64,
    pub latest_block_num: i64,
    pub time: u64
}


pub fn graph_healthcheck(previous_state: State<HealthcheckState>) -> Result<(), HealthcheckError> {
    // let not_indexed_blocks = previous_state.indexed_block_num.load(Ordering::Relaxed);
    // let not_indexed_blocks = previous_state.indexed_block_num.load(Ordering::Relaxed);
    //
    // println!("current state: not_indexed_blocks_count {:?}", &previous_state.indexed_block_number);
    // println!("current state: time {:?}", &previous_state.time);
    // println!("current state: is_ok {:?}", &is_ok);

    let current_time = current_time_as_secs();
    let mut is_ok = previous_state.is_ok.lock().unwrap();

    let previous_time = previous_state.time.load(Ordering::Relaxed);
    if current_time < (previous_time + HEALTHCHECK_INTERVAL) as u64 {
        return if *is_ok {
            Ok(())
        } else {
            Err(HealthcheckError::new("Blocks are not being indexed."))
        };
    }
    return Ok(())
    //
    // return match get_not_indexed_block_count(api) {
    //     Ok(current_not_indexed_blocks) => {
    //         previous_state.indexed_block_number.store(current_not_indexed_blocks as isize, Ordering::Relaxed);
    //         previous_state.time.store(current_time as isize, Ordering::Relaxed);
    //
    //         if not_indexed_blocks > 0 && current_not_indexed_blocks >= not_indexed_blocks as i64 {
    //             *is_ok = false;
    //             return Err(HealthcheckError::new(
    //                 format!("Blocks are not being indexed. Not indexed blocks count: {:?}", &current_not_indexed_blocks).as_str())
    //             );
    //         }
    //         *is_ok = true;
    //         return Ok(())
    //     },
    //     Err(err) => {
    //         std::mem::drop(is_ok);
    //         eprintln!("Problem fetching not indexed blocks: {}", err);
    //         Err(HealthcheckError::new("Cannot get not indexed blocks count"))
    //     }
    // }
}

//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_recent_ok_result_if_checked_before_healthcheck_interval() {
        // given
        let time = current_time_as_secs();
        let healthcheck_state = HealthcheckState::new(0, 0, time, Mutex::new(true));
        let rocket = rocket::ignite().manage(healthcheck_state);
        let state = State::from(&rocket).expect("managing `HealthcheckState`");

        // when
        let result = graph_healthcheck(state);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn return_recent_err_result_if_checked_before_healthcheck_interval() {
        // given
        let time = current_time_as_secs();
        let healthcheck_state = HealthcheckState::new(0,0, time, Mutex::new(false));
        let rocket = rocket::ignite().manage(healthcheck_state);
        let state = State::from(&rocket).expect("managing `HealthcheckState`");

        // when
        let result = graph_healthcheck(state);

        // then
        assert!(result.is_err());
    }


//
//     #[test]
//     fn healthcheck_ok_when_blocks_indexing_increased() {
//         // given
//         let api = TestApi::new(2, 2);
//         let time = current_time_as_secs() - HEALTHCHECK_INTERVAL as u64;
//         let healthcheck_state = HealthcheckState::new(0, time, Mutex::new(true));
//         let rocket = rocket::ignite().manage(healthcheck_state);
//         let state = State::from(&rocket).expect("managing `HealthcheckState`");
//
//         // when
//         let result = graph_healthcheck(&api, state);
//
//         // then
//         assert!(result.is_ok());
//     }
//
//     #[test]
//     fn healthcheck_err_when_num_of_not_indexed_blocks_is_equal() {
//         // given
//         let api = TestApi::new(0, 2);
//         let time = current_time_as_secs() - HEALTHCHECK_INTERVAL as u64;
//         let healthcheck_state = HealthcheckState::new(2, time, Mutex::new(true));
//         let rocket = rocket::ignite().manage(healthcheck_state);
//         let state = State::from(&rocket).expect("managing `HealthcheckState`");
//
//         // when
//         let result = graph_healthcheck(&api, state);
//
//         // then
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn healthcheck_err_when_num_of_not_indexed_blocks_increased() {
//         // given
//         let api = TestApi::new(0, 3);
//         let time = current_time_as_secs() - HEALTHCHECK_INTERVAL as u64;
//         let healthcheck_state = HealthcheckState::new(2, time, Mutex::new(true));
//         let rocket = rocket::ignite().manage(healthcheck_state);
//         let state = State::from(&rocket).expect("managing `HealthcheckState`");
//
//         // when
//         let result = graph_healthcheck(&api, state);
//
//         // then
//         assert!(result.is_err());
//     }
//
}