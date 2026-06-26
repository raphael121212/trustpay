# TrustPay Escrow

A decentralized escrow payment dApp built on the Stellar network using Soroban smart contracts, allowing buyers to securely lock XLM or USDC until they confirm that the seller has completed the agreed work.

---

## Problem

Freelancers in the Philippines and other Southeast Asian countries often work with international clients who are hesitant to pay upfront, while freelancers risk not getting paid after delivering their work. This lack of trust leads to delayed projects, cancelled contracts, and financial losses.

---

## Solution

TrustPay Escrow enables buyers to deposit XLM or USDC into a Soroban smart contract before work begins. Once the buyer confirms that the work has been completed, the smart contract automatically releases the payment to the seller, ensuring a secure, transparent, and trustless transaction.

---

## Timeline

| Phase | Description |
|--------|-------------|
| Week 1 | Project planning and UI design |
| Week 2 | Develop Soroban smart contract |
| Week 3 | Build frontend and wallet integration |
| Week 4 | Testing, deployment, and demo preparation |

---

## Stellar Features Used

- Soroban Smart Contracts
- XLM Transfers
- USDC Transfers
- On-chain Escrow Storage
- Authorization using `require_auth()`
- Event Logging

---

## MVP Features

- Create a new escrow agreement
- Lock payment on-chain
- Confirm delivery
- Release payment
- Cancel escrow
- View escrow details

---

## Vision and Purpose

Our vision is to create a trusted payment system for freelancers, clients, and small businesses without relying on expensive third-party escrow services.

By leveraging Stellar's fast settlement, low transaction fees, and Soroban smart contracts, TrustPay Escrow makes cross-border work safer and more accessible.

---

## Prerequisites

Before building this project, install:

- Rust (latest stable)
- Cargo
- Soroban CLI
- Stellar CLI

Check installation:

```bash
rustc --version
cargo --version
soroban --version
stellar --version
```

---

## Build Contract

```bash
soroban contract build
```

Or using Cargo:

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

## Run Tests

```bash
cargo test
```

---

## Deploy to Stellar Testnet

Generate a keypair:

```bash
stellar keys generate alice
```

Deploy:

```bash
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/trustpay_escrow.wasm \
--source alice \
--network testnet
```

After deployment, save the returned Contract ID.

---

## Example CLI Commands

### Create Escrow

```bash
soroban contract invoke \
--id <CONTRACT_ID> \
--source alice \
--network testnet \
-- create_escrow \
--buyer <BUYER_ADDRESS> \
--seller <SELLER_ADDRESS> \
--amount 1000
```

---

### Confirm Escrow

```bash
soroban contract invoke \
--id <CONTRACT_ID> \
--source alice \
--network testnet \
-- confirm \
--id 1
```

---

### Cancel Escrow

```bash
soroban contract invoke \
--id <CONTRACT_ID> \
--source alice \
--network testnet \
-- cancel \
--id 1
```

---

### Get Escrow Details

```bash
soroban contract invoke \
--id <CONTRACT_ID> \
--network testnet \
-- get_escrow \
--id 1
```

---

## Project Structure

```
trustpay-escrow/
│
├── Cargo.toml
├── README.md
│
└── src/
    ├── lib.rs
    └── test.rs
```

---

## Future Improvements

- Support multiple Stellar assets
- Automatic escrow expiration
- Refund mechanism
- Dispute resolution
- Multi-signature approval
- Wallet integration (Freighter, Lobstr)
- Notification system
- Mobile application

---

## License

This project is licensed under the **MIT License**.

```
MIT License

Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
```

---

## Authors

**TrustPay Escrow Team**

Built for the **Stellar Hackathon** using **Soroban Smart Contracts**.
## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CBZFORW7SFFX63MTG5QFWB2ZEUTHRFCGFTH2BUYUVN6XNEQ7G24UBBAU` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CBZFORW7SFFX63MTG5QFWB2ZEUTHRFCGFTH2BUYUVN6XNEQ7G24UBBAU) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/75cf1eb867c3ccf631eb6610515d68de42641a4c6fad3b6e702d421f81757bd4) |
| Deployed | 2026-06-26 07:17:57 UTC |
| Wallet | freighter (`GAZY…5MNB`) |
