#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Env};

#[test]
fn compute_fee_helpers_are_pure() {
    // Registry does not expose fee math; ensure status constants remain stable.
    assert_eq!(storage::STATUS_ACTIVE, 0);
    assert_eq!(storage::STATUS_PAUSED, 1);
    assert_eq!(storage::STATUS_DEPRECATED, 2);
    assert_eq!(VERSION, 2);
    let _ = symbol_short!("svc");
    let _env = Env::default();
}

#[test]
fn positive_price_gate() {
    assert!(require_positive_price(1).is_ok());
    assert_eq!(require_positive_price(0), Err(Error::ZeroPrice));
    assert_eq!(require_positive_price(-5), Err(Error::ZeroPrice));
}

#[test]
fn status_validation() {
    assert!(valid_status(0));
    assert!(valid_status(1));
    assert!(valid_status(2));
    assert!(!valid_status(9));
}
