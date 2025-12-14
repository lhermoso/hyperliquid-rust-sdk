//! Example of using the WebSocket provider for real-time data
//!
//! This example demonstrates Phase 1 and Phase 2 WebSocket subscriptions.
//! Phase 2 adds TWAP states, webData3, and active asset context subscriptions.

use alloy::primitives::Address;
use hyperliquid_rust_sdk::{providers::WsProvider, types::ws::Message, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Install crypto provider for rustls
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install crypto provider");

    // Example user address (replace with a real address for user-specific subscriptions)
    let user: Address = "0x1234567890123456789012345678901234567890".parse()?;

    // Connect to WebSocket
    let mut ws = WsProvider::connect(Network::Mainnet).await?;
    println!("Connected to Hyperliquid WebSocket");

    // ==================== Phase 1 Subscriptions ====================

    println!("\n=== Phase 1 Subscriptions ===");

    // Subscribe to BTC order book
    let (_btc_book_id, mut btc_book_rx) = ws.subscribe_l2_book("BTC").await?;
    println!("Subscribed to BTC L2 book");

    // Subscribe to all mid prices
    let (_mids_id, mut mids_rx) = ws.subscribe_all_mids().await?;
    println!("Subscribed to all mids");

    // Subscribe to trades
    let (_trades_id, mut trades_rx) = ws.subscribe_trades("ETH").await?;
    println!("Subscribed to ETH trades");

    // Subscribe to best bid/offer
    let (_bbo_id, mut bbo_rx) = ws.subscribe_bbo("SOL").await?;
    println!("Subscribed to SOL BBO");

    // Subscribe to user's open orders (requires valid user address)
    let (_open_orders_id, mut open_orders_rx) = ws.subscribe_open_orders(user).await?;
    println!("Subscribed to open orders for user");

    // Subscribe to user's clearinghouse state
    let (_clearinghouse_id, mut clearinghouse_rx) =
        ws.subscribe_clearinghouse_state(user).await?;
    println!("Subscribed to clearinghouse state for user");

    // ==================== Phase 2 Subscriptions ====================

    println!("\n=== Phase 2 Subscriptions ===");

    // Subscribe to TWAP order states
    let (_twap_states_id, mut twap_states_rx) = ws.subscribe_twap_states(user).await?;
    println!("Subscribed to TWAP states for user");

    // Subscribe to webData3 (aggregate user information)
    let (_web_data3_id, mut web_data3_rx) = ws.subscribe_web_data3(user).await?;
    println!("Subscribed to webData3 for user");

    // Subscribe to active asset context
    let (_active_ctx_id, mut active_ctx_rx) =
        ws.subscribe_active_asset_ctx("BTC").await?;
    println!("Subscribed to active asset context for BTC");

    // Subscribe to active asset data (perps only)
    let (_active_data_id, mut active_data_rx) =
        ws.subscribe_active_asset_data(user, "ETH").await?;
    println!("Subscribed to active asset data for ETH");

    // Subscribe to TWAP slice fills
    let (_twap_fills_id, mut twap_fills_rx) =
        ws.subscribe_user_twap_slice_fills(user).await?;
    println!("Subscribed to TWAP slice fills for user");

    // Subscribe to TWAP history
    let (_twap_history_id, mut twap_history_rx) =
        ws.subscribe_user_twap_history(user).await?;
    println!("Subscribed to TWAP history for user");

    // Start reading messages
    ws.start_reading().await?;

    // Handle messages for a limited time (10 seconds for demo)
    let mut message_count = 0;
    let timeout = tokio::time::sleep(std::time::Duration::from_secs(10));
    tokio::pin!(timeout);

    println!("\n=== Receiving messages ===");

    loop {
        tokio::select! {
            // === Phase 1 Message Handlers ===

            // Handle BTC book updates
            Some(msg) = btc_book_rx.recv() => {
                if let Message::L2Book(book) = msg {
                    println!("\n[L2Book] BTC book update:");
                    println!("  Coin: {}", book.data.coin);
                    println!("  Time: {}", book.data.time);
                    if let Some(bids) = book.data.levels.first() {
                        if let Some(best_bid) = bids.first() {
                            println!("  Best bid: {} @ {}", best_bid.sz, best_bid.px);
                        }
                    }
                    if let Some(asks) = book.data.levels.get(1) {
                        if let Some(best_ask) = asks.first() {
                            println!("  Best ask: {} @ {}", best_ask.sz, best_ask.px);
                        }
                    }
                    message_count += 1;
                }
            }

            // Handle all mids updates
            Some(msg) = mids_rx.recv() => {
                if let Message::AllMids(mids) = msg {
                    println!("\n[AllMids] Mid prices update:");
                    for (coin, price) in mids.data.mids.iter().take(5) {
                        println!("  {}: {}", coin, price);
                    }
                    println!("  ... and {} more", mids.data.mids.len().saturating_sub(5));
                    message_count += 1;
                }
            }

            // Handle trades
            Some(msg) = trades_rx.recv() => {
                if let Message::Trades(trades) = msg {
                    println!("\n[Trades] Trade update:");
                    for trade in trades.data.iter().take(3) {
                        println!("  {} {} @ {} ({})",
                            trade.side, trade.sz, trade.px, trade.time);
                    }
                    message_count += 1;
                }
            }

            // Handle BBO updates
            Some(msg) = bbo_rx.recv() => {
                if let Message::Bbo(bbo) = msg {
                    println!("\n[BBO] Best bid/offer for {}:", bbo.data.coin);
                    println!("  Bid: {} @ {}", bbo.data.bbo.bid.sz, bbo.data.bbo.bid.px);
                    println!("  Ask: {} @ {}", bbo.data.bbo.ask.sz, bbo.data.bbo.ask.px);
                    message_count += 1;
                }
            }

            // Handle open orders updates
            Some(msg) = open_orders_rx.recv() => {
                if let Message::OpenOrders(orders) = msg {
                    println!("\n[OpenOrders] Open orders update:");
                    println!("  {} orders", orders.data.orders.len());
                    for order in orders.data.orders.iter().take(3) {
                        println!("  {:?}", order);
                    }
                    message_count += 1;
                }
            }

            // Handle clearinghouse state updates
            Some(msg) = clearinghouse_rx.recv() => {
                if let Message::ClearinghouseState(state) = msg {
                    println!("\n[ClearinghouseState] State update:");
                    println!("  {:?}", state.data);
                    message_count += 1;
                }
            }

            // === Phase 2 Message Handlers ===

            // Handle TWAP states updates
            Some(msg) = twap_states_rx.recv() => {
                if let Message::TwapStates(states) = msg {
                    println!("\n[TwapStates] TWAP states update:");
                    println!("  {} active TWAP orders", states.data.twap_states.len());
                    for state in states.data.twap_states.iter().take(3) {
                        println!("  {:?}", state);
                    }
                    message_count += 1;
                }
            }

            // Handle webData3 updates
            Some(msg) = web_data3_rx.recv() => {
                if let Message::WebData3(data) = msg {
                    println!("\n[WebData3] Aggregate user data update:");
                    println!("  {:?}", data.data);
                    message_count += 1;
                }
            }

            // Handle active asset context updates
            Some(msg) = active_ctx_rx.recv() => {
                if let Message::ActiveAssetCtx(ctx) = msg {
                    println!("\n[ActiveAssetCtx] Asset context update:");
                    println!("  {:?}", ctx.data);
                    message_count += 1;
                }
            }

            // Handle active asset data updates
            Some(msg) = active_data_rx.recv() => {
                if let Message::ActiveAssetData(data) = msg {
                    println!("\n[ActiveAssetData] Asset data update:");
                    println!("  {:?}", data.data);
                    message_count += 1;
                }
            }

            // Handle TWAP slice fills
            Some(msg) = twap_fills_rx.recv() => {
                if let Message::UserTwapSliceFills(fills) = msg {
                    println!("\n[UserTwapSliceFills] TWAP fills update:");
                    println!("  {} fills", fills.data.twap_slice_fills.len());
                    for fill in fills.data.twap_slice_fills.iter().take(3) {
                        println!("  {:?}", fill);
                    }
                    message_count += 1;
                }
            }

            // Handle TWAP history
            Some(msg) = twap_history_rx.recv() => {
                if let Message::UserTwapHistory(history) = msg {
                    println!("\n[UserTwapHistory] TWAP history update:");
                    println!("  {} entries", history.data.twap_history.len());
                    for entry in history.data.twap_history.iter().take(3) {
                        println!("  {:?}", entry);
                    }
                    message_count += 1;
                }
            }

            // Handle timeout
            _ = &mut timeout => {
                println!("\n\nDemo timeout reached after 10 seconds");
                break;
            }

            // Handle channel closure
            else => {
                println!("\nAll channels closed, exiting");
                break;
            }
        }

        // Optional: Exit after certain number of messages
        if message_count >= 20 {
            println!("\n\nReceived {} messages, exiting demo", message_count);
            break;
        }
    }

    println!("\n=== WebSocket demo completed ===");
    println!("\nPhase 1 subscriptions demonstrated:");
    println!("  - L2 order book (subscribe_l2_book)");
    println!("  - All mid prices (subscribe_all_mids)");
    println!("  - Trades (subscribe_trades)");
    println!("  - Best bid/offer (subscribe_bbo)");
    println!("  - Open orders (subscribe_open_orders)");
    println!("  - Clearinghouse state (subscribe_clearinghouse_state)");
    println!("\nPhase 2 subscriptions demonstrated:");
    println!("  - TWAP states (subscribe_twap_states)");
    println!("  - WebData3 (subscribe_web_data3)");
    println!("  - Active asset context (subscribe_active_asset_ctx)");
    println!("  - Active asset data (subscribe_active_asset_data)");
    println!("  - TWAP slice fills (subscribe_user_twap_slice_fills)");
    println!("  - TWAP history (subscribe_user_twap_history)");

    Ok(())
}
