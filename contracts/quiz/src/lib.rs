#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]

mod contract;

// Export the contract and all its types for tests
pub use contract::*;
