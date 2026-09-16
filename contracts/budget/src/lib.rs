#![no_std]

mod errors;
mod storage;

use errors::Error;
use storage::{DataKey, Limit, PAYMENT_SESSION, PAYMENT_TASK, VERSION};
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

pub use errors::Error as BudgetError;
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
pub struct Budget;

#[contractimpl]
impl Budget {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Version, &VERSION);
        env.storage().instance().set(&DataKey::Paused, &false);
        Ok(())
    }

    pub fn version(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Version)
            .unwrap_or(0)
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

    pub fn set_limit(
        env: Env,
        agent: Symbol,
        owner: Address,
        session_cap: i128,
        task_cap: i128,
    ) -> Result<(), Error> {
        require_unpaused(&env)?;
        owner.require_auth();
        if session_cap < 0 || task_cap < 0 {
            return Err(Error::InvalidCap);
        }
        let limit = Limit {
            owner,
            session_cap,
            task_cap,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Limit(agent), &limit);
        Ok(())
    }

    pub fn check_limit(
        env: Env,
        agent: Symbol,
        amount: i128,
        payment_type: u32,
    ) -> Result<bool, Error> {
        if amount < 0 {
            return Err(Error::InvalidCap);
        }
        let key = DataKey::Limit(agent);
        let Some(limit) = env.storage().persistent().get::<_, Limit>(&key) else {
            return Ok(true);
        };
        let ok = match payment_type {
            PAYMENT_SESSION => amount <= limit.session_cap,
            PAYMENT_TASK => amount <= limit.task_cap,
            _ => return Err(Error::InvalidPaymentType),
        };
        Ok(ok)
    }

    pub fn get_limit(env: Env, agent: Symbol) -> Result<Limit, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Limit(agent))
            .ok_or(Error::NotFound)
    }

    pub fn assert_within_budget(
        env: Env,
        agent: Symbol,
        amount: i128,
        payment_type: u32,
    ) -> Result<(), Error> {
        if !Self::check_limit(env, agent, amount, payment_type)? {
            return Err(Error::OverBudget);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Symbol};

    #[test]
    fn fee_free_check_without_limit_allows() {
        let env = Env::default();
        let id = env.register(Budget, ());
        let client = BudgetClient::new(&env, &id);
        let agent = Symbol::new(&env, "agent_a");
        assert!(client.check_limit(&agent, &999i128, &0u32));
    }

    #[test]
    fn compute_paths_allow_task_without_limit() {
        let env = Env::default();
        let id = env.register(Budget, ());
        let client = BudgetClient::new(&env, &id);
        let agent = Symbol::new(&env, "agent_b");
        assert!(client.check_limit(&agent, &1i128, &1u32));
    }
}
