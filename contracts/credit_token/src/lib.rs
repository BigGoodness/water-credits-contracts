#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[cfg(test)]
extern crate std;

#[contracttype]
pub enum DataKey {
    Balance(Address),
    Allowance(Address, Address),
    Admin,
    TotalSupply,
    Name,
    Symbol,
    Decimals,
}

fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Admin)
}

fn save_balance(e: &Env, addr: &Address, amount: i128) {
    e.storage().instance().set(&DataKey::Balance(addr.clone()), &amount);
}

fn read_balance(e: &Env, addr: &Address) -> i128 {
    e.storage().instance().get(&DataKey::Balance(addr.clone())).unwrap_or(0)
}

fn save_allowance(e: &Env, from: &Address, spender: &Address, amount: i128) {
    e.storage().instance().set(&DataKey::Allowance(from.clone(), spender.clone()), &amount);
}

fn read_allowance(e: &Env, from: &Address, spender: &Address) -> i128 {
    e.storage().instance().get(&DataKey::Allowance(from.clone(), spender.clone())).unwrap_or(0)
}

#[contract]
pub struct CreditToken;

#[contractimpl]
impl CreditToken {
    pub fn initialize(e: Env, admin: Address, name: String, symbol: String) {
        if has_admin(&e) {
            panic!("already initialized");
        }
        e.storage().instance().set(&DataKey::Admin, &admin);
        e.storage().instance().set(&DataKey::Name, &name);
        e.storage().instance().set(&DataKey::Symbol, &symbol);
        e.storage().instance().set(&DataKey::Decimals, &7u32);
        e.storage().instance().set(&DataKey::TotalSupply, &0i128);
    }

    pub fn set_admin(e: Env, new_admin: Address) {
        let admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        e.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    fn admin(e: &Env) -> Address {
        e.storage().instance().get(&DataKey::Admin).unwrap()
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }
        let admin: Address = Self::admin(&e);
        admin.require_auth();

        let balance = read_balance(&e, &to);
        let total = e.storage().instance().get::<_, i128>(&DataKey::TotalSupply).unwrap();
        save_balance(&e, &to, balance.checked_add(amount).expect("overflow"));
        e.storage().instance().set(&DataKey::TotalSupply, &total.checked_add(amount).expect("overflow"));
    }

    pub fn burn(e: Env, from: Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }
        let admin: Address = Self::admin(&e);
        admin.require_auth();

