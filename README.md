# SAC Swap Mint Admin

A Soroban smart contract that serves as a SAC (Stellar Asset Contract) admin, enabling token swaps by burning one asset and minting another.

## Overview

This contract accepts an "allow" asset and administers another SAC. Users can call `swap_mint` to convert their "allow" tokens into the administered asset at a 1:1 ratio.

## Contract Functions

- **Constructor**: Accepts `admin_asset` (the SAC this contract administers) and `allow_asset` (the SAC accepted for swapping)
- **swap_mint(sender, amount)**: Transfers `amount` of allow asset from sender, burns it, then mints equivalent amount of admin asset to sender

## Project Structure

```text
.
├── contracts
│   └── sac-swap-mint-admin
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```
