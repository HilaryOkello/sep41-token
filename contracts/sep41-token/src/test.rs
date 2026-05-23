#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::our_token::{HilToken, HilTokenClient};

const INITIAL_SUPPLY: i128 = 1_000_000_0000000; // 1M tokens (7 decimals)

struct SetUpResult<'a> {
    env: Env,
    client: HilTokenClient<'a>,
    admin: Address,
    sender: Address,
    receiver: Address,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let receiver = Address::generate(&env);

    let contract_id = env.register(HilToken, (&admin, INITIAL_SUPPLY));
    let client = HilTokenClient::new(&env, &contract_id);

    SetUpResult {
        env,
        client,
        admin,
        sender,
        receiver,
    }
}

#[test]
fn test_name() {
    let s = setup();
    assert_eq!(s.client.name(), String::from_str(&s.env, "HilaryToken"));
}

#[test]
fn test_symbol() {
    let s = setup();
    assert_eq!(s.client.symbol(), String::from_str(&s.env, "HIL"));
}

#[test]
fn test_decimals() {
    let s = setup();
    assert_eq!(s.client.decimals(), 7);
}

#[test]
fn test_constructor_mints_initial_supply_to_admin() {
    let s = setup();
    assert_eq!(s.client.balance(&s.admin), INITIAL_SUPPLY);
    assert_eq!(s.client.total_supply(), INITIAL_SUPPLY);
}

#[test]
fn test_mint_by_admin() {
    let s = setup();
    let amount = 500_0000000_i128;
    s.client.mint(&s.sender, &amount);
    assert_eq!(s.client.balance(&s.sender), amount);
    assert_eq!(s.client.total_supply(), INITIAL_SUPPLY + amount);
}

#[test]
#[should_panic]
fn test_mint_non_admin_fails() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let contract_id = env.register(HilToken, (&admin, INITIAL_SUPPLY));
    let client = HilTokenClient::new(&env, &contract_id);

    // no mock_all_auths — admin.require_auth() will panic
    let attacker = Address::generate(&env);
    client.mint(&attacker, &100_i128);
}

#[test]
fn test_transfer() {
    let s = setup();
    let amount = 1000_0000000_i128;

    s.client.mint(&s.sender, &amount);
    s.client.transfer(&s.sender, &s.receiver, &amount);

    assert_eq!(s.client.balance(&s.sender), 0);
    assert_eq!(s.client.balance(&s.receiver), amount);
}

#[test]
#[should_panic]
fn test_transfer_insufficient_funds() {
    let s = setup();
    s.client.transfer(&s.sender, &s.receiver, &100_i128);
}

#[test]
fn test_approve_and_transfer_from() {
    let s = setup();
    let amount = 500_0000000_i128;

    s.client.mint(&s.sender, &amount);
    s.client.approve(&s.sender, &s.receiver, &amount, &999_999);
    assert_eq!(s.client.allowance(&s.sender, &s.receiver), amount);

    s.client.transfer_from(&s.receiver, &s.sender, &s.receiver, &amount);

    assert_eq!(s.client.balance(&s.sender), 0);
    assert_eq!(s.client.balance(&s.receiver), amount);
    assert_eq!(s.client.allowance(&s.sender, &s.receiver), 0);
}

#[test]
#[should_panic]
fn test_transfer_from_insufficient_allowance() {
    let s = setup();
    let amount = 500_0000000_i128;

    s.client.mint(&s.sender, &amount);
    s.client.approve(&s.sender, &s.receiver, &100_i128, &999_999);
    s.client.transfer_from(&s.receiver, &s.sender, &s.receiver, &amount);
}

#[test]
fn test_burn() {
    let s = setup();
    let amount = 200_0000000_i128;

    s.client.mint(&s.sender, &amount);
    s.client.burn(&s.sender, &amount);

    assert_eq!(s.client.balance(&s.sender), 0);
    assert_eq!(s.client.total_supply(), INITIAL_SUPPLY);
}

#[test]
#[should_panic]
fn test_burn_insufficient_funds() {
    let s = setup();
    s.client.burn(&s.sender, &100_i128);
}

#[test]
fn test_approve_and_burn_from() {
    let s = setup();
    let amount = 300_0000000_i128;

    s.client.mint(&s.sender, &amount);
    s.client.approve(&s.sender, &s.receiver, &amount, &999_999);
    s.client.burn_from(&s.receiver, &s.sender, &amount);

    assert_eq!(s.client.balance(&s.sender), 0);
    assert_eq!(s.client.allowance(&s.sender, &s.receiver), 0);
    assert_eq!(s.client.total_supply(), INITIAL_SUPPLY);
}

#[test]
#[should_panic]
fn test_burn_from_insufficient_allowance() {
    let s = setup();
    let amount = 300_0000000_i128;

    s.client.mint(&s.sender, &amount);
    s.client.approve(&s.sender, &s.receiver, &50_i128, &999_999);
    s.client.burn_from(&s.receiver, &s.sender, &amount);
}
