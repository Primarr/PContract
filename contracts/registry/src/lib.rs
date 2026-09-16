#![no_std]

mod errors;
mod events;
mod storage;

use errors::Error;
use storage::{DataKey, Service, STATUS_ACTIVE, STATUS_DEPRECATED, STATUS_PAUSED, VERSION};
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

pub use errors::Error as RegistryError;
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

fn require_positive_price(price: i128) -> Result<(), Error> {
    if price <= 0 {
        Err(Error::ZeroPrice)
    } else {
        Ok(())
    }
}

fn valid_status(status: u32) -> bool {
    status == STATUS_ACTIVE || status == STATUS_PAUSED || status == STATUS_DEPRECATED
}

#[contract]
pub struct Registry;

#[contractimpl]
impl Registry {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Version, &VERSION);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().extend_ttl(100_000, 100_000);
        Ok(())
    }

    pub fn version(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Version)
            .unwrap_or(0)
    }

    pub fn admin(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Admin)
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
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

    pub fn register(
        env: Env,
        service_id: Symbol,
        provider: Address,
        price_per_call: i128,
    ) -> Result<(), Error> {
        require_unpaused(&env)?;
        provider.require_auth();
        require_positive_price(price_per_call)?;
        let key = DataKey::Service(service_id.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::AlreadyExists);
        }
        let service = Service {
            provider: provider.clone(),
            price_per_call,
            status: STATUS_ACTIVE,
        };
        env.storage().persistent().set(&key, &service);
        events::registered(&env, &service_id, &provider, price_per_call);
        Ok(())
    }

    pub fn get_price(env: Env, service_id: Symbol) -> Result<i128, Error> {
        let key = DataKey::Service(service_id);
        let service: Service = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::NotFound)?;
        if service.status != STATUS_ACTIVE {
            return Err(Error::Paused);
        }
        Ok(service.price_per_call)
    }

    pub fn get_service(env: Env, service_id: Symbol) -> Result<Service, Error> {
        let key = DataKey::Service(service_id);
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(Error::NotFound)
    }

    pub fn update_price(
        env: Env,
        service_id: Symbol,
        new_price: i128,
    ) -> Result<(), Error> {
        require_unpaused(&env)?;
        require_positive_price(new_price)?;
        let key = DataKey::Service(service_id.clone());
        let mut service: Service = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::NotFound)?;
        service.provider.require_auth();
        service.price_per_call = new_price;
        env.storage().persistent().set(&key, &service);
        events::price_updated(&env, &service_id, new_price);
        Ok(())
    }

    pub fn set_status(env: Env, service_id: Symbol, new_status: u32) -> Result<(), Error> {
        require_unpaused(&env)?;
        if !valid_status(new_status) {
            return Err(Error::InvalidStatus);
        }
        let key = DataKey::Service(service_id.clone());
        let mut service: Service = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::NotFound)?;
        service.provider.require_auth();
        service.status = new_status;
        env.storage().persistent().set(&key, &service);
        events::status_updated(&env, &service_id, new_status);
        Ok(())
    }
}

#[cfg(test)]
mod test;
