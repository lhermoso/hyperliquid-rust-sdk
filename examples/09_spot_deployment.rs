//! Example of spot token deployment workflow
//!
//! Hyperliquid allows users to deploy their own spot tokens with:
//! - Custom tokenomics (supply, decimals)
//! - Optional hyperliquidity (automated market making)
//! - Trading pair creation against USDC or other tokens
//! - Freeze controls for compliance
//!
//! This example demonstrates the complete spot deployment workflow.

use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use hyperliquid_sdk_rs::{
    providers::InfoProvider, signers::AlloySigner, ExchangeProvider,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the crypto provider for TLS
    rustls::crypto::CryptoProvider::install_default(
        rustls::crypto::aws_lc_rs::default_provider(),
    )
    .expect("Failed to install rustls crypto provider");

    println!("=== Spot Token Deployment Example ===\n");

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

    // ==================== Part 1: Query Deployment State ====================

    println!("--- Part 1: Query Deployment State ---\n");

    // Check current deployment state for this user
    match info.spot_deploy_state(user_address).await {
        Ok(state) => {
            println!("Current deployment state: {:?}", state);
        }
        Err(e) => println!("Error fetching deployment state: {}", e),
    }

    // Check spot pair deployment auction status
    match info.spot_pair_deploy_auction_status("PURR", "USDC").await {
        Ok(status) => {
            println!("PURR/USDC auction status: {:?}", status);
        }
        Err(e) => println!("Error fetching auction status: {}", e),
    }

    // ==================== Part 2: Complete Deployment Workflow ====================

    println!("\n--- Part 2: Complete Deployment Workflow ---\n");

    println!("=== STEP 1: Register Token ===\n");
    println!("Register a new token with the following parameters:");
    println!("  Token Name: MYTOKEN");
    println!("  Size Decimals: 8 (for trading precision)");
    println!("  Wei Decimals: 18 (standard ERC-20)");
    println!("  Max Gas: 1000000");
    println!("  Full Name: My Custom Token");

    println!("\nCode:");
    println!("  exchange.spot_deploy_register_token(");
    println!("      \"MYTOKEN\",");
    println!("      8,       // sz_decimals");
    println!("      18,      // wei_decimals");
    println!("      \"1000000\",");
    println!("      Some(\"My Custom Token\".to_string())");
    println!("  ).await");

    println!("\n=== STEP 2: User Genesis (Initial Distribution) ===\n");
    println!("Distribute tokens to initial holders:");

    let holder1: Address = "0x1111111111111111111111111111111111111111".parse()?;
    let holder2: Address = "0x2222222222222222222222222222222222222222".parse()?;

    println!("  Holder 1: {} - 1,000,000 tokens", holder1);
    println!("  Holder 2: {} - 500,000 tokens", holder2);

    println!("\nCode:");
    println!("  exchange.spot_deploy_user_genesis(");
    println!("      \"MYTOKEN\",");
    println!("      vec![");
    println!(
        "          (\"{}\".to_string(), \"1000000000000000000000000\".to_string()),",
        holder1
    );
    println!(
        "          (\"{}\".to_string(), \"500000000000000000000000\".to_string()),",
        holder2
    );
    println!("      ],");
    println!("      None  // or Some((existing_token, wei)) to use existing holdings");
    println!("  ).await");

    println!("\n=== STEP 3: Genesis (Finalize Token) ===\n");
    println!("Finalize the token with total supply:");
    println!("  Max Supply: 10,000,000 tokens");
    println!("  Hyperliquidity: Enabled");

    println!("\nCode:");
    println!("  exchange.spot_deploy_genesis(");
    println!("      \"MYTOKEN\",");
    println!("      \"10000000000000000000000000\",  // max supply in wei");
    println!("      None  // no_hyperliquidity = None (enabled)");
    println!("  ).await");

    println!("\n=== STEP 4: Register Trading Pair ===\n");
    println!("Create a trading pair against USDC:");
    println!("  Base: MYTOKEN");
    println!("  Quote: USDC");

    println!("\nCode:");
    println!("  exchange.spot_deploy_register_spot(\"MYTOKEN\", \"USDC\").await");

    println!("\n=== STEP 5: Register Hyperliquidity ===\n");
    println!("Set up automated market making:");
    println!("  Starting Price: 1.0 USDC");
    println!("  Order Size: 100 tokens per level");
    println!("  Number of Orders: 10 per side");
    println!("  Seeded Levels: 5");

    println!("\nCode:");
    println!("  exchange.spot_deploy_register_hyperliquidity(");
    println!("      \"MYTOKEN/USDC\",");
    println!("      \"1.0\",    // start_px");
    println!("      \"100.0\",  // order_sz");
    println!("      10,        // n_orders");
    println!("      5          // n_seeded_levels");
    println!("  ).await");

    // ==================== Part 3: Optional Token Controls ====================

    println!("\n--- Part 3: Optional Token Controls ---\n");

    println!("=== Enable Freeze Privilege ===");
    println!("Allow the deployer to freeze user accounts:");
    println!("  exchange.spot_deploy_enable_freeze_privilege(\"MYTOKEN\").await");

    println!("\n=== Freeze a User ===");
    println!("Freeze a specific user's token balance:");
    let user_to_freeze: Address = "0xBADBADBADBADBADBADBADBADBADBADBADBADBAD0".parse()?;
    println!(
        "  exchange.spot_deploy_freeze_user(\"MYTOKEN\", {}, true).await",
        user_to_freeze
    );

    println!("\n=== Unfreeze a User ===");
    println!("Restore a user's ability to transfer:");
    println!(
        "  exchange.spot_deploy_freeze_user(\"MYTOKEN\", {}, false).await",
        user_to_freeze
    );

    println!("\n=== Revoke Freeze Privilege ===");
    println!("Permanently revoke the ability to freeze (cannot be re-enabled):");
    println!("  exchange.spot_deploy_revoke_freeze_privilege(\"MYTOKEN\").await");
    println!("\n  WARNING: This is IRREVERSIBLE!");

    println!("\n=== Set Deployer Trading Fee Share ===");
    println!("Take a percentage of trading fees:");
    println!("  exchange.spot_deploy_set_deployer_trading_fee_share(\"MYTOKEN\", \"0.001\").await");
    println!("  // 0.001 = 0.1% of trading fees go to deployer");

    println!("\n=== Enable as Quote Token ===");
    println!("Allow your token to be used as a quote currency:");
    println!("  exchange.spot_deploy_enable_quote_token(\"MYTOKEN\").await");
    println!("  // Now other tokens can trade against MYTOKEN");

    // ==================== Part 4: Token Economics Planning ====================

    println!("\n--- Part 4: Token Economics Planning ---\n");

    println!("Before deploying, plan your tokenomics:");
    println!("");
    println!("1. TOTAL SUPPLY");
    println!("   - Fixed supply vs inflationary");
    println!("   - Distribution schedule");
    println!("");
    println!("2. INITIAL DISTRIBUTION");
    println!("   - Team allocation (consider vesting)");
    println!("   - Community allocation");
    println!("   - Liquidity pool allocation");
    println!("");
    println!("3. DECIMALS");
    println!("   - wei_decimals: On-chain precision (usually 18)");
    println!("   - sz_decimals: Trading UI precision (4-8 typical)");
    println!("");
    println!("4. LIQUIDITY");
    println!("   - Hyperliquidity parameters");
    println!("   - Initial price discovery");
    println!("   - Order depth");
    println!("");
    println!("5. FREEZE CONTROLS");
    println!("   - Compliance requirements");
    println!("   - Emergency controls");
    println!("   - Decentralization timeline");

    // ==================== Part 5: Deployment Checklist ====================

    println!("\n--- Part 5: Deployment Checklist ---\n");

    println!("Pre-deployment:");
    println!("  [ ] Token name finalized (cannot be changed)");
    println!("  [ ] Decimal configuration verified");
    println!("  [ ] Initial distribution addresses verified");
    println!("  [ ] Initial distribution amounts calculated");
    println!("  [ ] Hyperliquidity parameters planned");
    println!("  [ ] Max supply determined");
    println!("");
    println!("Deployment steps:");
    println!("  [ ] 1. Register token");
    println!("  [ ] 2. User genesis for all initial holders");
    println!("  [ ] 3. Genesis to finalize");
    println!("  [ ] 4. Register spot pair");
    println!("  [ ] 5. Register hyperliquidity");
    println!("");
    println!("Post-deployment:");
    println!("  [ ] Verify token appears in spot metadata");
    println!("  [ ] Test trading with small amounts");
    println!("  [ ] Announce to community");
    println!("  [ ] Set up fee collection (if applicable)");

    // ==================== Summary ====================

    println!("\n=== Spot Deployment Example Complete ===\n");
    println!("Key APIs for spot deployment:");
    println!("");
    println!("Registration:");
    println!("  - spot_deploy_register_token(name, sz_dec, wei_dec, gas, full_name)");
    println!("  - spot_deploy_user_genesis(token, [(user, wei)], existing)");
    println!("  - spot_deploy_genesis(token, max_supply, no_hyperliquidity)");
    println!("");
    println!("Trading Setup:");
    println!("  - spot_deploy_register_spot(base, quote)");
    println!("  - spot_deploy_register_hyperliquidity(spot, px, sz, n, levels)");
    println!("");
    println!("Token Controls:");
    println!("  - spot_deploy_enable_freeze_privilege(token)");
    println!("  - spot_deploy_freeze_user(token, user, freeze)");
    println!("  - spot_deploy_revoke_freeze_privilege(token)");
    println!("  - spot_deploy_set_deployer_trading_fee_share(token, share)");
    println!("  - spot_deploy_enable_quote_token(token)");
    println!("");
    println!("Query APIs:");
    println!("  - spot_deploy_state(user)");
    println!("  - spot_pair_deploy_auction_status(base, quote)");

    Ok(())
}
