#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct RetirementRegistry;

#[contractimpl]
impl RetirementRegistry {
    pub fn initialize(_e: Env, _admin: Address) {}
}