        let balance = read_balance(&e, &from);
        let total = e.storage().instance().get::<_, i128>(&DataKey::TotalSupply).unwrap();
        if balance < amount {
            panic!("insufficient balance");
        }
        save_balance(&e, &from, balance - amount);
        e.storage().instance().set(&DataKey::TotalSupply, &(total - amount));
    }

    pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }
        from.require_auth();

        let from_balance = read_balance(&e, &from);
        if from_balance < amount {
            panic!("insufficient balance");
        }
        let to_balance = read_balance(&e, &to);
        save_balance(&e, &from, from_balance - amount);
        save_balance(&e, &to, to_balance.checked_add(amount).expect("overflow"));
    }

    pub fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }
        spender.require_auth();

        let allowance = read_allowance(&e, &from, &spender);
        if allowance < amount {
            panic!("insufficient allowance");
        }
        let from_balance = read_balance(&e, &from);
        if from_balance < amount {
            panic!("insufficient balance");
        }
        let to_balance = read_balance(&e, &to);
        save_allowance(&e, &from, &spender, allowance - amount);
        save_balance(&e, &from, from_balance - amount);
        save_balance(&e, &to, to_balance.checked_add(amount).expect("overflow"));
    }

    pub fn approve(e: Env, from: Address, spender: Address, amount: i128, _expiration_ledger: u32) {
        if amount < 0 {
            panic!("amount must be non-negative");
        }
        from.require_auth();
        save_allowance(&e, &from, &spender, amount);
    }

    pub fn balance(e: Env, addr: Address) -> i128 {
        read_balance(&e, &addr)
    }

    pub fn total_supply(e: Env) -> i128 {
        e.storage().instance().get(&DataKey::TotalSupply).unwrap()
    }

    pub fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        read_allowance(&e, &from, &spender)
    }

    pub fn name(e: Env) -> String {
        e.storage().instance().get(&DataKey::Name).unwrap()
    }

    pub fn symbol(e: Env) -> String {
        e.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    pub fn decimals(e: Env) -> u32 {
        e.storage().instance().get(&DataKey::Decimals).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Env, String};

    #[test]
    fn test_initialize_sets_values() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);

        assert_eq!(client.name(), name);
        assert_eq!(client.symbol(), symbol);
        assert_eq!(client.decimals(), 7);
        assert_eq!(client.total_supply(), 0);
    }

    #[test]
    fn test_initialize_panics_twice() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            CreditToken::initialize(
                e.clone(),
                admin.clone(),
                name.clone(),
                symbol.clone(),
            );
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_mint_increases_balance_and_supply() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);
        client.mock_all_auths().mint(&user, &1000);

        assert_eq!(client.balance(&user), 1000);
        assert_eq!(client.total_supply(), 1000);
    }

    #[test]
    fn test_burn_decreases_balance_and_supply() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);
        client.mock_all_auths().mint(&user, &1000);
        client.mock_all_auths().burn(&user, &300);

        assert_eq!(client.balance(&user), 700);
        assert_eq!(client.total_supply(), 700);
    }

    #[test]
    fn test_burn_fails_if_insufficient() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);
        client.mock_all_auths().mint(&user, &100);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            CreditToken::burn(
                e.clone(),
                user.clone(),
                200,
            );
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_transfer_moves_balance() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user1 = Address::generate(&e);
        let user2 = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);
        client.mock_all_auths().mint(&user1, &1000);
        client.mock_all_auths().transfer(&user1, &user2, &300);

        assert_eq!(client.balance(&user1), 700);
        assert_eq!(client.balance(&user2), 300);
    }

    #[test]
    fn test_transfer_fails_if_insufficient() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user1 = Address::generate(&e);
        let user2 = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);
        client.mock_all_auths().mint(&user1, &100);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            CreditToken::transfer(
                e.clone(),
                user1.clone(),
                user2.clone(),
                200,
            );
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_transfer_from_with_allowance() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let owner = Address::generate(&e);
        let spender = Address::generate(&e);
        let recipient = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);
        client.mock_all_auths().mint(&owner, &1000);
        client.mock_all_auths().approve(&owner, &spender, &500, &100000);
        client.mock_all_auths().transfer_from(&spender, &owner, &recipient, &200);

        assert_eq!(client.balance(&owner), 800);
        assert_eq!(client.balance(&recipient), 200);
        assert_eq!(client.allowance(&owner, &spender), 300);
    }

    #[test]
    fn test_transfer_from_fails_without_allowance() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let owner = Address::generate(&e);
        let spender = Address::generate(&e);
        let recipient = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);
        client.mock_all_auths().mint(&owner, &1000);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            CreditToken::transfer_from(
                e.clone(),
                spender.clone(),
                owner.clone(),
                recipient.clone(),
                200,
            );
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_approve_sets_and_overwrites() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let owner = Address::generate(&e);
        let spender = Address::generate(&e);
        let name = String::from_str(&e, "WaterCredit");
        let symbol = String::from_str(&e, "WCR");
        let contract_id = e.register_contract(None, CreditToken);
        let client = CreditTokenClient::new(&e, &contract_id);

        client.initialize(&admin, &name, &symbol);
        client.mock_all_auths().approve(&owner, &spender, &100, &100000);
        assert_eq!(client.allowance(&owner, &spender), 100);

        client.mock_all_auths().approve(&owner, &spender, &250, &100001);
        assert_eq!(client.allowance(&owner, &spender), 250);
    }
}
