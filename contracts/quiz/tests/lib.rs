#![cfg(test)]
extern crate std;

use soroban_sdk::{
    testutils::{Address as _},
    Address, Env, String, Vec,
    token::{StellarAssetClient, TokenClient},
};
use quiz::{
    QuizRoomContract, QuizRoomContractClient,
    PrizeAsset,
};

// Test helper functions
fn create_quiz_contract(e: &Env) -> (QuizRoomContractClient, Address) {
    let contract_id = e.register(QuizRoomContract, ());
    let client = QuizRoomContractClient::new(e, &contract_id);
    (client, contract_id)
}

fn create_token_contract(e: &Env, admin: &Address) -> Address {
    let token_contract = e.register_stellar_asset_contract_v2(admin.clone());
    token_contract.address()
}

fn initialize_contract_with_tokens(
    e: &Env
) -> (QuizRoomContractClient, Address, Address, Address, Vec<Address>) {
    let admin = Address::generate(e);
    let platform_wallet = Address::generate(e);
    let charity_wallet = Address::generate(e);
    
    let (contract, contract_address) = create_quiz_contract(e);
    
    // Initialize contract
    contract.initialize(&admin, &platform_wallet, &charity_wallet);
    
    // Create test tokens
    let token1_address = create_token_contract(e, &admin);
    let token2_address = create_token_contract(e, &admin);
    let token3_address = create_token_contract(e, &admin);
    
    // Add tokens to approved list
    contract.add_approved_token(
        &token1_address,
        &String::from_str(e, "USDC"),
        &String::from_str(e, "USD Coin")
    );
    
    contract.add_approved_token(
        &token2_address,
        &String::from_str(e, "XLM"),
        &String::from_str(e, "Stellar Lumens")
    );
    
    contract.add_approved_token(
        &token3_address,
        &String::from_str(e, "EURC"),
        &String::from_str(e, "Euro Coin")
    );
    
    let tokens = Vec::from_array(e, [token1_address, token2_address, token3_address]);
    
    (contract, contract_address, admin, platform_wallet, tokens)
}

fn mint_tokens_for_users(
    e: &Env,
    token_address: &Address,
    users: &[Address],
    amount: i128
) {
    let stellar_client = StellarAssetClient::new(e, token_address);
    for user in users {
        stellar_client.mint(user, &amount);
    }
}

// Basic initialization tests
#[test]
fn test_contract_initialization() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let platform_wallet = Address::generate(&e);
    let charity_wallet = Address::generate(&e);
    
    e.mock_all_auths();
    let (contract, _) = create_quiz_contract(&e);
    
    // Test successful initialization
    contract.initialize(&admin, &platform_wallet, &charity_wallet);
    
    // Verify admin config
    let retrieved_platform = contract.get_platform_wallet();
    let retrieved_charity = contract.get_charity_wallet();
    
    assert_eq!(retrieved_platform, platform_wallet);
    assert_eq!(retrieved_charity, charity_wallet);
    
    // Test that double initialization fails
    let result = contract.try_initialize(&admin, &platform_wallet, &charity_wallet);
    assert!(result.is_err());
}

#[test]
fn test_token_management() {
    let e = Env::default();
    e.mock_all_auths();
    let (contract, _, _admin, _, _) = initialize_contract_with_tokens(&e);
    
    // Test getting approved tokens
    let approved_tokens = contract.get_approved_tokens_list();
    assert_eq!(approved_tokens.len(), 3);
    
    // Test token approval check
    let token_address = approved_tokens.get(0).unwrap().contract_id.clone();
    assert!(contract.is_token_approved(&token_address));
    
    // Test adding duplicate token fails
    let result = contract.try_add_approved_token(
        &token_address,
        &String::from_str(&e, "USDC"),
        &String::from_str(&e, "USD Coin")
    );
    assert!(result.is_err());
    
    // Test removing token
    contract.remove_approved_token(&token_address);
    assert!(!contract.is_token_approved(&token_address));
    
    let updated_tokens = contract.get_approved_tokens_list();
    assert_eq!(updated_tokens.len(), 2);
}

#[test]
fn test_emergency_controls() {
    let e = Env::default();
    e.mock_all_auths();
    let (contract, _, _admin, _, _) = initialize_contract_with_tokens(&e);
    
    // Test emergency pause
    assert!(!contract.is_emergency_paused());
    
    contract.emergency_pause();
    assert!(contract.is_emergency_paused());
    
    // Test that operations fail when paused
    let host = Address::generate(&e);
    let approved_tokens = contract.get_approved_tokens_list();
    let token_address = approved_tokens.get(0).unwrap().contract_id.clone();
    
    let result = contract.try_init_pool_room(
        &1,
        &host,
        &token_address,
        &1000000,
        &Some(250),
        &2000,
        &60,
        &Some(30),
        &Some(10)
    );
    assert!(result.is_err());
    
    // Test unpause
    contract.emergency_unpause();
    assert!(!contract.is_emergency_paused());
}

