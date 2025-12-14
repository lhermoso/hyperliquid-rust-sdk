//! Example of TWAP (Time-Weighted Average Price) order workflows
//!
//! TWAP orders split a large order into smaller pieces executed over time
//! to minimize market impact. This is useful for:
//! - Executing large orders without moving the market
//! - Reducing slippage on size
//! - Automating DCA-style entries/exits
//!
//! This example demonstrates:
//! - Placing TWAP orders
//! - Monitoring TWAP order state via WebSocket
//! - Cancelling TWAP orders
//! - Querying TWAP fills

use alloy::signers::local::PrivateKeySigner;
use hyperliquid_rust_sdk::{
    providers::{InfoProvider, WsProvider},
    signers::AlloySigner,
    types::ws::Message,
    ExchangeProvider, Network,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the crypto provider for TLS
    rustls::crypto::CryptoProvider::install_default(
        rustls::crypto::aws_lc_rs::default_provider(),
    )
    .expect("Failed to install rustls crypto provider");

    println!("=== TWAP Order Workflow Example ===\n");

    // ==================== Setup ====================

    // Create a test signer (DO NOT USE IN PRODUCTION)
    let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer = private_key.parse::<PrivateKeySigner>()?;
    let user_address = signer.address();
    let alloy_signer = AlloySigner { inner: signer };

    println!("User address: {}", user_address);

    // Create providers
    let _exchange = ExchangeProvider::testnet(alloy_signer);
    let info = InfoProvider::testnet();

    println!("Providers created for testnet");

    // ==================== Part 1: Place a TWAP Order ====================

    println!("\n--- Part 1: Placing a TWAP Order ---\n");

    // TWAP parameters
    let asset = 0; // BTC perpetual
    let is_buy = true;
    let total_size = "1.0"; // Total size to execute
    let reduce_only = false;
    let duration_minutes = 60; // Execute over 60 minutes
    let randomize = true; // Randomize execution timing to avoid detection

    println!("TWAP Order Parameters:");
    println!("  Asset: {} (BTC perpetual)", asset);
    println!("  Direction: {}", if is_buy { "BUY" } else { "SELL" });
    println!("  Total Size: {} BTC", total_size);
    println!("  Duration: {} minutes", duration_minutes);
    println!("  Reduce Only: {}", reduce_only);
    println!("  Randomize: {}", randomize);

    // In a real scenario, you would execute this:
    // let result = exchange.twap_order(
    //     asset,
    //     is_buy,
    //     total_size,
    //     reduce_only,
    //     duration_minutes,
    //     randomize,
    // ).await?;
    // println!("TWAP order placed: {:?}", result);

    println!("\n[Demo mode - not executing actual order]");
    println!("To place a TWAP order, call:");
    println!(
        "  exchange.twap_order({}, {}, \"{}\", {}, {}, {}).await",
        asset, is_buy, total_size, reduce_only, duration_minutes, randomize
    );

    // ==================== Part 2: Monitor TWAP State via WebSocket ====================

    println!("\n--- Part 2: Monitoring TWAP State via WebSocket ---\n");

    // Connect to WebSocket
    let mut ws = WsProvider::connect(Network::Testnet).await?;
    println!("Connected to WebSocket");

    // Subscribe to TWAP states
    let (_twap_states_id, mut twap_states_rx) =
        ws.subscribe_twap_states(user_address).await?;
    println!("Subscribed to TWAP states");

    // Subscribe to TWAP slice fills
    let (_twap_fills_id, mut twap_fills_rx) =
        ws.subscribe_user_twap_slice_fills(user_address).await?;
    println!("Subscribed to TWAP slice fills");

    // Subscribe to TWAP history
    let (_twap_history_id, mut twap_history_rx) =
        ws.subscribe_user_twap_history(user_address).await?;
    println!("Subscribed to TWAP history");

    // Start reading messages
    ws.start_reading().await?;

    println!("\nMonitoring for TWAP updates (5 second demo)...\n");

    // Monitor for a short time
    let timeout = tokio::time::sleep(std::time::Duration::from_secs(5));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(msg) = twap_states_rx.recv() => {
                if let Message::TwapStates(states) = msg {
                    println!("[TwapStates] {} active TWAP orders:", states.data.twap_states.len());
                    for state in &states.data.twap_states {
                        println!("  TWAP ID: {:?}", state);
                        // In a real implementation, you would access fields like:
                        // state.twap_id, state.asset, state.is_buy,
                        // state.sz, state.executed_sz, state.status, etc.
                    }
                }
            }

            Some(msg) = twap_fills_rx.recv() => {
                if let Message::UserTwapSliceFills(fills) = msg {
                    println!("[TwapFills] {} slice fills:", fills.data.twap_slice_fills.len());
                    for fill in &fills.data.twap_slice_fills {
                        println!("  Fill: {:?}", fill);
                    }
                }
            }

            Some(msg) = twap_history_rx.recv() => {
                if let Message::UserTwapHistory(history) = msg {
                    println!("[TwapHistory] {} history entries:", history.data.twap_history.len());
                    for entry in &history.data.twap_history {
                        println!("  Entry: {:?}", entry);
                    }
                }
            }

            _ = &mut timeout => {
                println!("WebSocket demo timeout");
                break;
            }
        }
    }

    // ==================== Part 3: Query TWAP Fills via Info API ====================

    println!("\n--- Part 3: Query TWAP Fills via Info API ---\n");

    match info.user_twap_slice_fills(user_address).await {
        Ok(fills) => {
            println!("Found {} TWAP slice fills", fills.len());
            for fill in fills.iter().take(5) {
                println!("  Fill: {:?}", fill);
            }
        }
        Err(e) => println!("Error querying TWAP fills: {}", e),
    }

    // ==================== Part 4: Cancel a TWAP Order ====================

    println!("\n--- Part 4: Cancelling a TWAP Order ---\n");

    // Example TWAP ID (would come from the place order response or state updates)
    let twap_id: u64 = 12345;

    println!("To cancel a TWAP order, call:");
    println!("  exchange.twap_cancel({}, {}).await", asset, twap_id);

    // In a real scenario:
    // let result = exchange.twap_cancel(asset, twap_id).await?;
    // println!("TWAP order cancelled: {:?}", result);

    println!("\n[Demo mode - not executing actual cancellation]");

    // ==================== Part 5: TWAP Best Practices ====================

    println!("\n--- Part 5: TWAP Best Practices ---\n");

    println!("1. DURATION SELECTION:");
    println!(
        "   - Longer durations = less market impact but more exposure to price movement"
    );
    println!("   - Consider volatility: use shorter durations in volatile markets");
    println!("   - Minimum: 5 minutes, Maximum: 1440 minutes (24 hours)");

    println!("\n2. SIZE CONSIDERATIONS:");
    println!("   - TWAP is most useful for sizes that would move the market");
    println!("   - Small orders might be better as regular limit orders");
    println!("   - Check your size against average volume");

    println!("\n3. RANDOMIZATION:");
    println!("   - Enable randomize=true to avoid predictable execution patterns");
    println!("   - This helps prevent front-running");

    println!("\n4. MONITORING:");
    println!("   - Subscribe to twap_states for real-time status");
    println!("   - Subscribe to twap_slice_fills for individual fill updates");
    println!("   - Use info.user_twap_slice_fills() for historical fills");

    println!("\n5. CANCELLATION:");
    println!("   - You can cancel at any time with twap_cancel()");
    println!("   - Already executed portions remain filled");

    // ==================== Summary ====================

    println!("\n=== TWAP Example Complete ===\n");
    println!("Key APIs demonstrated:");
    println!("  Exchange: twap_order(), twap_cancel()");
    println!("  Info: user_twap_slice_fills()");
    println!("  WebSocket: subscribe_twap_states(), subscribe_user_twap_slice_fills(), subscribe_user_twap_history()");

    Ok(())
}
