use soroban_sdk::{contracttype, Address, Symbol};

pub const VERSION: u32 = 2;
pub const STATUS_ACTIVE: u32 = 0;
pub const STATUS_PAUSED: u32 = 1;
pub const STATUS_DEPRECATED: u32 = 2;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Version,
    Paused,
    Service(Symbol),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Service {
    pub provider: Address,
    pub price_per_call: i128,
    pub status: u32,
}