#[test]
fn test_pool_room_creation() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    
    // Test successful pool room creation
    contract.init_pool_room(
        &1,
        &host,
        &token_address,
        &1000000, // 0.1 tokens entry fee
        &Some(250), // 2.5% host fee
        &2000, // 20% prize pool
        &60, // 60% first place
        &Some(30), // 30% second place
        &Some(10) // 10% third place
    );
    
    // Verify room was created
    let room_config = contract.get_room_config(&1).unwrap();
    assert_eq!(room_config.host(), &host);
    assert_eq!(room_config.entry_fee(), 1000000);
    assert_eq!(room_config.host_fee_bps(), 250);
    assert_eq!(room_config.prize_pool_bps(), 2000);
    assert!(!room_config.ended());
    assert_eq!(room_config.player_count(), 0);
}

#[test]
fn test_pool_room_creation_validation() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    
    // Test invalid host fee (too high)
    let result = contract.try_init_pool_room(
        &1,
        &host,
        &token_address,
        &1000000,
        &Some(600), // 6% (max is 5%)
        &2000,
        &60,
        &Some(30),
        &Some(10)
    );
    assert!(result.is_err());
    
    // Test invalid prize pool (too high)
    let result = contract.try_init_pool_room(
        &2,
        &host,
        &token_address,
        &1000000,
        &Some(250),
        &2600, // 26% (max is 25%)
        &60,
        &Some(30),
        &Some(10)
    );
    assert!(result.is_err());
    
    // Test invalid prize distribution (doesn't sum to 100)
    let result = contract.try_init_pool_room(
        &3,
        &host,
        &token_address,
        &1000000,
        &Some(250),
        &2000,
        &60,
        &Some(30),
        &Some(20) // 60 + 30 + 20 = 110%
    );
    assert!(result.is_err());
    
    // Test using non-approved token
    let invalid_token = Address::generate(&e);
    let result = contract.try_init_pool_room(
        &4,
        &host,
        &invalid_token,
        &1000000,
        &Some(250),
        &2000,
        &60,
        &Some(30),
        &Some(10)
    );
    assert!(result.is_err());
}

#[test]
fn test_asset_room_creation() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, contract_address, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    let prize_token_address = tokens.get(1).unwrap();
    
    // Mint prizes to host using stellar client
    mint_tokens_for_users(&e, &prize_token_address, &[host.clone()], 1000000000);
    
    // Create prize assets
    let mut prizes = Vec::new(&e);
    prizes.push_back(PrizeAsset {
        contract_id: prize_token_address.clone(),
        amount: 50000000, // 5 tokens
    });
    prizes.push_back(PrizeAsset {
        contract_id: prize_token_address.clone(),
        amount: 30000000, // 3 tokens
    });
    prizes.push_back(PrizeAsset {
        contract_id: prize_token_address.clone(),
        amount: 20000000, // 2 tokens
    });
    
    // Create asset room
    contract.init_asset_room(
        &1,
        &host,
        &token_address,
        &2000000, // 0.2 tokens entry fee
        &Some(300), // 3% host fee
        &prizes
    );
    
    // Verify room was created
    let room_config = contract.get_room_config(&1).unwrap();
    assert_eq!(room_config.host(), &host);
    assert_eq!(room_config.entry_fee(), 2000000);
    assert_eq!(room_config.host_fee_bps(), 300);
    assert!(!room_config.ended());
    
    // Verify prizes were escrowed
    let token_client = TokenClient::new(&e, &prize_token_address);
    let contract_balance = token_client.balance(&contract_address);
    assert_eq!(contract_balance, 100000000); // 10 tokens total
}

