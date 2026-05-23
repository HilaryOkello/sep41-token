use soroban_sdk::{contracttype, Address};

#[contracttype]
pub struct AllowanceKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub enum DataKey {
    // Per-address entries use persistent storage so their TTL can be managed independently.
    Balance(Address),
    Allowance(AllowanceKey),
    // Contract-level entries use instance storage: they share the contract's TTL
    // and are archived or restored together with it.
    Admin,
    TotalSupply,
}
