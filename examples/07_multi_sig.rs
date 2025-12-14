//! Example of multi-sig operations
//!
//! Multi-sig accounts require multiple signers to authorize transactions.
//! This is useful for:
//! - Team treasury management
//! - Enhanced security for large accounts
//! - Governance and approval workflows
//!
//! This example demonstrates:
//! - Converting an account to multi-sig
//! - Understanding multi-sig thresholds and weights
//! - Querying multi-sig configuration
//! - Executing multi-sig transactions (conceptual)

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

    println!("=== Multi-Sig Operations Example ===\n");

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

    // ==================== Part 1: Understanding Multi-Sig ====================

    println!("\n--- Part 1: Understanding Multi-Sig ---\n");

    println!("Multi-sig accounts use a threshold-based signing system:");
    println!("- Each authorized signer has a weight (e.g., 1, 2, 3)");
    println!("- A threshold defines the minimum total weight required");
    println!("- Multiple signers must collaborate to reach the threshold");

    println!("\nExample configurations:");
    println!("  2-of-3: 3 signers with weight 1 each, threshold 2");
    println!("  3-of-5: 5 signers with weight 1 each, threshold 3");
    println!("  Weighted: Signer A (weight 2), B (weight 1), C (weight 1), threshold 2");

    // ==================== Part 2: Query Existing Multi-Sig Config ====================

    println!("\n--- Part 2: Query Multi-Sig Configuration ---\n");

    // Check if an address is a multi-sig and get its signers
    let multi_sig_address: Address =
        "0x1234567890123456789012345678901234567890".parse()?;

    match info.user_to_multi_sig_signers(multi_sig_address).await {
        Ok(multi_sig_info) => {
            println!("Multi-sig configuration for {}:", multi_sig_address);
            println!("  Threshold: {}", multi_sig_info.threshold);
            println!("  Signers:");
            for signer in &multi_sig_info.signers {
                println!("    {} (weight: {})", signer.address, signer.weight);
            }
        }
        Err(e) => {
            println!("Not a multi-sig account or error: {}", e);
        }
    }

    // Check user role to see if it's a multi-sig
    match info.user_role(multi_sig_address).await {
        Ok(role) => {
            println!("\nUser role: {:?}", role);
        }
        Err(e) => {
            println!("Error checking user role: {}", e);
        }
    }

    // ==================== Part 3: Convert to Multi-Sig ====================

    println!("\n--- Part 3: Convert to Multi-Sig ---\n");

    // Example: Set up a 2-of-3 multi-sig
    let signer1: Address = "0x1111111111111111111111111111111111111111".parse()?;
    let signer2: Address = "0x2222222222222222222222222222222222222222".parse()?;
    let signer3: Address = "0x3333333333333333333333333333333333333333".parse()?;

    println!("Converting account to 2-of-3 multi-sig:");
    println!("  Signer 1: {} (weight: 1)", signer1);
    println!("  Signer 2: {} (weight: 1)", signer2);
    println!("  Signer 3: {} (weight: 1)", signer3);
    println!("  Threshold: 2");

    // In a real scenario:
    // let result = exchange.convert_to_multi_sig_user(
    //     vec![
    //         (signer1, 1),
    //         (signer2, 1),
    //         (signer3, 1),
    //     ],
    //     2, // threshold
    // ).await?;
    // println!("Converted to multi-sig: {:?}", result);

    println!("\n[Demo mode - not executing actual conversion]");
    println!("To convert to multi-sig, call:");
    println!("  exchange.convert_to_multi_sig_user(");
    println!("      vec![(signer1, 1), (signer2, 1), (signer3, 1)],");
    println!("      2  // threshold");
    println!("  ).await");

    println!("\nWARNING: Converting to multi-sig is IRREVERSIBLE!");
    println!("Make sure all signers are valid and accessible before converting.");

    // ==================== Part 4: Weighted Multi-Sig Example ====================

    println!("\n--- Part 4: Weighted Multi-Sig Configuration ---\n");

    println!("Example: Corporate treasury with weighted voting");
    println!("  CEO: weight 3");
    println!("  CFO: weight 2");
    println!("  Board Member 1: weight 1");
    println!("  Board Member 2: weight 1");
    println!("  Threshold: 4");

    println!("\nThis allows:");
    println!("  - CEO + any one other signer (3+2=5 or 3+1=4)");
    println!("  - CFO + both board members (2+1+1=4)");
    println!("  - Cannot act with just board members (1+1=2 < 4)");

    let ceo: Address = "0xCE0CE0CE0CE0CE0CE0CE0CE0CE0CE0CE0CE0CE0C".parse()?;
    let cfo: Address = "0xCF0CF0CF0CF0CF0CF0CF0CF0CF0CF0CF0CF0CF0C".parse()?;
    let board1: Address = "0xB0A1B0A1B0A1B0A1B0A1B0A1B0A1B0A1B0A1B0A1".parse()?;
    let board2: Address = "0xB0A2B0A2B0A2B0A2B0A2B0A2B0A2B0A2B0A2B0A2".parse()?;

    println!("\nCode to create this configuration:");
    println!("  exchange.convert_to_multi_sig_user(");
    println!("      vec![");
    println!("          ({}, 3),  // CEO", ceo);
    println!("          ({}, 2),  // CFO", cfo);
    println!("          ({}, 1),  // Board 1", board1);
    println!("          ({}, 1),  // Board 2", board2);
    println!("      ],");
    println!("      4  // threshold");
    println!("  ).await");

    // ==================== Part 5: Executing Multi-Sig Transactions ====================

    println!("\n--- Part 5: Executing Multi-Sig Transactions ---\n");

    println!("To execute a transaction on a multi-sig account:");
    println!("1. Prepare the inner action (e.g., place order, transfer)");
    println!("2. Collect signatures from enough signers to meet threshold");
    println!("3. Submit the multi_sig action with all signatures");

    println!("\nExample workflow for a USD transfer:");
    println!("```");
    println!("// 1. Create the inner action (as JSON)");
    println!("let inner_action = serde_json::json!({{");
    println!("    \"type\": \"usdSend\",");
    println!("    \"destination\": \"0x...\",");
    println!("    \"amount\": \"1000.0\",");
    println!("    \"time\": current_timestamp");
    println!("}});");
    println!("");
    println!("// 2. Each signer signs the action hash");
    println!("// (This happens off-chain, typically via a UI or script)");
    println!("let signatures = vec![");
    println!("    (r1, s1, v1),  // Signer 1's signature");
    println!("    (r2, s2, v2),  // Signer 2's signature");
    println!("];");
    println!("");
    println!("// 3. One signer submits the multi-sig transaction");
    println!("exchange.multi_sig(");
    println!("    multi_sig_address,");
    println!("    inner_action,");
    println!("    signatures,");
    println!(").await");
    println!("```");

    // ==================== Part 6: Multi-Sig Security Best Practices ====================

    println!("\n--- Part 6: Security Best Practices ---\n");

    println!("1. SIGNER SELECTION:");
    println!("   - Use hardware wallets for all signers");
    println!("   - Distribute signers across geographic locations");
    println!("   - Use different custody solutions for different signers");

    println!("\n2. THRESHOLD SELECTION:");
    println!("   - 2-of-3 minimum for any significant funds");
    println!("   - Consider 3-of-5 for larger treasuries");
    println!("   - Never use 1-of-N (defeats the purpose)");

    println!("\n3. OPERATIONAL SECURITY:");
    println!("   - Test with small amounts first");
    println!("   - Document the signing process");
    println!("   - Have backup procedures for lost keys");
    println!("   - Regular security audits");

    println!("\n4. RECOVERY PLANNING:");
    println!("   - Multi-sig conversion is IRREVERSIBLE");
    println!("   - Plan for signer key loss scenarios");
    println!("   - Consider time-locked recovery mechanisms");

    // ==================== Summary ====================

    println!("\n=== Multi-Sig Example Complete ===\n");
    println!("Key APIs demonstrated:");
    println!("  Exchange: convert_to_multi_sig_user(), multi_sig()");
    println!("  Info: user_to_multi_sig_signers(), user_role()");

    Ok(())
}
