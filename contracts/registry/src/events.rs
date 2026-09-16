use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub fn registered(env: &Env, service_id: &Symbol, provider: &Address, price: i128) {
    env.events()
        .publish((symbol_short!("reg_svc"), service_id.clone()), (provider.clone(), price));
}

pub fn price_updated(env: &Env, service_id: &Symbol, price: i128) {
    env.events()
        .publish((symbol_short!("upd_prc"), service_id.clone()), price);
}

pub fn status_updated(env: &Env, service_id: &Symbol, status: u32) {
    env.events()
        .publish((symbol_short!("upd_st"), service_id.clone()), status);
}