#[test]
fn test_player_joining() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let player1 = Address::generate(&e);
    let player2 = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    
    // Mint tokens to players
    mint_tokens_for_users(&e, &token_address, &[player1.clone(), player2.clone()], 10000000);
    
    // Create room
    contract.init_pool_room(
        &1,
        &host,
        &token_address,
        &1000000,
        &Some(250),
        &2000,
        &60,
        &Some(30),
        &Some(10)
    );
    
    // Player 1 joins with extras
    contract.join_room(
        &1,
        &player1,
        &String::from_str(&e, "Player1"),
        &500000 // 0.05 tokens extras
    );
    
    // Player 2 joins without extras
    contract.join_room(
        &1,
        &player2,
        &String::from_str(&e, "Player2"),
        &0
    );
    
    // Verify players joined
    let room_config = contract.get_room_config(&1).unwrap();
    assert_eq!(room_config.player_count(), 2);
    assert_eq!(room_config.total_pool(), 2500000); // 2 * 1M + 0.5M extras
    
    let players = contract.get_room_players(&1);
    assert_eq!(players.len(), 2);
    
    // Verify player lookup by screen name
    let player1_addr = contract.get_player_by_screen_name(&1, &String::from_str(&e, "Player1"));
    assert_eq!(player1_addr, Some(player1));
}

#[test]
fn test_player_joining_validation() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let player1 = Address::generate(&e);
    let player2 = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    
    // Mint tokens to players
    mint_tokens_for_users(&e, &token_address, &[player1.clone(), player2.clone()], 10000000);
    
    // Create room
    contract.init_pool_room(&1, &host, &token_address, &1000000, &Some(250), &2000, &100, &None, &None);
    
    // Player 1 joins successfully
    contract.join_room(&1, &player1, &String::from_str(&e, "Player1"), &0);
    
    // Test duplicate player
    let result = contract.try_join_room(&1, &player1, &String::from_str(&e, "NewName"), &0);
    assert!(result.is_err());
    
    // Test duplicate screen name
    let result = contract.try_join_room(&1, &player2, &String::from_str(&e, "Player1"), &0);
    assert!(result.is_err());
    
    // Test invalid screen name (too long)
    let long_name = String::from_str(&e, "ThisNameIsTooLongForValidation");
    let result = contract.try_join_room(&1, &player2, &long_name, &0);
    assert!(result.is_err());
    
    // Test invalid screen name (empty)
    let empty_name = String::from_str(&e, "");
    let result = contract.try_join_room(&1, &player2, &empty_name, &0);
    assert!(result.is_err());
}

#[test]
fn test_room_completion_with_winners() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, contract_address, _, platform_wallet, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let player1 = Address::generate(&e);
    let player2 = Address::generate(&e);
    let player3 = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    
    // Mint tokens to players
    let players = [player1.clone(), player2.clone(), player3.clone()];
    mint_tokens_for_users(&e, &token_address, &players, 10000000);
    
    // Create room
    contract.init_pool_room(&1, &host, &token_address, &1000000, &Some(200), &2000, &50, &Some(30), &Some(20));
    
    // Players join
    contract.join_room(&1, &player1, &String::from_str(&e, "Winner"), &0);
    contract.join_room(&1, &player2, &String::from_str(&e, "Second"), &500000);
    contract.join_room(&1, &player3, &String::from_str(&e, "Third"), &250000);
    
    // Check initial balances
    let token_client = TokenClient::new(&e, &token_address);
    let initial_platform_balance = token_client.balance(&platform_wallet);
    
    // End room with winners
    contract.end_room(&1, &Some(player1.clone()), &Some(player2.clone()), &Some(player3.clone()));
    
    // Verify room ended
    let room_config = contract.get_room_config(&1).unwrap();
    assert!(room_config.ended());
    assert_eq!(room_config.winners().len(), 3);
    assert_eq!(room_config.winners().get(0).unwrap(), player1);
    
    // Verify prize distribution occurred
    let final_contract_balance = token_client.balance(&contract_address);
    let final_platform_balance = token_client.balance(&platform_wallet);
    
    // Contract should have distributed all funds
    assert_eq!(final_contract_balance, 0);
    // Platform should have received their 20% fee
    assert!(final_platform_balance > initial_platform_balance);
}

#[test]
fn test_room_completion_by_screen_names() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let player1 = Address::generate(&e);
    let player2 = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    
    // Mint tokens to players
    mint_tokens_for_users(&e, &token_address, &[player1.clone(), player2.clone()], 10000000);
    
    // Create room
    contract.init_pool_room(&1, &host, &token_address, &1000000, &None, &2000, &70, &Some(30), &None);
    
    // Players join
    contract.join_room(&1, &player1, &String::from_str(&e, "Champion"), &0);
    contract.join_room(&1, &player2, &String::from_str(&e, "Runner"), &0);
    
    // End room by screen names
    contract.end_room_by_screen_names(
        &1,
        &Some(String::from_str(&e, "Champion")),
        &Some(String::from_str(&e, "Runner")),
        &None
    );
    
    // Verify room ended correctly
    let room_config = contract.get_room_config(&1).unwrap();
    assert!(room_config.ended());
    assert_eq!(room_config.winners().get(0).unwrap(), player1);
    assert_eq!(room_config.winners().get(1).unwrap(), player2);
}

