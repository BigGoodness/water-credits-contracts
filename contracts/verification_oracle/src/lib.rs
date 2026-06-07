#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct VerificationOracle;

#[contractimpl]
impl VerificationOracle {
    pub fn initialize(_e: Env, _admin: Address) {}
}
