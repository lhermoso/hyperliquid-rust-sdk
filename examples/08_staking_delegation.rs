//! Example of staking and delegation operations
//!
//! Hyperliquid uses a proof-of-stake system where users can:
//! - Delegate tokens to validators to earn staking rewards
//! - Become validators to help secure the network
//!
//! This example demonstrates:
//! - Querying staking/delegation information
//! - Delegating tokens to validators
//! - Undelegating tokens
//! - Validator registration and management

use alloy::primitives::Address;
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

    println!("=== Staking & Delegation Example ===\n");

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

    // ==================== Part 1: Query Staking Summary ====================

    println!("\n--- Part 1: Query Staking Summary ---\n");

    // Get overall staking summary for user
    match info.delegator_summary(user_address).await {
        Ok(summary) => {
            println!("Staking Summary for {}:", user_address);
            println!("  Total Delegated: {:?} HYPE", summary.delegated);
            println!("  Undelegating: {:?} HYPE", summary.undelegating);
            println!("  Total Rewards: {:?} HYPE", summary.total_rewards);
        }
        Err(e) => println!("Error fetching delegator summary: {}", e),
    }

    // ==================== Part 2: Query Active Delegations ====================

    println!("\n--- Part 2: Query Active Delegations ---\n");

    match info.delegations(user_address).await {
        Ok(delegations) => {
            println!("Active Delegations ({} total):", delegations.len());
            for delegation in &delegations {
                println!("\n  Validator: {}", delegation.validator);
                println!("    Amount: {} HYPE", delegation.amount);
                if let Some(locked_until) = delegation.locked_until {
                    println!("    Locked Until: {}", locked_until);
                }
            }
        }
        Err(e) => println!("Error fetching delegations: {}", e),
    }

    // ==================== Part 3: Query Staking Rewards ====================

    println!("\n--- Part 3: Query Staking Rewards ---\n");

    match info.delegator_rewards(user_address).await {
        Ok(rewards) => {
            println!("Reward History ({} entries):", rewards.len());
            for reward in rewards.iter().take(5) {
                println!("  Reward: {:?}", reward);
            }
            if rewards.len() > 5 {
                println!("  ... and {} more", rewards.len() - 5);
            }
        }
        Err(e) => println!("Error fetching rewards: {}", e),
    }

    // ==================== Part 4: Query Staking History ====================

    println!("\n--- Part 4: Query Staking History ---\n");

    match info.delegator_history(user_address).await {
        Ok(history) => {
            println!("Staking History ({} entries):", history.len());
            for entry in history.iter().take(5) {
                println!("  Entry: {:?}", entry);
            }
            if history.len() > 5 {
                println!("  ... and {} more", history.len() - 5);
            }
        }
        Err(e) => println!("Error fetching history: {}", e),
    }

    // ==================== Part 5: Delegate Tokens ====================

    println!("\n--- Part 5: Delegate Tokens to Validator ---\n");

    // Example validator address
    let validator: Address = "0xVAL1DAT0RVAL1DAT0RVAL1DAT0RVAL1DAT0R00"
        .parse()
        .unwrap_or(user_address); // Fallback to user if parse fails

    let amount_wei = "1000000000000000000"; // 1 HYPE in wei (18 decimals)

    println!("Delegating tokens:");
    println!("  Validator: {}", validator);
    println!("  Amount: 1 HYPE ({} wei)", amount_wei);

    // In a real scenario:
    // let result = exchange.token_delegate(
    //     validator,
    //     amount_wei,
    //     false,  // is_undelegate = false means delegate
    // ).await?;
    // println!("Delegation successful: {:?}", result);

    println!("\n[Demo mode - not executing actual delegation]");
    println!("To delegate tokens, call:");
    println!("  exchange.token_delegate(");
    println!("      validator_address,");
    println!("      \"1000000000000000000\",  // amount in wei");
    println!("      false                      // is_undelegate");
    println!("  ).await");

    // ==================== Part 6: Undelegate Tokens ====================

    println!("\n--- Part 6: Undelegate Tokens ---\n");

    println!("Undelegating tokens:");
    println!("  Validator: {}", validator);
    println!("  Amount: 1 HYPE ({} wei)", amount_wei);

    println!("\nNote: Undelegation has a cooldown period!");
    println!("Tokens are not immediately available after undelegating.");

    println!("\n[Demo mode - not executing actual undelegation]");
    println!("To undelegate tokens, call:");
    println!("  exchange.token_delegate(");
    println!("      validator_address,");
    println!("      \"1000000000000000000\",  // amount in wei");
    println!("      true                       // is_undelegate = true");
    println!("  ).await");

    // ==================== Part 7: Validator Registration ====================

    println!("\n--- Part 7: Validator Registration (Advanced) ---\n");

    println!("To become a validator, you need:");
    println!("  1. Sufficient stake (initial deposit)");
    println!("  2. Infrastructure to run a validator node");
    println!("  3. Technical expertise for node operations");

    let signer_address: Address = "0x5160E150E150E150E150E150E150E150E150E150"
        .parse()
        .unwrap_or(user_address);

    println!("\nValidator registration parameters:");
    println!("  Node IP: 192.168.1.1");
    println!("  Name: My Validator");
    println!("  Description: Secure and reliable validator");
    println!("  Commission: 5% (500 bps)");
    println!("  Signer: {}", signer_address);
    println!("  Initial stake: 10,000 HYPE");

    println!("\n[Demo mode - not executing actual registration]");
    println!("To register as a validator, call:");
    println!("  exchange.c_validator_register(");
    println!("      \"192.168.1.1\",                   // node_ip");
    println!("      \"My Validator\",                   // name");
    println!("      \"Secure and reliable validator\",  // description");
    println!("      false,                             // delegations_disabled");
    println!("      500,                               // commission_bps (5%)");
    println!("      signer_address,                    // signer");
    println!("      true,                              // unjailed");
    println!("      \"10000000000000000000000\"        // initial_wei (10,000 HYPE)");
    println!("  ).await");

    // ==================== Part 8: Validator Management ====================

    println!("\n--- Part 8: Validator Management ---\n");

    println!("Update validator profile:");
    println!("  exchange.c_validator_change_profile(");
    println!("      Some(\"new_ip\".to_string()),       // update node_ip");
    println!("      Some(\"New Name\".to_string()),     // update name");
    println!("      None,                              // keep description");
    println!("      None,                              // keep unjailed status");
    println!("      None,                              // keep delegations enabled");
    println!("      Some(300),                         // new commission (3%)");
    println!("      None                               // keep signer");
    println!("  ).await");

    println!("\nJail/Unjail operations:");
    println!("  exchange.c_signer_jail_self().await    // Voluntarily jail");
    println!("  exchange.c_signer_unjail_self().await  // Unjail after fixing issues");

    println!("\nUnregister validator:");
    println!("  exchange.c_validator_unregister().await");

    // ==================== Part 9: Staking Best Practices ====================

    println!("\n--- Part 9: Staking Best Practices ---\n");

    println!("FOR DELEGATORS:");
    println!("  1. Research validators before delegating");
    println!("  2. Consider commission rates and uptime");
    println!("  3. Diversify across multiple validators");
    println!("  4. Monitor for slashing events");
    println!("  5. Regularly claim rewards");

    println!("\nFOR VALIDATORS:");
    println!("  1. Maintain high uptime (>99%)");
    println!("  2. Keep node software updated");
    println!("  3. Set reasonable commission rates");
    println!("  4. Communicate with delegators");
    println!("  5. Have backup infrastructure");

    println!("\nRISKS:");
    println!("  - Slashing: Validators can be penalized for misbehavior");
    println!("  - Lock-up: Undelegation has a waiting period");
    println!("  - Commission changes: Validators can change rates");

    // ==================== Summary ====================

    println!("\n=== Staking & Delegation Example Complete ===\n");
    println!("Key APIs demonstrated:");
    println!("");
    println!("Info (Query) APIs:");
    println!("  - delegator_summary(user) - Overall staking status");
    println!("  - delegations(user) - Active delegations");
    println!("  - delegator_rewards(user) - Reward history");
    println!("  - delegator_history(user) - Full staking history");
    println!("");
    println!("Exchange (Action) APIs:");
    println!("  - token_delegate(validator, wei, is_undelegate)");
    println!("  - c_validator_register(...) - Register as validator");
    println!("  - c_validator_change_profile(...) - Update validator");
    println!("  - c_validator_unregister() - Unregister validator");
    println!("  - c_signer_jail_self() / c_signer_unjail_self()");

    Ok(())
}
