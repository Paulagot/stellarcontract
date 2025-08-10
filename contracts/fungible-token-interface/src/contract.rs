//! Fungible Pausable Example Contract with Public Rate-Limited Minting.
//!
//! This contract replicates the functionality of the contract in
//! "examples/fungible-pausable", offering the same features. The key difference
//! lies in how SEP-41 compliance is achieved. The contract in "contract.rs"
//! accomplishes this by implementing
//! [`stellar_tokens::fungible::FungibleToken`] and
//! [`stellar_tokens::fungible_burnable::FungibleBurnable`], whereas this
//! version directly implements [`soroban_sdk::token::TokenInterface`].
//!
//! This version has been modified to allow public minting with rate limiting
//! to prevent abuse while maintaining token supply control.

use soroban_sdk::{
    contract, contracterror, contractimpl, panic_with_error, symbol_short, token::TokenInterface,
    Address, Env, String, Symbol,
};
use stellar_contract_utils::pausable::{self as pausable, Pausable};
use stellar_macros::when_not_paused;
use stellar_tokens::fungible::Base;

pub const OWNER: Symbol = symbol_short!("OWNER");
pub const LAST_MINT: Symbol = symbol_short!("LAST_MINT");
pub const MINT_COOLDOWN: u64 = 86400; // 24 hours in seconds
pub const MAX_MINT_AMOUNT: i128 = 1000_0000000000000000; // 1000 tokens (18 decimals)

#[contract]
pub struct ExampleContract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ExampleContractError {
    Unauthorized = 1,
    ExceedsMaxMint = 2,
    MintCooldownActive = 3,
}

#[contractimpl]
impl ExampleContract {
    pub fn __constructor(e: &Env, owner: Address, initial_supply: i128) {
        Base::set_metadata(e, 18, String::from_str(e, "My Token"), String::from_str(e, "TKN"));
        Base::mint(e, &owner, initial_supply);
        e.storage().instance().set(&OWNER, &owner);
    }

    /// `TokenInterface` doesn't require implementing `total_supply()` because
    /// of the need for backwards compatibility with Stellar classic assets.
    pub fn total_supply(e: &Env) -> i128 {
        Base::total_supply(e)
    }

    /// Public mint function with rate limiting.
    /// Anyone can mint tokens to their own address with daily limits.
    #[when_not_paused]
    pub fn mint(e: &Env, account: Address, amount: i128) {
        // Users can only mint to themselves
        account.require_auth();
        
        // Enforce maximum mint amount per transaction
        if amount > MAX_MINT_AMOUNT {
            panic_with_error!(e, ExampleContractError::ExceedsMaxMint);
        }
        
        // Check cooldown period
        let current_time = e.ledger().timestamp();
        let last_mint_key = (LAST_MINT, account.clone());
        
        if let Some(last_mint_time) = e.storage().persistent().get::<(Symbol, Address), u64>(&last_mint_key) {
            if current_time - last_mint_time < MINT_COOLDOWN {
                panic_with_error!(e, ExampleContractError::MintCooldownActive);
            }
        }
        
        // Update last mint time
        e.storage().persistent().set(&last_mint_key, &current_time);
        
        Base::mint(e, &account, amount);
    }

    /// Owner-only mint function for administrative purposes.
    /// Allows the owner to mint without restrictions.
    #[when_not_paused]
    pub fn admin_mint(e: &Env, account: Address, amount: i128) {
        // When `ownable` module is available,
        // the following checks should be equivalent to:
        // `ownable::only_owner(&e);`
        let owner: Address = e.storage().instance().get(&OWNER).expect("owner should be set");
        owner.require_auth();

        Base::mint(e, &account, amount);
    }

    /// Get the remaining cooldown time for an account.
    /// Returns 0 if the account can mint immediately.
    pub fn get_mint_cooldown(e: &Env, account: Address) -> u64 {
        let current_time = e.ledger().timestamp();
        let last_mint_key = (LAST_MINT, account);
        
        if let Some(last_mint_time) = e.storage().persistent().get::<(Symbol, Address), u64>(&last_mint_key) {
            let time_elapsed = current_time - last_mint_time;
            if time_elapsed < MINT_COOLDOWN {
                return MINT_COOLDOWN - time_elapsed;
            }
        }
        
        0
    }

    /// Get the maximum amount that can be minted per transaction.
    pub fn get_max_mint_amount(e: &Env) -> i128 {
        MAX_MINT_AMOUNT
    }

    /// Get the cooldown period in seconds.
    pub fn get_mint_cooldown_period(e: &Env) -> u64 {
        MINT_COOLDOWN
    }
}

#[contractimpl]
impl Pausable for ExampleContract {
    fn paused(e: &Env) -> bool {
        pausable::paused(e)
    }

    fn pause(e: &Env, caller: Address) {
        // When `ownable` module is available,
        // the following checks should be equivalent to:
        // `ownable::only_owner(&e);`
        caller.require_auth();
        let owner: Address = e.storage().instance().get(&OWNER).expect("owner should be set");
        if owner != caller {
            panic_with_error!(e, ExampleContractError::Unauthorized);
        }

        pausable::pause(e);
    }

    fn unpause(e: &Env, caller: Address) {
        // When `ownable` module is available,
        // the following checks should be equivalent to:
        // `ownable::only_owner(&e);`
        caller.require_auth();
        let owner: Address = e.storage().instance().get(&OWNER).expect("owner should be set");
        if owner != caller {
            panic_with_error!(e, ExampleContractError::Unauthorized);
        }

        pausable::unpause(e);
    }
}

#[contractimpl]
impl TokenInterface for ExampleContract {
    fn balance(e: Env, account: Address) -> i128 {
        Base::balance(&e, &account)
    }

    fn allowance(e: Env, owner: Address, spender: Address) -> i128 {
        Base::allowance(&e, &owner, &spender)
    }

    #[when_not_paused]
    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        Base::transfer(&e, &from, &to, amount);
    }

    #[when_not_paused]
    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        Base::transfer_from(&e, &spender, &from, &to, amount);
    }

    fn approve(e: Env, owner: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        Base::approve(&e, &owner, &spender, amount, live_until_ledger);
    }

    #[when_not_paused]
    fn burn(e: Env, from: Address, amount: i128) {
        Base::burn(&e, &from, amount)
    }

    #[when_not_paused]
    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        Base::burn_from(&e, &spender, &from, amount)
    }

    fn decimals(e: Env) -> u32 {
        Base::decimals(&e)
    }

    fn name(e: Env) -> String {
        Base::name(&e)
    }

    fn symbol(e: Env) -> String {
        Base::symbol(&e)
    }
}
