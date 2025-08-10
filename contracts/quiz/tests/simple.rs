#![cfg(test)]
use soroban_sdk::Env;

#[test] 
fn basic_test() {
    let env = Env::default();
    assert!(env.ledger().sequence() >= 0);
    println!("✅ Basic test passed!");
}