#[test]
fn test_room_completion_validation() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let player1 = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    
    // Create room
    contract.init_pool_room(&1, &host, &token_address, &1000000, &None, &2000, &100, &None, &None);
    
    // Test ending room with no players
    let result = contract.try_end_room(&1, &None, &None, &None);
    assert!(result.is_err());
    
    // Add player
    mint_tokens_for_users(&e, &token_address, &[player1.clone()], 10000000);
    contract.join_room(&1, &player1, &String::from_str(&e, "Player1"), &0);
    
    // Test ending with invalid winner (not a player)
    let fake_winner = Address::generate(&e);
    let result = contract.try_end_room(&1, &Some(fake_winner), &None, &None);
    assert!(result.is_err());
    
    // Test ending already ended room
    contract.end_room(&1, &Some(player1.clone()), &None, &None);
    let result = contract.try_end_room(&1, &Some(player1.clone()), &None, &None);
    assert!(result.is_err());
}

#[test]
fn test_financial_calculations() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    
    // Create room with specific fee structure
    contract.init_pool_room(
        &1,
        &host,
        &token_address,
        &10000000, // 1 token entry fee
        &Some(500), // 5% host fee
        &2500, // 25% prize pool
        &100, // 100% to winner
        &None,
        &None
    );
    
    // Add players with different extras
    let players = [
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    
    mint_tokens_for_users(&e, &token_address, &players, 50000000);
    
    contract.join_room(&1, &players[0], &String::from_str(&e, "P1"), &1000000); // 0.1 extra
    contract.join_room(&1, &players[1], &String::from_str(&e, "P2"), &2000000); // 0.2 extra
    contract.join_room(&1, &players[2], &String::from_str(&e, "P3"), &0); // no extra
    
    // Check financials
    let financials = contract.get_room_financials(&1).unwrap();
    let (total_pool, entry_fees, extras_fees, expected_payouts, remainder) = financials;
    
    // Expected: 3 * 10M + 3M extras = 33M total
    assert_eq!(total_pool, 33000000);
    assert_eq!(entry_fees, 30000000);
    assert_eq!(extras_fees, 3000000);
    
    // Expected distribution:
    // Platform: 20% of 33M = 6.6M
    // Host: 5% of 33M = 1.65M
    // Prize: 25% of 33M = 8.25M
    // Charity: 50% of 33M = 16.5M
    // Total: 33M
    
    let expected_total = 6600000 + 1650000 + 8250000 + 16500000;
    assert_eq!(expected_payouts, expected_total);
    assert_eq!(remainder, 0);
}

#[test]
fn test_edge_cases() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (contract, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let token_address = tokens.get(0).unwrap();
    
    // Use minimum valid entry fee (from economic config: 1000000)
    let min_entry_fee = 1000000;
    
    // Test room with minimum entry fee
    contract.init_pool_room(&1, &host, &token_address, &min_entry_fee, &None, &0, &100, &None, &None);
    
    // Test room with zero host fee and zero prize pool (100% charity)
    contract.init_pool_room(&2, &host, &token_address, &min_entry_fee, &None, &0, &100, &None, &None);
    
    // Test room with maximum allowed fees
    contract.init_pool_room(&3, &host, &token_address, &min_entry_fee, &Some(500), &2500, &100, &None, &None);
    
    // Test single player room
    let player = Address::generate(&e);
    mint_tokens_for_users(&e, &token_address, &[player.clone()], 10000000);
    
    contract.join_room(&1, &player, &String::from_str(&e, "Solo"), &0);
    contract.end_room(&1, &Some(player), &None, &None);
}

#[test]
fn test_extreme_edge_cases() {
    let e = Env::default();
    e.mock_all_auths();
    
    let admin = Address::generate(&e);
    let platform_wallet = Address::generate(&e);
    let charity_wallet = Address::generate(&e);
    
    e.mock_all_auths();
    let (contract, _) = create_quiz_contract(&e);
    
    // Initialize with custom config
    contract.initialize(&admin, &platform_wallet, &charity_wallet);
    
    // Create a token for testing
    let token_address = create_token_contract(&e, &admin);
    contract.add_approved_token(
        &token_address,
        &String::from_str(&e, "TEST"),
        &String::from_str(&e, "Test Token")
    );
    
    let host = Address::generate(&e);
    
    // Now we can test with very low entry fees if we modify the economic config
    // For now, let's test with the minimum allowed values
    let min_fee = 1000000; // 0.1 tokens (from economic config)
    
    // Test room with absolute minimum settings
    contract.init_pool_room(&1, &host, &token_address, &min_fee, &None, &0, &100, &None, &None);
    
    // Test single player scenario
    let player = Address::generate(&e);
    mint_tokens_for_users(&e, &token_address, &[player.clone()], 10000000);
    
    contract.join_room(&1, &player, &String::from_str(&e, "Solo"), &0);
    contract.end_room(&1, &Some(player), &None, &None);
}

