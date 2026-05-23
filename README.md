# HilaryToken (HIL)

A SEP-41 compliant fungible token on the Stellar network, built with Soroban smart contracts.

## Token Details

| Property | Value |
|---|---|
| Name | HilaryToken |
| Symbol | HIL |
| Decimals | 7 |
| Standard | SEP-41 |

## Features

- **Mint** — admin-only, issues new tokens to any address
- **Transfer** — move tokens between addresses
- **Transfer From** — spender moves tokens on behalf of another address using an approved allowance
- **Burn** — holder destroys their own tokens, reducing total supply
- **Burn From** — spender burns tokens on behalf of another address using an approved allowance
- **Approve / Allowance** — grant and query spending permissions

## Project Structure

```
contracts/
└── sep41-token/
    └── src/
        ├── lib.rs          # module declarations
        ├── our_token.rs    # contract implementation
        ├── token_trait.rs  # SEP-41 TokenInterface trait
        ├── storage.rs      # storage key types
        ├── events.rs       # contract events
        ├── error.rs        # error types
        └── test.rs         # unit tests
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/stellar-cli)

```bash
rustup target add wasm32-unknown-unknown
```

## Build

```bash
stellar contract build
```

The compiled WASM will be at `target/wasm32-unknown-unknown/release/sep41_token.wasm`.

## Test

```bash
cargo test
```

## Deploy to Testnet

Generate and fund a keypair:

```bash
stellar keys generate --global deployer --network testnet
stellar keys fund deployer --network testnet
```

Copy `.env.example` to `.env` inside `contracts/sep41-token/` and fill in your values, then:

```bash
cd contracts/sep41-token
make deploy
```

This builds, uploads the WASM, and deploys in one step. The contract ID is saved to `contract_id.txt`.

To query the admin balance after deploy:

```bash
make balance
```

## Deployed Contract

| Network | Contract ID |
|---|---|
| Testnet | `CCIVE463U53GPGTJVI7XLNE7FND2PXQ6PEOR7HKZANVERD2Q7K6KKAMG` |

## Design Notes

- `__constructor` runs on deployment — mints `initial_supply` to `admin` automatically
- `instance` storage holds `Admin` and `TotalSupply` (contract-level, lives with the contract)
- `persistent` storage holds per-address `Balance` and `Allowance` entries
- All token functions implement `TokenInterface` for compile-time SEP-41 compliance
