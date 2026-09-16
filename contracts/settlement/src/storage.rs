use soroban_sdk::{contracttype, Address, Symbol};

pub const VERSION: u32 = 2;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Version,
    Paused,
    FeeBps,
    Tx(Symbol),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct SettlementRecord {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub fee: i128,
    pub timestamp: u64,
}
