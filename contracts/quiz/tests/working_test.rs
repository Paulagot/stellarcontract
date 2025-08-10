#![cfg(test)]

use soroban_sdk::{Env, testutils::Address as _, Address};
use quiz::{QuizRoomContract, QuizRoomContractClient};

#[test]
fn test_contract_registration() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(QuizRoomContract, ());
    println!("✅ Contract registered: {:?}", contract_id);
    assert!(true);
}

#[test]
fn test_basic_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let id = env.register(QuizRoomContract, ());
    let client = QuizRoomContractClient::new(&env, &id);

    let admin    = Address::generate(&env);
    let platform = Address::generate(&env);
    let charity  = Address::generate(&env);

    // Non-try call: returns (), panics on Err
    client.initialize(&admin, &platform, &charity);

    // Verify state via getters (these will also panic on Err)
    assert_eq!(client.get_platform_wallet(), platform);
    assert_eq!(client.get_charity_wallet(),  charity);
}

#[test]
fn test_cannot_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let id = env.register(QuizRoomContract, ());
    let client = QuizRoomContractClient::new(&env, &id);

    let admin    = Address::generate(&env);
    let platform = Address::generate(&env);
    let charity  = Address::generate(&env);

    client.initialize(&admin, &platform, &charity);

    // try_ form returns Result<Inner, HostError>
    let res = client.try_initialize(&admin, &platform, &charity);
    assert!(res.is_err());
}

