#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct ProjectRegistry;

#[contractimpl]
impl ProjectRegistry {
    pub fn initialize(_e: Env, _admin: Address) {}
}
