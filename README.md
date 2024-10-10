# SolanaKeyConverter

## Description

SolanaKeyConverter is a Rust utility that converts Base58-encoded private keys into byte arrays, streamlining wallet management for Solana developers. This tool simplifies the handling of private keys, making it easier to integrate with Solana's wallet systems and improve your development workflow.

## Features

- Convert Base58 private keys to byte arrays.
- Lightweight and easy to use.
- Built with Rust for performance and reliability.

## Installation

To use SolanaKeyConverter, simply add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
solana-sdk = "1.15.2"
bs58 = "0.4"
solana-client = "1.15.2"
solana-program = "1.15.2"
borsh = "0.10.3"
solana-idlgen = { git = "https://github.com/deanmlittle/solana-idlgen.git" }
```
