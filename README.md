# Hyperliquid Rust SDK

![Hyperliquid Rust SDK](ferrofluid-background.png)

> **Fork Notice**: This is an actively maintained fork of [ferrofluid](https://github.com/ControlCplusControlV/ferrofluid) by ControlCplusControlV. This fork provides complete coverage of all Hyperliquid API endpoints and is under active development.

A high-performance Rust SDK for the Hyperliquid Protocol, built with a "thin wrapper, maximum control" philosophy.

[![Crates.io](https://img.shields.io/crates/v/hyperliquid-rust-sdk.svg)](https://crates.io/crates/hyperliquid-rust-sdk)

## Features

- 🚀 **High Performance**: Uses `simd-json` for 10x faster JSON parsing than `serde_json`
- 🔒 **Type-Safe**: Strongly-typed Rust bindings with compile-time guarantees
- 🎯 **Direct Control**: No hidden retry logic or complex abstractions - you control the flow
- ⚡ **Fast WebSockets**: Built on `fastwebsockets` for 3-4x performance over `tungstenite`
- 🛠️ **Builder Support**: Native support for MEV builders with configurable fees
- 📊 **Complete API Coverage**: Info, Exchange, and WebSocket providers for all endpoints

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
hyperliquid-rust-sdk = "0.1.0"
```

## Quick Start

### Reading Market Data

```rust
use ferrofluid::{InfoProvider, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let info = InfoProvider::new(Network::Mainnet);
    
    // Get all mid prices
    let mids = info.all_mids().await?;
    println!("BTC mid price: {}", mids["BTC"]);
    
    // Get L2 order book
    let book = info.l2_book("ETH").await?;
    println!("ETH best bid: {:?}", book.levels[0][0]);
    
    Ok(())
}
```

### Placing Orders

```rust
use ferrofluid::{ExchangeProvider, signers::AlloySigner};
use alloy::signers::local::PrivateKeySigner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup signer
    let signer = PrivateKeySigner::random();
    let hyperliquid_signer = AlloySigner { inner: signer };
    
    // Create exchange provider
    let exchange = ExchangeProvider::mainnet(hyperliquid_signer);
    
    // Place an order using the builder pattern
    let result = exchange.order(0) // BTC perpetual
        .limit_buy("50000", "0.001")
        .reduce_only(false)
        .send()
        .await?;
        
    println!("Order placed: {:?}", result);
    Ok(())
}
```

### WebSocket Subscriptions

```rust
use ferrofluid::{WsProvider, Network, types::ws::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WsProvider::connect(Network::Mainnet).await?;
    
    // Subscribe to BTC order book
    let (_id, mut rx) = ws.subscribe_l2_book("BTC").await?;
    ws.start_reading().await?;
    
    // Handle updates
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::L2Book(book) => {
                println!("BTC book update: {:?}", book.data.coin);
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

### Managed WebSocket with Auto-Reconnect

For production use, consider the `ManagedWsProvider` which adds automatic reconnection and keep-alive:

```rust
use ferrofluid::{ManagedWsProvider, WsConfig, Network};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure with custom settings
    let config = WsConfig {
        ping_interval: Duration::from_secs(30),
        auto_reconnect: true,
        exponential_backoff: true,
        ..Default::default()
    };
    
    let ws = ManagedWsProvider::connect(Network::Mainnet, config).await?;
    
    // Subscriptions automatically restore on reconnect
    let (_id, mut rx) = ws.subscribe_l2_book("BTC").await?;
    ws.start_reading().await?;
    
    // Your subscriptions survive disconnections!
    while let Some(msg) = rx.recv().await {
        // Handle messages...
    }
    
    Ok(())
}
```

## Examples

The `examples/` directory contains comprehensive examples:

- `00_symbols.rs` - Working with pre-defined symbols
- `01_info_types.rs` - Using the Info provider for market data
- `02_info_provider.rs` - Advanced Info provider usage
- `03_exchange_provider.rs` - Placing and managing orders
- `04_websocket.rs` - Real-time WebSocket subscriptions
- `05_builder_orders.rs` - Using MEV builders for orders
- `06_basis_trade.rs` - Example basis trading strategy
- `07_managed_websocket.rs` - WebSocket with auto-reconnect and keep-alive

Run examples with:
```bash
cargo run --example 01_info_types
```

## Architecture

Ferrofluid follows a modular architecture:

```
ferrofluid/
├── providers/
│   ├── info.rs      // Read-only market data (HTTP)
│   ├── exchange.rs  // Trading operations (HTTP, requires signer)
│   └── websocket.rs // Real-time subscriptions
├── types/
│   ├── actions.rs   // EIP-712 signable actions
│   ├── requests.rs  // Order, Cancel, Modify structs
│   ├── responses.rs // API response types
│   └── ws.rs        // WebSocket message types
└── signers/
    └── signer.rs    // HyperliquidSigner trait
```

## Performance

Ferrofluid is designed for maximum performance:

- **JSON Parsing**: Uses `simd-json` for vectorized parsing
- **HTTP Client**: Built on `hyper` + `tower` for connection pooling
- **WebSocket**: Uses `fastwebsockets` for minimal overhead
- **Zero-Copy**: Minimizes allocations where possible

## Builder Support

Native support for MEV builders with configurable fees:

```rust
let exchange = ExchangeProvider::mainnet_builder(signer, builder_address);

// All orders automatically include builder info
let order = exchange.order(0)
    .limit_buy("50000", "0.001")
    .send()
    .await?;
    
// Or specify custom builder fee
let result = exchange.place_order_with_builder_fee(&order_request, 10).await?;
```

## Rate Limiting

Built-in rate limiter respects Hyperliquid's limits:

```rust
// Rate limiting is automatic
let result = info.l2_book("BTC").await?; // Uses 1 weight
let fills = info.user_fills(address).await?; // Uses 2 weight
```

## Error Handling

Comprehensive error types with `thiserror`:

```rust
match exchange.place_order(&order).await {
    Ok(status) => println!("Success: {:?}", status),
    Err(HyperliquidError::RateLimited { available, required }) => {
        println!("Rate limited: need {} but only {} available", required, available);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Testing

Run the test suite:

```bash
cargo test
```

Integration tests against testnet:
```bash
cargo test --features testnet
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This project is a fork of [ferrofluid](https://github.com/ControlCplusControlV/ferrofluid) by [ControlCplusControlV](https://github.com/ControlCplusControlV). We are grateful for the original work that made this SDK possible.

Built with high-performance crates from the Rust ecosystem:
- [alloy-rs](https://github.com/alloy-rs/alloy) for Ethereum primitives
- [hyperliquid-rust-sdk](https://github.com/hyperliquid-dex/hyperliquid-rust-sdk/tree/master) (official SDK reference)
- [hyper](https://hyper.rs/) for HTTP
- [fastwebsockets](https://github.com/littledivy/fastwebsockets) for WebSocket
- [simd-json](https://github.com/simd-lite/simd-json) for JSON parsing

---
