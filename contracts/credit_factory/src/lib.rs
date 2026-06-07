#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

#[contract]
pub struct CreditFactory;

#[contractimpl]
impl CreditFactory {
    pub fn initialize(_e: Env, _admin: Address, _token_wasm_hash: BytesN<32>) {}
}
