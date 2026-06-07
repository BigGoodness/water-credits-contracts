# Water Credits Smart Contracts

## Contracts

### credit_token
- `initialize(admin, name, symbol)` — Initialize the token
- `mint(to, amount)` — Mint tokens (admin only)
- `burn(from, amount)` — Burn tokens (admin only)
- `transfer(from, to, amount)` — Transfer tokens
- `transfer_from(spender, from, to, amount)` — Transfer with allowance
- `approve(from, spender, amount, expiration_ledger)` — Approve spender
- `balance(addr)` — Get balance
- `total_supply()` — Get total supply
- `allowance(from, spender)` — Get allowance
- `name()` — Get token name
- `symbol()` — Get token symbol
- `decimals()` — Get decimals (7)
- `set_admin(new_admin)` — Change admin

### credit_factory
### verification_oracle
### retirement_registry
### project_registry
### governance
