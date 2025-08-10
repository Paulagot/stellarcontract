#![cfg(test)]

use soroban_sdk::Env;

#[test] 
fn test_env_works() {
    let env = Env::default();
    let ledger_seq = env.ledger().sequence();
    println!("✅ Ledger sequence: {}", ledger_seq);
    assert!(ledger_seq >= 0);
}
