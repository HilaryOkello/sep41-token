use soroban_sdk::{contract, contractimpl, Address, Env, String};

use crate::{
    error::ContractError,
    events::{Approval, Burn, Mint, Transfer},
    storage::{AllowanceKey, DataKey},
    token_trait::TokenInterface,
};

#[contract]
pub struct HilToken;

#[contractimpl]
impl HilToken {
    pub fn __constructor(env: Env, admin: Address, initial_supply: i128) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
        Self::mint_to(&env, &admin, initial_supply);
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), ContractError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();
        admin.require_auth();

        Self::mint_to(&env, &to, amount);

        Ok(())
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(AllowanceKey { from, spender }))
            .unwrap_or(0)
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), ContractError> {
        from.require_auth();

        let key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        env.storage().persistent().set(&key, &amount);

        Approval {
            from,
            spender,
            amount,
            live_until_ledger,
        }
        .publish(&env);

        Ok(())
    }

    pub fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        spender.require_auth();

        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        let allowance: i128 = env
            .storage()
            .persistent()
            .get(&allowance_key)
            .unwrap_or(0);

        if allowance < amount {
            return Err(ContractError::InsufficientAllowance);
        }

        env.storage()
            .persistent()
            .set(&allowance_key, &(allowance - amount));

        let sender_balance = Self::balance(env.clone(), from.clone());
        let receiver_balance = Self::balance(env.clone(), to.clone());

        if sender_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(sender_balance - amount));

        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(receiver_balance + amount));

        Transfer { from, to, amount }.publish(&env);

        Ok(())
    }

    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        from.require_auth();

        let sender_balance = Self::balance(env.clone(), from.clone());
        let receiver_balance = Self::balance(env.clone(), to.clone());

        if sender_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(sender_balance - amount));

        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(receiver_balance + amount));

        Transfer { from, to, amount }.publish(&env);

        Ok(())
    }

    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), ContractError> {
        from.require_auth();

        let balance = Self::balance(env.clone(), from.clone());

        if balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(balance - amount));

        let total: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total - amount));

        Burn { from, amount }.publish(&env);

        Ok(())
    }

    pub fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        spender.require_auth();

        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        let allowance: i128 = env
            .storage()
            .persistent()
            .get(&allowance_key)
            .unwrap_or(0);

        if allowance < amount {
            return Err(ContractError::InsufficientAllowance);
        }

        env.storage()
            .persistent()
            .set(&allowance_key, &(allowance - amount));

        let balance = Self::balance(env.clone(), from.clone());

        if balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(balance - amount));

        let total: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total - amount));

        Burn { from, amount }.publish(&env);

        Ok(())
    }

    pub fn decimals(_env: Env) -> u32 {
        7
    }

    pub fn name(env: Env) -> String {
        String::from_str(&env, "HilaryToken")
    }

    pub fn symbol(env: Env) -> String {
        String::from_str(&env, "HIL")
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    fn mint_to(env: &Env, to: &Address, amount: i128) {
        let balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(balance + amount));

        let total: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total + amount));

        Mint {
            to: to.clone(),
            amount,
        }
        .publish(env);
    }
}

impl TokenInterface for HilToken {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        HilToken::allowance(env, from, spender)
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        HilToken::approve(env, from, spender, amount, live_until_ledger).unwrap()
    }

    fn balance(env: Env, id: Address) -> i128 {
        HilToken::balance(env, id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        HilToken::transfer(env, from, to, amount).unwrap()
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        HilToken::transfer_from(env, spender, from, to, amount).unwrap()
    }

    fn burn(env: Env, from: Address, amount: i128) {
        HilToken::burn(env, from, amount).unwrap()
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        HilToken::burn_from(env, spender, from, amount).unwrap()
    }

    fn decimals(env: Env) -> u32 {
        HilToken::decimals(env)
    }

    fn name(env: Env) -> String {
        HilToken::name(env)
    }

    fn symbol(env: Env) -> String {
        HilToken::symbol(env)
    }
}