#[test]
fn token_disable_blocks_new_rooms() {
    let e = Env::default(); e.mock_all_auths();
    let (c, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let token = tokens.get(0).unwrap();

    // disable the token
    c.enable_disable_token(&token, &false);
    assert_eq!(c.get_approved_tokens_list().len(), 2);
    assert!(!c.is_token_approved(&token));

    // cannot init a room with a disabled token
    let r = c.try_init_pool_room(&1, &host, &token, &1_000_000, &None, &2000, &100, &None, &None);
    assert!(r.is_err());
}

#[test]
fn join_insufficient_balance() {
    let e = Env::default(); e.mock_all_auths();
    let (c, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let player = Address::generate(&e);
    let token = tokens.get(0).unwrap();

    c.init_pool_room(&1, &host, &token, &1_000_000, &None, &2000, &100, &None, &None);
    // Mint less than entry fee
    mint_tokens_for_users(&e, &token, &[player.clone()], 900_000);
    let r = c.try_join_room(&1, &player, &String::from_str(&e, "P"), &0);
    assert!(r.is_err());
}

#[test]
fn rounding_remainder_goes_to_charity() {
    let e = Env::default(); e.mock_all_auths();
    let (c, contract_addr, _, charity, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let p = Address::generate(&e);
    let token = tokens.get(0).unwrap();
    let t = TokenClient::new(&e, &token);

    // Choose values likely to produce truncation dust
    c.init_pool_room(&1, &host, &token, &1_000_001, &Some(123), &2000, &100, &None, &None);
    mint_tokens_for_users(&e, &token, &[p.clone()], 1_000_001);
    c.join_room(&1, &p, &String::from_str(&e, "P"), &0);

    let charity_before = t.balance(&charity);
    c.end_room(&1, &Some(p), &None, &None);

    assert_eq!(t.balance(&contract_addr), 0); // contract drained
    let charity_after = t.balance(&charity);
    assert!(charity_after > charity_before);   // got charity fee (+ remainder)
}

#[test]
fn add_invalid_token_fails() {
    let e = Env::default(); e.mock_all_auths();
    let (c, _, _, _, _) = initialize_contract_with_tokens(&e);
    let bogus = Address::generate(&e); // not a token contract
    let r = c.try_add_approved_token(
        &bogus,
        &String::from_str(&e,"BOGUS"),
        &String::from_str(&e,"NotAToken"),
    );
    assert!(r.is_err());
}

#[test]
fn atomic_update_rolls_back_on_error() {
    let e = Env::default(); e.mock_all_auths();
    let (c, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let a = Address::generate(&e);
    let b = Address::generate(&e);
    let token = tokens.get(0).unwrap();

    mint_tokens_for_users(&e, &token, &[a.clone(), b.clone()], 2_000_000);
    c.init_pool_room(&1, &host, &token, &1_000_000, &None, &2000, &100, &None, &None);

    c.join_room(&1, &a, &String::from_str(&e,"Dup"), &0);
    let r = c.try_join_room(&1, &b, &String::from_str(&e,"Dup"), &0); // duplicate name
    assert!(r.is_err());

    let cfg = c.get_room_config(&1).unwrap();
    assert_eq!(cfg.player_count(), 1);
    assert_eq!(cfg.total_pool(), 1_000_000);
}
#[test]
fn paused_blocks_join_and_end() {
    let e = Env::default(); e.mock_all_auths();
    let (c, _, _, _, tokens) = initialize_contract_with_tokens(&e);
    let host = Address::generate(&e);
    let p = Address::generate(&e);
    let t = tokens.get(0).unwrap();

    mint_tokens_for_users(&e, &t, &[p.clone()], 2_000_000);
    c.init_pool_room(&1, &host, &t, &1_000_000, &None, &2000, &100, &None, &None);

    c.emergency_pause();

    let r1 = c.try_join_room(&1, &p, &String::from_str(&e,"P"), &0);
    assert!(r1.is_err());

    let r2 = c.try_end_room(&1, &Some(p.clone()), &None, &None);
    assert!(r2.is_err());

    c.emergency_unpause();
    c.join_room(&1, &p, &String::from_str(&e,"P"), &0); // now succeeds
}

