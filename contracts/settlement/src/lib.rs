#![no_std]

mod errors;
mod storage;

use errors::Error;
use storage::{DataKey, SettlementRecord, VERSION};
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

pub use errors::Error as SettlementError;
pub use storage::VERSION as CONTRACT_VERSION;

fn require_unpaused(env: &Env) -> Result<(), Error> {
    let paused: bool = env
        .storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false);
    if paused {
        Err(Error::Paused)
    } else {
        Ok(())
    }
}

fn require_admin(env: &Env) -> Result<Address, Error> {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotFound)?;
    admin.require_auth();
    Ok(admin)
}

#[contract]
pub struct Settlement;

#[contractimpl]
impl Settlement {
    pub fn initialize(env: Env, admin: Address, fee_bps: i128) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if fee_bps < 0 || fee_bps > 10_000 {
            return Err(Error::InvalidFee);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Version, &VERSION);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().set(&DataKey::FeeBps, &fee_bps);
        Ok(())
    }

    pub fn version(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Version)
            .unwrap_or(0)
    }

    pub fn fee_bps(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::FeeBps)
            .unwrap_or(0)
    }

    pub fn set_fee_bps(env: Env, fee_bps: i128) -> Result<(), Error> {
        require_admin(&env)?;
        if fee_bps < 0 || fee_bps > 10_000 {
            return Err(Error::InvalidFee);
        }
        env.storage().instance().set(&DataKey::FeeBps, &fee_bps);
        Ok(())
    }

    pub fn pause(env: Env) -> Result<(), Error> {
        require_admin(&env)?;
        env.storage().instance().set(&DataKey::Paused, &true);
        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), Error> {
        require_admin(&env)?;
        env.storage().instance().set(&DataKey::Paused, &false);
        Ok(())
    }

    pub fn compute_fee(amount: i128, fee_bps: i128) -> i128 {
        (amount * fee_bps) / 10_000i128
    }

    pub fn record_settlement(
        env: Env,
        tx_id: Symbol,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<SettlementRecord, Error> {
        require_unpaused(&env)?;
        from.require_auth();
        if amount <= 0 {
            return Err(Error::ZeroAmount);
        }
        let key = DataKey::Tx(tx_id);
        if env.storage().persistent().has(&key) {
            return Err(Error::AlreadyExists);
        }
        let bps = Self::fee_bps(env.clone());
        let fee = Self::compute_fee(amount, bps);
        let record = SettlementRecord {
            from,
            to,
            amount,
            fee,
            timestamp: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&key, &record);
        Ok(record)
    }

    pub fn get_transaction(env: Env, tx_id: Symbol) -> Result<SettlementRecord, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Tx(tx_id))
            .ok_or(Error::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_fee_twenty_bps() {
        assert_eq!(Settlement::compute_fee(1000i128, 20i128), 2i128);
    }

    #[test]
    fn compute_fee_zero_bps() {
        assert_eq!(Settlement::compute_fee(5_000i128, 0i128), 0i128);
    }

    #[test]
    fn compute_fee_hundred_bps() {
        assert_eq!(Settlement::compute_fee(10_000i128, 100i128), 100i128);
    }
}
