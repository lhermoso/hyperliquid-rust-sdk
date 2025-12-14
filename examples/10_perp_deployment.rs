//! Example of perpetual asset deployment workflow
//!
//! Hyperliquid allows deploying custom perpetual markets on DEXs.
//! This enables:
//! - Custom perpetual contracts for any asset
//! - Custom oracle configurations
//! - Integration with the Hyperliquid trading infrastructure
//!
//! This example demonstrates the perp deployment workflow.

use alloy::signers::local::PrivateKeySigner;
use hyperliquid_rust_sdk::{
    providers::InfoProvider, signers::AlloySigner, ExchangeProvider,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the crypto provider for TLS
    rustls::crypto::CryptoProvider::install_default(
        rustls::crypto::aws_lc_rs::default_provider(),
    )
    .expect("Failed to install rustls crypto provider");

    println!("=== Perpetual Asset Deployment Example ===\n");

    // ==================== Setup ====================

    // Create a test signer (DO NOT USE IN PRODUCTION)
    let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer = private_key.parse::<PrivateKeySigner>()?;
    let user_address = signer.address();
    let alloy_signer = AlloySigner { inner: signer };

    println!("Deployer address: {}", user_address);

    // Create providers
    let _exchange = ExchangeProvider::testnet(alloy_signer);
    let info = InfoProvider::testnet();

    println!("Providers created for testnet\n");

    // ==================== Part 1: Query DEX and Auction Status ====================

    println!("--- Part 1: Query DEX and Auction Status ---\n");

    // Query available perp DEXs
    match info.perp_dexs().await {
        Ok(dexs) => {
            println!("Available Perp DEXs ({}):", dexs.len());
            for dex in &dexs {
                println!("  DEX: {:?}", dex);
            }
        }
        Err(e) => println!("Error fetching perp DEXs: {}", e),
    }

    // Query perp deployment auction status
    match info.perp_deploy_auction_status().await {
        Ok(status) => {
            println!("\nPerp Deployment Auction Status:");
            println!("  {:?}", status);
        }
        Err(e) => println!("Error fetching auction status: {}", e),
    }

    // ==================== Part 2: Perp Deployment Workflow ====================

    println!("\n--- Part 2: Perp Deployment Workflow ---\n");

    println!("=== STEP 1: Register Perpetual Asset ===\n");

    println!("Register a new perpetual contract with these parameters:");
    println!("  DEX ID: 1 (your assigned DEX)");
    println!("  Coin: MYPERP");
    println!("  Size Decimals: 4 (trading precision)");
    println!("  Oracle Price: 100.0 (initial price)");
    println!("  Max Gas: 1000000");

    println!("\nCode:");
    println!("  use hyperliquid_rust_sdk::types::actions::PerpDeployRegisterAsset;");
    println!("");
    println!("  let asset = PerpDeployRegisterAsset {{");
    println!("      dex: 1,");
    println!("      max_gas: \"1000000\".to_string(),");
    println!("      coin: \"MYPERP\".to_string(),");
    println!("      sz_decimals: 4,");
    println!("      oracle_px: \"100.0\".to_string(),");
    println!("      margin_table_id: None,");
    println!("      only_isolated: Some(false),");
    println!("      schema: None,");
    println!("  }};");
    println!("  exchange.perp_deploy_register_asset(asset).await");

    println!("\n=== STEP 2: Set Oracle Prices ===\n");

    println!("Configure oracle prices for your perp:");
    println!("  Oracle Price: 100.0");
    println!("  Mark Price: 100.5");

    println!("\nCode:");
    println!("  exchange.perp_deploy_set_oracle(");
    println!("      1,                              // dex");
    println!("      vec![\"100.0\".to_string()],    // oracle_pxs");
    println!("      vec![\"100.5\".to_string()],    // all_mark_pxs");
    println!("      None                            // external_perp_pxs (optional)");
    println!("  ).await");

    // ==================== Part 3: Oracle Configuration ====================

    println!("\n--- Part 3: Oracle Configuration ---\n");

    println!("Oracles are crucial for perpetual contracts:");
    println!("");
    println!("1. ORACLE PRICE");
    println!("   - Used for funding rate calculations");
    println!("   - Should reflect spot/index price");
    println!("   - Must be updated regularly");
    println!("");
    println!("2. MARK PRICE");
    println!("   - Used for margin calculations");
    println!("   - Used for liquidation triggers");
    println!("   - Dampened to prevent manipulation");
    println!("");
    println!("3. EXTERNAL PERP PRICES");
    println!("   - Optional: prices from external perp markets");
    println!("   - Used for cross-market arbitrage protection");

    println!("\nOracle Update Example:");
    println!("  // Regular oracle update (should be called frequently)");
    println!("  exchange.perp_deploy_set_oracle(");
    println!("      1,                              // dex");
    println!("      vec![\"101.5\".to_string()],    // new oracle price");
    println!("      vec![\"101.3\".to_string()],    // new mark price");
    println!("      Some(vec![                      // external prices");
    println!("          \"101.2\".to_string(),      // Binance perp");
    println!("          \"101.4\".to_string(),      // Bybit perp");
    println!("      ])");
    println!("  ).await");

    // ==================== Part 4: Margin Configuration ====================

    println!("\n--- Part 4: Margin Configuration ---\n");

    println!("Perpetual contracts support different margin modes:");
    println!("");
    println!("1. CROSS MARGIN (only_isolated = false)");
    println!("   - Shared margin across all positions");
    println!("   - More capital efficient");
    println!("   - Risk of cascading liquidations");
    println!("");
    println!("2. ISOLATED MARGIN (only_isolated = true)");
    println!("   - Separate margin per position");
    println!("   - Limited loss to allocated margin");
    println!("   - Less capital efficient");
    println!("");
    println!("For new perps, consider starting with only_isolated = true");
    println!("for safer trading until the market matures.");

    // ==================== Part 5: DEX Management ====================

    println!("\n--- Part 5: DEX Management ---\n");

    println!("Check user's DEX abstraction settings:");
    match info.user_dex_abstraction(user_address).await {
        Ok(abstraction) => {
            println!("  DEX Abstraction: {:?}", abstraction);
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nEnable DEX abstraction for your agent:");
    println!("  exchange.agent_enable_dex_abstraction().await");
    println!("");
    println!("This allows your agent to interact with DEX-specific features.");

    // ==================== Part 6: Deployment Best Practices ====================

    println!("\n--- Part 6: Deployment Best Practices ---\n");

    println!("1. ORACLE INFRASTRUCTURE");
    println!("   - Set up reliable price feeds");
    println!("   - Have redundant data sources");
    println!("   - Implement circuit breakers");
    println!("   - Update prices frequently (every block if possible)");
    println!("");
    println!("2. INITIAL PARAMETERS");
    println!("   - Start with conservative margin requirements");
    println!("   - Use isolated margin initially");
    println!("   - Set reasonable position limits");
    println!("");
    println!("3. LIQUIDITY");
    println!("   - Ensure sufficient market maker participation");
    println!("   - Consider incentives for liquidity providers");
    println!("   - Monitor order book depth");
    println!("");
    println!("4. MONITORING");
    println!("   - Watch funding rates");
    println!("   - Monitor open interest");
    println!("   - Track liquidation events");
    println!("   - Alert on oracle deviations");
    println!("");
    println!("5. RISK MANAGEMENT");
    println!("   - Implement position limits");
    println!("   - Have emergency pause capability");
    println!("   - Regular security reviews");

    // ==================== Part 7: Deployment Checklist ====================

    println!("\n--- Part 7: Deployment Checklist ---\n");

    println!("Pre-deployment:");
    println!("  [ ] DEX ID obtained through auction");
    println!("  [ ] Oracle infrastructure ready");
    println!("  [ ] Price feeds verified");
    println!("  [ ] Margin parameters calculated");
    println!("  [ ] Market maker commitments secured");
    println!("");
    println!("Deployment:");
    println!("  [ ] Register perpetual asset");
    println!("  [ ] Set initial oracle prices");
    println!("  [ ] Verify asset appears in meta");
    println!("  [ ] Test with small positions");
    println!("");
    println!("Post-deployment:");
    println!("  [ ] Oracle update automation running");
    println!("  [ ] Monitoring dashboards live");
    println!("  [ ] Liquidity providers active");
    println!("  [ ] Community announcement");

    // ==================== Summary ====================

    println!("\n=== Perp Deployment Example Complete ===\n");
    println!("Key APIs for perpetual deployment:");
    println!("");
    println!("Deployment:");
    println!("  - perp_deploy_register_asset(dex, gas, coin, decimals, oracle, ...)");
    println!("  - perp_deploy_set_oracle(dex, oracle_pxs, mark_pxs, external)");
    println!("");
    println!("Query APIs:");
    println!("  - perp_dexs() - List available DEXs");
    println!("  - perp_deploy_auction_status() - Check auction status");
    println!("  - user_dex_abstraction(user) - Check DEX abstraction");
    println!("");
    println!("DEX Management:");
    println!("  - agent_enable_dex_abstraction() - Enable DEX features");

    Ok(())
}
