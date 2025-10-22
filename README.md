# Switchboard Price Feeds

A simple Solana program that fetches real-time cryptocurrency prices (BTC, SOL, ETH) from Pyth Network price feeds.

## What it does

This program connects to Pyth Network's price feeds on Solana devnet and retrieves current USD prices for:
- Bitcoin (BTC)
- Solana (SOL) 
- Ethereum (ETH)

## Features

- ✅ Fetches live price data from Pyth Network
- ✅ Displays prices in USD with timestamps
- ✅ Built with Anchor framework
- ✅ Includes comprehensive tests

## Prerequisites

- [Anchor](https://www.anchor-lang.com/) framework
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- Node.js and Yarn

## Setup

1. Clone the repository
2. Install dependencies:
   ```bash
   yarn install
   ```

3. Build the program:
   ```bash
   anchor build
   ```

4. Run tests:
   ```bash
   anchor test
   ```

## Usage

The program exposes a `get_prices` instruction that fetches current prices for BTC, SOL, and ETH from Pyth Network feeds.

**Program ID:** `5NMAuN2UdTN5Qm1RYwUJ6HEGhzGSDeAca1TM2bFcKdqH`

## Example Output

```
💰 Price Data:
BTC/USD: $59553.475
SOL/USD: $139.817  
ETH/USD: $2529.670
```

## Price Feed Addresses

- BTC: `HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J`
- SOL: `J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix`
- ETH: `EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw`

## License

ISC
