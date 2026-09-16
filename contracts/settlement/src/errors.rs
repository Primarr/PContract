use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotFound = 2,
    AlreadyExists = 3,
    Unauthorized = 4,
    Paused = 5,
    ZeroAmount = 6,
    InvalidFee = 7,
}
