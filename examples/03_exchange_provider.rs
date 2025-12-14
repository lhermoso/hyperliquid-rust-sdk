//! Example of using the ExchangeProvider for trading operations
//!
//! This example demonstrates Phase 1, 2, and 3 exchange API functionality.
//! Note: Most operations require a funded account and proper setup.
//! The examples here show the API patterns without executing real trades.

use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use hyperliquid_sdk_rs::constants::TIF_GTC;
use hyperliquid_sdk_rs::types::requests::OrderRequest;
use hyperliquid_sdk_rs::{signers::AlloySigner, ExchangeProvider};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the crypto provider for TLS
    rustls::crypto::CryptoProvider::install_default(
        rustls::crypto::aws_lc_rs::default_provider(),
    )
    .expect("Failed to install rustls crypto provider");

    // Create a test signer (DO NOT USE IN PRODUCTION)
    let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer = private_key.parse::<PrivateKeySigner>()?;
    let alloy_signer = AlloySigner { inner: signer };

    // Create ExchangeProvider for testnet
    let exchange = ExchangeProvider::testnet(alloy_signer);

    println!("ExchangeProvider created successfully!");

    // ==================== Phase 1 Order Examples ====================

    println!("\n=== Phase 1 Order Examples ===");

    // Example 1: Create an order with client order ID
    let cloid = Uuid::new_v4();
    let _order = OrderRequest::limit(
        0,         // BTC-USD asset ID
        true,      // buy
        "45000.0", // price
        "0.01",    // size
        TIF_GTC,
    );

    println!("\nOrder created:");
    println!("- Asset: 0 (BTC-USD)");
    println!("- Side: Buy");
    println!("- Price: $45,000");
    println!("- Size: 0.01");
    println!("- Client Order ID: {}", cloid);

    // Example 2: Using OrderBuilder pattern
    let _builder_order = exchange
        .order(0)
        .buy()
        .limit_px("45000.0")
        .size("0.01")
        .cloid(Uuid::new_v4());

    println!("\nOrderBuilder created successfully!");

    // Example 3: Create bulk orders with mixed tracking
    let orders_with_ids = vec![
        (
            OrderRequest::limit(0, true, "44900.0", "0.01", TIF_GTC),
            Some(Uuid::new_v4()),
        ),
        (
            OrderRequest::limit(0, true, "44800.0", "0.01", TIF_GTC),
            None,
        ),
        (
            OrderRequest::limit(0, true, "44700.0", "0.01", TIF_GTC),
            Some(Uuid::new_v4()),
        ),
    ];

    println!("\nBulk orders created:");
    for (i, (order, cloid)) in orders_with_ids.iter().enumerate() {
        println!(
            "- Order {}: price={}, cloid={:?}",
            i + 1,
            &order.limit_px,
            cloid.as_ref().map(|id| id.to_string())
        );
    }

    // Example 4: Different constructor types
    let vault_address: Address = "0x742d35Cc6634C0532925a3b844Bc9e7595f8fA49".parse()?;
    let vault_signer = private_key.parse::<PrivateKeySigner>()?;
    let _vault_exchange = ExchangeProvider::testnet_vault(
        AlloySigner {
            inner: vault_signer,
        },
        vault_address,
    );
    println!(
        "\nVault ExchangeProvider created for address: {}",
        vault_address
    );

    let agent_address: Address = "0x742d35Cc6634C0532925a3b844Bc9e7595f8fA49".parse()?;
    let agent_signer = private_key.parse::<PrivateKeySigner>()?;
    let _agent_exchange = ExchangeProvider::testnet_agent(
        AlloySigner {
            inner: agent_signer,
        },
        agent_address,
    );
    println!(
        "Agent ExchangeProvider created for address: {}",
        agent_address
    );

    // ==================== Phase 1 Account Management ====================

    println!("\n=== Phase 1 Account Management Examples ===");

    // Schedule cancel example (would cancel all orders at specified time)
    println!("\nSchedule cancel example:");
    println!("  exchange.schedule_cancel(Some(timestamp)).await");
    println!("  // Cancels all open orders at the specified time");
    println!("  exchange.schedule_cancel(None).await");
    println!("  // Removes any scheduled cancellation");

    // Sub-account creation example
    println!("\nSub-account creation example:");
    println!("  exchange.create_sub_account(Some(\"Trading Bot\".to_string())).await");

    // Sub-account transfer examples
    println!("\nSub-account transfer examples:");
    println!("  let sub_account: Address = \"0x...\".parse()?;");
    println!("  // Transfer USD to sub-account");
    println!("  exchange.sub_account_transfer(sub_account, true, 1000_000000).await");
    println!("  // Transfer spot tokens to sub-account");
    println!(
        "  exchange.sub_account_spot_transfer(sub_account, true, \"ETH\", \"1.0\").await"
    );

    // USD class transfer example
    println!("\nUSD class transfer example:");
    println!("  // Transfer from spot to perp");
    println!("  exchange.usd_class_transfer(\"1000.0\", true).await");
    println!("  // Transfer from perp to spot");
    println!("  exchange.usd_class_transfer(\"1000.0\", false).await");

    // ==================== Phase 2 Exchange Examples ====================

    println!("\n=== Phase 2 Exchange Examples ===");

    // TWAP Order example
    println!("\nTWAP Order example:");
    println!("  // Place a TWAP order to buy 1.0 BTC over 60 minutes");
    println!("  exchange.twap_order(");
    println!("      0,        // BTC-USD asset ID");
    println!("      true,     // is_buy");
    println!("      \"1.0\",    // total size");
    println!("      false,    // reduce_only");
    println!("      60,       // duration in minutes");
    println!("      true      // randomize execution timing");
    println!("  ).await");
    println!("");
    println!("  // Cancel a TWAP order");
    println!("  exchange.twap_cancel(0, twap_id).await");

    // Multi-sig conversion example
    println!("\nMulti-sig conversion example:");
    println!("  let signer1: Address = \"0x...\".parse()?;");
    println!("  let signer2: Address = \"0x...\".parse()?;");
    println!("  let signer3: Address = \"0x...\".parse()?;");
    println!("");
    println!("  // Convert to multi-sig requiring 2 of 3 signers");
    println!("  exchange.convert_to_multi_sig_user(");
    println!("      vec![(signer1, 1), (signer2, 1), (signer3, 1)],");
    println!("      2  // threshold");
    println!("  ).await");

    // Multi-sig execution example
    println!("\nMulti-sig execution example:");
    println!("  // Execute action on multi-sig account");
    println!("  exchange.multi_sig(");
    println!("      multi_sig_user,");
    println!("      inner_action_json,");
    println!("      signatures  // Vec<(r, s, v)>");
    println!("  ).await");

    // Agent DEX abstraction example
    println!("\nAgent DEX abstraction example:");
    println!("  exchange.agent_enable_dex_abstraction().await");

    // ==================== Phase 3 Exchange Examples ====================

    println!("\n=== Phase 3 Exchange Examples ===");

    // --- Spot Deployment ---

    println!("\n--- Spot Deployment Workflow ---");
    println!("Step 1: Register a new token");
    println!("  exchange.spot_deploy_register_token(");
    println!("      \"MYTOKEN\",     // token name");
    println!("      8,              // sz_decimals");
    println!("      18,             // wei_decimals");
    println!("      \"1000000\",     // max_gas");
    println!("      Some(\"My Custom Token\".to_string())  // full name");
    println!("  ).await");

    println!("\nStep 2: User genesis distribution");
    println!("  exchange.spot_deploy_user_genesis(");
    println!("      \"MYTOKEN\",");
    println!("      vec![");
    println!(
        "          (\"0xUser1...\".to_string(), \"1000000000000000000000\".to_string()),"
    );
    println!(
        "          (\"0xUser2...\".to_string(), \"500000000000000000000\".to_string()),"
    );
    println!("      ],");
    println!("      None  // or Some((existing_token, wei)) to use existing token");
    println!("  ).await");

    println!("\nStep 3: Genesis");
    println!("  exchange.spot_deploy_genesis(");
    println!("      \"MYTOKEN\",");
    println!("      \"1000000000000000000000000\",  // max_supply");
    println!("      None  // no_hyperliquidity flag");
    println!("  ).await");

    println!("\nStep 4: Register spot trading pair");
    println!("  exchange.spot_deploy_register_spot(\"MYTOKEN\", \"USDC\").await");

    println!("\nStep 5: Register hyperliquidity");
    println!("  exchange.spot_deploy_register_hyperliquidity(");
    println!("      \"MYTOKEN/USDC\",");
    println!("      \"1.0\",      // start_px");
    println!("      \"100.0\",    // order_sz");
    println!("      10,          // n_orders");
    println!("      5            // n_seeded_levels");
    println!("  ).await");

    println!("\nOptional: Token management");
    println!("  // Enable freeze privilege");
    println!("  exchange.spot_deploy_enable_freeze_privilege(\"MYTOKEN\").await");
    println!("  // Freeze a user");
    println!("  exchange.spot_deploy_freeze_user(\"MYTOKEN\", user_address, true).await");
    println!("  // Revoke freeze privilege (cannot be re-enabled)");
    println!("  exchange.spot_deploy_revoke_freeze_privilege(\"MYTOKEN\").await");
    println!("  // Set deployer trading fee share");
    println!("  exchange.spot_deploy_set_deployer_trading_fee_share(\"MYTOKEN\", \"0.001\").await");
    println!("  // Enable as quote token");
    println!("  exchange.spot_deploy_enable_quote_token(\"MYTOKEN\").await");

    // --- Perp Deployment ---

    println!("\n--- Perp Deployment Workflow ---");
    println!("Step 1: Register perpetual asset");
    println!("  exchange.perp_deploy_register_asset(");
    println!("      1,              // dex");
    println!("      \"1000000\",     // max_gas");
    println!("      \"MYPERP\",      // coin");
    println!("      4,              // sz_decimals");
    println!("      \"100.0\",       // oracle_px");
    println!("      None,           // margin_table_id");
    println!("      Some(false),    // only_isolated");
    println!("      None            // schema");
    println!("  ).await");

    println!("\nStep 2: Set oracle prices");
    println!("  exchange.perp_deploy_set_oracle(");
    println!("      1,                              // dex");
    println!("      vec![\"100.0\".to_string()],    // oracle_pxs");
    println!("      vec![\"100.5\".to_string()],    // all_mark_pxs");
    println!("      None                            // external_perp_pxs");
    println!("  ).await");

    // --- Validator/Staking ---

    println!("\n--- Validator/Staking Operations ---");

    println!("\nRegister as validator:");
    println!("  let signer_address: Address = \"0x...\".parse()?;");
    println!("  exchange.c_validator_register(");
    println!("      \"192.168.1.1\",        // node_ip");
    println!("      \"My Validator\",        // name");
    println!("      \"Secure validator\",    // description");
    println!("      false,                   // delegations_disabled");
    println!("      500,                     // commission_bps (5%)");
    println!("      signer_address,          // signer");
    println!("      true,                    // unjailed");
    println!("      \"1000000000000000000\"  // initial_wei");
    println!("  ).await");

    println!("\nChange validator profile:");
    println!("  exchange.c_validator_change_profile(");
    println!("      Some(\"192.168.1.2\".to_string()),  // new node_ip");
    println!("      Some(\"Updated Name\".to_string()), // new name");
    println!("      None,                               // description unchanged");
    println!("      None,                               // unjailed unchanged");
    println!(
        "      None,                               // disable_delegations unchanged"
    );
    println!("      Some(300),                          // new commission_bps (3%)");
    println!("      None                                // signer unchanged");
    println!("  ).await");

    println!("\nUnregister validator:");
    println!("  exchange.c_validator_unregister().await");

    println!("\nJail/unjail self:");
    println!("  exchange.c_signer_jail_self().await");
    println!("  exchange.c_signer_unjail_self().await");

    println!("\nToken delegation:");
    println!("  let validator: Address = \"0x...\".parse()?;");
    println!("  // Delegate tokens to validator");
    println!("  exchange.token_delegate(");
    println!("      validator,");
    println!("      \"1000000000000000000\",  // wei");
    println!("      false                      // is_undelegate = false means delegate");
    println!("  ).await");
    println!("");
    println!("  // Undelegate tokens from validator");
    println!("  exchange.token_delegate(");
    println!("      validator,");
    println!("      \"1000000000000000000\",  // wei");
    println!("      true                       // is_undelegate = true");
    println!("  ).await");

    // --- Other Phase 3 Actions ---

    println!("\n--- Other Phase 3 Actions ---");

    println!("\nBig blocks mode:");
    println!("  exchange.use_big_blocks(true).await   // Enable");
    println!("  exchange.use_big_blocks(false).await  // Disable");

    println!("\nNo-op action (for testing/keep-alive):");
    println!("  exchange.noop(1234567890).await");

    // ==================== Summary ====================

    println!("\n=== All examples completed ===");
    println!("\nNOTE: This example only demonstrates the API patterns.");
    println!("To execute actual operations, you would need:");
    println!("1. A funded testnet/mainnet account");
    println!("2. Valid asset IDs for the network");
    println!("3. Appropriate permissions (e.g., validator registration requires stake)");
    println!("4. Proper error handling for production use");

    Ok(())
}
