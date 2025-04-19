mod memory;

use std::cell::RefCell;

use did::State;
use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use memory::{MEMORY_MANAGER, STATE_MEMORY_ID};

thread_local! {
    /// Initialize the state randomness with the current time.
    static STATE: RefCell<StableCell<State, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(STATE_MEMORY_ID)), State::default()).unwrap());
}

#[update]
fn set_state(state: State) {
    STATE.with_borrow_mut(|s| {
        s.set(state).unwrap();
    });
}

#[query]
fn get_state() -> State {
    STATE.with_borrow(|s| s.get().clone())
}

#[pre_upgrade]
fn pre_upgrade() {}

#[post_upgrade]
fn post_upgrade() {}

ic_cdk::export_candid!();
