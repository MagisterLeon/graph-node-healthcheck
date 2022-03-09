use std::sync::atomic::Ordering;
use rocket::State;

use crate::{GlobalHealthcheckState, Health};
use crate::state::HealthcheckState;

const HEALTHCHECK_INTERVAL: u64 = 5;

// After this number of not indexed blocks we assume that the graph is down
const HEALTHCHECK_BUFFER: i64 = 10;

pub fn graph_healthcheck(global_state: State<GlobalHealthcheckState>) -> Health {
    let current_state = HealthcheckState::new();
    let previous_state = HealthcheckState::from_global(&global_state);
    println!("Current state: {:?}", &current_state);
    println!("Current previous_state: {:?}", &previous_state);

    let mut global_state_health = global_state.health.lock().unwrap();

    if !healthcheck_interval_passed(current_state.time, previous_state.time) {
        println!("Healthcheck interval hasn't passed yet, returning former value: {:?}", &global_state_health);
        return *global_state_health;
    }

    let current_health = validate_state(&current_state, &previous_state);
    update_global_state(&current_state, &global_state);
    println!("previous health was: {:?}, current health is {:?}", &global_state_health, &current_health);

    *global_state_health = current_health;

    return current_health;
}

fn healthcheck_interval_passed(current_time: u64, previous_time: u64) -> bool {
    return current_time >= previous_time + HEALTHCHECK_INTERVAL
}

fn validate_state(current_state: &HealthcheckState, previous_state: &HealthcheckState) -> Health {
    return if current_state.indexed_block_num == current_state.latest_block_num {
        Health::UP
    } else if current_state.indexed_block_num > previous_state.indexed_block_num {
        Health::UP
    } else if current_state.latest_block_num - current_state.indexed_block_num < HEALTHCHECK_BUFFER {
        Health::UP
    } else {
        Health::DOWN
    };
}

fn update_global_state(current_state: &HealthcheckState, global_state: &State<GlobalHealthcheckState>) {
    global_state.indexed_block_num.store(current_state.indexed_block_num as isize, Ordering::Relaxed);
    global_state.latest_block_num.store(current_state.latest_block_num as isize, Ordering::Relaxed);
    global_state.time.store(current_state.time as isize, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use crate::Health;
    use crate::{current_time_as_secs};
    use super::*;

    #[test]
    fn return_true_if_checked_after_healthcheck_interval() {
        // given
        let current_time = current_time_as_secs();
        let previous_time = current_time_as_secs() - HEALTHCHECK_INTERVAL;

        // when
        let result = healthcheck_interval_passed(current_time, previous_time);

        // then
        assert!(result);
    }

    #[test]
    fn return_false_if_checked_before_healthcheck_interval() {
        // given
        let current_time = current_time_as_secs();
        let previous_time = current_time_as_secs();

        // when
        let result = healthcheck_interval_passed(current_time, previous_time);

        // then
        assert_eq!(result, false);
    }

    #[test]
    fn is_healthy_when_all_blocks_are_indexed() {
        // given
        let current_state = HealthcheckState::from(2, 2);
        let previous_state = HealthcheckState::from(0, 0);

        // when
        let result = validate_state(&current_state, &previous_state);

        // then
        assert!(matches!(result, Health::UP));
    }

    #[test]
    fn is_healthy_when_num_of_indexed_blocks_increased() {
        // given
        let current_state = HealthcheckState::from(2, 3);
        let previous_state = HealthcheckState::from(0, 0);

        // when
        let result = validate_state(&current_state, &previous_state);

        // then
        assert!(matches!(result, Health::UP));
    }

    #[test]
    fn is_healthy_when_num_of_indexed_blocks_did_not_increase_but_is_smaller_than_healthcheck_buffer() {
        // given
        let current_state = HealthcheckState::from(2, 10);
        let previous_state = HealthcheckState::from(2, 2);

        // when
        let result = validate_state(&current_state, &previous_state);

        // then
        assert!(matches!(result, Health::UP));
    }

    #[test]
    fn is_not_healthy_when_num_of_not_indexed_blocks_passed_healthcheck_buffer() {
        // given
        let current_state = HealthcheckState::from(2, 13);
        let previous_state = HealthcheckState::from(2, 2);

        // when
        let result = validate_state(&current_state, &previous_state);

        // then
        assert!(matches!(result, Health::DOWN));
    }
}