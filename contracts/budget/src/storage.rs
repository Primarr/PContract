use soroban_sdk::{contracttype, Address, Symbol};

pub const VERSION: u32 = 2;
pub const PAYMENT_SESSION: u32 = 0;
pub const PAYMENT_TASK: u32 = 1;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Version,
    Paused,
    Limit(Symbol),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Limit {
    pub owner: Address,
    pub session_cap: i128,
    pub task_cap: i128,
}
