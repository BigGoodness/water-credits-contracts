#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct Governance;

#[contractimpl]
impl Governance {
    pub fn initialize(_e: Env, _admin: Address) {}
}
