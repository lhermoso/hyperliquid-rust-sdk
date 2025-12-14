//! Example of using the order tracking system
//!
//! The order tracking system provides:
//! - Automatic client order ID (CLOID) generation
//! - Order state tracking (pending, submitted, failed)
//! - Order lookup by CLOID
//! - Status queries for monitoring
//!
//! This is useful for:
//! - Correlating orders with responses
//! - Building trading dashboards
//! - Implementing retry logic
//! - Audit trails

use alloy::signers::local::PrivateKeySigner;
use hyperliquid_sdk_rs::{
    constants::TIF_GTC, signers::AlloySigner, types::requests::OrderRequest,
    ExchangeProvider,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the crypto provider for TLS
    rustls::crypto::CryptoProvider::install_default(
        rustls::crypto::aws_lc_rs::default_provider(),
    )
    .expect("Failed to install rustls crypto provider");

    println!("=== Order Tracking System Example ===\n");

    // ==================== Setup ====================

    // Create a test signer (DO NOT USE IN PRODUCTION)
    let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer = private_key.parse::<PrivateKeySigner>()?;
    let alloy_signer = AlloySigner { inner: signer };

    // Create ExchangeProvider WITH order tracking enabled
    let exchange = ExchangeProvider::testnet(alloy_signer).with_order_tracking();

    println!("ExchangeProvider created with order tracking enabled");
    println!(
        "Initial tracked order count: {}\n",
        exchange.tracked_order_count()
    );

    // ==================== Part 1: Automatic CLOID Generation ====================

    println!("--- Part 1: Automatic CLOID Generation ---\n");

    println!("When order tracking is enabled:");
    println!("  - Orders without CLOIDs get auto-generated UUIDs");
    println!("  - All orders are automatically tracked");
    println!("  - Status updates are recorded after submission");

    // Create an order without a CLOID
    let _order = OrderRequest::limit(
        0,         // BTC-USD asset ID
        true,      // buy
        "45000.0", // price
        "0.01",    // size
        TIF_GTC,
    );

    println!("\nOrder created (without CLOID):");
    println!("  Asset: 0 (BTC-USD)");
    println!("  Side: Buy");
    println!("  Price: $45,000");
    println!("  Size: 0.01");

    // In a real scenario, this would submit and track the order:
    // let result = exchange.place_order(&order).await?;
    // The order is now tracked with an auto-generated CLOID

    println!("\n[Demo mode - showing tracking pattern]");
    println!(
        "After place_order(), the order would be tracked with an auto-generated CLOID"
    );

    // ==================== Part 2: Custom CLOID Usage ====================

    println!("\n--- Part 2: Custom CLOID Usage ---\n");

    // Generate a custom CLOID
    let my_cloid = Uuid::new_v4();
    println!("Custom CLOID: {}", my_cloid);

    // Create an order with the custom CLOID
    let _order_with_cloid = OrderRequest::limit(
        0,         // BTC-USD
        true,      // buy
        "44500.0", // price
        "0.02",    // size
        TIF_GTC,
    )
    .with_cloid(Some(my_cloid));

    println!("Order created with custom CLOID:");
    println!("  Asset: 0");
    println!("  Price: $44,500");
    println!("  Size: 0.02");
    println!("  CLOID: {}", my_cloid);

    // Alternative: use place_order_with_cloid
    // let result = exchange.place_order_with_cloid(order, my_cloid).await?;

    println!("\nYou can also use place_order_with_cloid():");
    println!("  exchange.place_order_with_cloid(order, cloid).await");

    // ==================== Part 3: Order Builder with Tracking ====================

    println!("\n--- Part 3: Order Builder with Tracking ---\n");

    // Using the builder pattern with CLOID
    let builder_cloid = Uuid::new_v4();
    let _tracked_order = exchange
        .order(0)
        .buy()
        .limit_px("44000.0")
        .size("0.03")
        .cloid(builder_cloid)
        .build()?;

    println!("Order built with builder pattern:");
    println!("  Asset: 0");
    println!("  Price: $44,000");
    println!("  Size: 0.03");
    println!("  CLOID: {}", builder_cloid);

    // ==================== Part 4: Querying Tracked Orders ====================

    println!("\n--- Part 4: Querying Tracked Orders ---\n");

    println!("Available query methods:");
    println!("");

    println!("1. Get a specific order by CLOID:");
    println!("   let order = exchange.get_tracked_order(&cloid);");
    println!("   if let Some(tracked) = order {{");
    println!("       println!(\"Status: {{:?}}\", tracked.status);");
    println!("   }}");

    println!("\n2. Get all tracked orders:");
    println!("   let all_orders = exchange.get_all_tracked_orders();");
    println!("   println!(\"Total tracked: {{}}\", all_orders.len());");

    println!("\n3. Get orders by status:");
    println!("   let pending = exchange.get_pending_orders();");
    println!("   let submitted = exchange.get_submitted_orders();");
    println!("   let failed = exchange.get_failed_orders();");

    println!("\n4. Get order count:");
    println!("   let count = exchange.tracked_order_count();");

    // Demo the actual API (even though we have no orders in demo mode)
    println!("\nCurrent tracking state:");
    println!("  All orders: {}", exchange.get_all_tracked_orders().len());
    println!("  Pending: {}", exchange.get_pending_orders().len());
    println!("  Submitted: {}", exchange.get_submitted_orders().len());
    println!("  Failed: {}", exchange.get_failed_orders().len());

    // ==================== Part 5: Order Status Lifecycle ====================

    println!("\n--- Part 5: Order Status Lifecycle ---\n");

    println!("Orders go through these states:");
    println!("");
    println!("  Pending    -> Order created, not yet sent to exchange");
    println!("      |");
    println!("      v");
    println!("  Submitted  -> Order sent and accepted by exchange");
    println!("      |");
    println!("      v");
    println!("  (Exchange manages the rest: open, filled, cancelled)");
    println!("");
    println!("  OR");
    println!("");
    println!("  Pending");
    println!("      |");
    println!("      v");
    println!("  Failed(reason) -> Order submission failed");

    println!("\nThe tracking system captures the submission phase.");
    println!("For order fill status, use the Info API:");
    println!("  info.order_status(user, oid).await");
    println!("  info.open_orders(user).await");
    println!("  info.user_fills(user).await");

    // ==================== Part 6: Practical Patterns ====================

    println!("\n--- Part 6: Practical Patterns ---\n");

    println!("PATTERN 1: Retry failed orders");
    println!("```");
    println!("let failed = exchange.get_failed_orders();");
    println!("for order in failed {{");
    println!("    println!(\"Retrying order {{}}\", order.cloid);");
    println!("    // Modify and resubmit");
    println!("    let new_cloid = Uuid::new_v4();");
    println!(
        "    exchange.place_order_with_cloid(order.order.clone(), new_cloid).await?;"
    );
    println!("}}");
    println!("```");

    println!("\nPATTERN 2: Correlate with external systems");
    println!("```");
    println!("// Generate deterministic CLOID from external ID");
    println!("let external_id = \"order-12345\";");
    println!("let cloid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, external_id.as_bytes());");
    println!("exchange.place_order_with_cloid(order, cloid).await?;");
    println!("```");

    println!("\nPATTERN 3: Order confirmation tracking");
    println!("```");
    println!("let cloid = Uuid::new_v4();");
    println!("exchange.place_order_with_cloid(order, cloid).await?;");
    println!("");
    println!("// Later, check status");
    println!("if let Some(tracked) = exchange.get_tracked_order(&cloid) {{");
    println!("    match &tracked.status {{");
    println!("        OrderStatus::Submitted => println!(\"Order confirmed!\"),");
    println!("        OrderStatus::Failed(e) => println!(\"Order failed: {{}}\", e),");
    println!("        OrderStatus::Pending => println!(\"Still pending...\"),");
    println!("    }}");
    println!("}}");
    println!("```");

    // ==================== Part 7: Cleanup ====================

    println!("\n--- Part 7: Cleanup and Memory Management ---\n");

    println!("Clear all tracked orders:");
    println!("  exchange.clear_tracked_orders();");

    println!("\nNote: Tracked orders are stored in memory.");
    println!("For long-running applications, periodically clear old orders");
    println!("or implement your own persistence layer.");

    // Demo cleanup
    exchange.clear_tracked_orders();
    println!("\nCleared. Order count: {}", exchange.tracked_order_count());

    // ==================== Summary ====================

    println!("\n=== Order Tracking Example Complete ===\n");
    println!("Key APIs:");
    println!("  Setup:");
    println!("    - ExchangeProvider::testnet(signer).with_order_tracking()");
    println!("");
    println!("  Submission:");
    println!("    - place_order(&order) - auto-generates CLOID if tracking enabled");
    println!("    - place_order_with_cloid(order, cloid) - use specific CLOID");
    println!("");
    println!("  Queries:");
    println!("    - get_tracked_order(&cloid) -> Option<TrackedOrder>");
    println!("    - get_all_tracked_orders() -> Vec<TrackedOrder>");
    println!("    - get_pending_orders() -> Vec<TrackedOrder>");
    println!("    - get_submitted_orders() -> Vec<TrackedOrder>");
    println!("    - get_failed_orders() -> Vec<TrackedOrder>");
    println!("    - tracked_order_count() -> usize");
    println!("");
    println!("  Cleanup:");
    println!("    - clear_tracked_orders()");

    Ok(())
}
