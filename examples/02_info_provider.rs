use std::time::{SystemTime, UNIX_EPOCH};

use alloy::primitives::Address;
use hyperliquid_rust_sdk::providers::InfoProvider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create mainnet provider
    let info = InfoProvider::mainnet();

    // Example user address (replace with a real address for actual queries)
    let user: Address = "0x1234567890123456789012345678901234567890".parse()?;

    // ==================== Phase 1 Info Endpoints ====================

    // 1. Test all_mids endpoint
    println!("=== Testing all_mids ===");
    match info.all_mids().await {
        Ok(mids) => {
            println!("Found {} mid prices", mids.len());
            // Print first 5 entries
            for (coin, price) in mids.iter().take(5) {
                println!("{}: {}", coin, price);
            }
        }
        Err(e) => println!("Error fetching mids: {}", e),
    }

    // 2. Test l2_book endpoint
    println!("\n=== Testing l2_book for BTC ===");
    match info.l2_book("BTC").await {
        Ok(book) => {
            println!("BTC Order Book at time {}", book.time);
            println!(
                "Levels: {} bid levels, {} ask levels",
                book.levels[0].len(),
                book.levels[1].len()
            );

            // Show top 3 levels each side
            println!("\nTop 3 Bids:");
            for level in book.levels[0].iter().take(3) {
                println!(
                    "  Price: {}, Size: {}, Count: {}",
                    level.px, level.sz, level.n
                );
            }

            println!("\nTop 3 Asks:");
            for level in book.levels[1].iter().take(3) {
                println!(
                    "  Price: {}, Size: {}, Count: {}",
                    level.px, level.sz, level.n
                );
            }
        }
        Err(e) => println!("Error fetching L2 book: {}", e),
    }

    // 3. Test recent_trades endpoint
    println!("\n=== Testing recent_trades for ETH ===");
    match info.recent_trades("ETH").await {
        Ok(trades) => {
            println!("Recent ETH trades: {} trades", trades.len());
            for trade in trades.iter().take(5) {
                println!(
                    "  Time: {}, Side: {}, Price: {}, Size: {}",
                    trade.time, trade.side, trade.px, trade.sz
                );
            }
        }
        Err(e) => println!("Error fetching recent trades: {}", e),
    }

    // 4. Test candles with builder pattern
    println!("\n=== Testing candles for SOL ===");
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
    let one_hour_ago = now - (60 * 60 * 1000); // 1 hour in milliseconds

    match info
        .candles("SOL")
        .interval("15m")
        .time_range(one_hour_ago, now)
        .send()
        .await
    {
        Ok(candles) => {
            println!("SOL 15m candles: {} candles", candles.len());
            for candle in candles.iter().take(3) {
                println!(
                    "  Time: {}-{}, O: {}, H: {}, L: {}, C: {}, V: {}",
                    candle.time_open,
                    candle.time_close,
                    candle.open,
                    candle.high,
                    candle.low,
                    candle.close,
                    candle.vlm
                );
            }
        }
        Err(e) => println!("Error fetching candles: {}", e),
    }

    // 5. Test meta endpoint
    println!("\n=== Testing meta endpoint ===");
    match info.meta().await {
        Ok(meta) => {
            println!("Found {} assets in universe", meta.universe.len());
            for asset in meta.universe.iter().take(5) {
                println!(
                    "  {}: decimals={}, max_leverage={}, isolated_only={}",
                    asset.name,
                    asset.sz_decimals,
                    asset.max_leverage,
                    asset.only_isolated
                );
            }
        }
        Err(e) => println!("Error fetching meta: {}", e),
    }

    // 6. Test spot_meta endpoint
    println!("\n=== Testing spot_meta endpoint ===");
    match info.spot_meta().await {
        Ok(spot_meta) => {
            println!("Found {} spot pairs", spot_meta.universe.len());
            println!("Found {} tokens", spot_meta.tokens.len());

            println!("\nFirst 5 spot pairs:");
            for pair in spot_meta.universe.iter().take(5) {
                println!(
                    "  {}: index={}, canonical={}, tokens={:?}",
                    pair.name, pair.index, pair.is_canonical, pair.tokens
                );
            }

            println!("\nFirst 5 tokens:");
            for token in spot_meta.tokens.iter().take(5) {
                println!(
                    "  {}: index={}, wei_decimals={}, token_id={}",
                    token.name,
                    token.index,
                    token.wei_decimals,
                    &token.token_id[..16.min(token.token_id.len())]
                );
            }
        }
        Err(e) => println!("Error fetching spot meta: {}", e),
    }

    // 7. Test funding_history with builder
    println!("\n=== Testing funding_history for BTC ===");
    let one_day_ago = now - (24 * 60 * 60 * 1000); // 24 hours in milliseconds

    match info
        .funding_history("BTC")
        .time_range(one_day_ago, now)
        .send()
        .await
    {
        Ok(history) => {
            println!("BTC funding history: {} entries", history.len());
            for entry in history.iter().take(5) {
                println!(
                    "  Time: {}, Rate: {}, Premium: {}",
                    entry.time, entry.funding_rate, entry.premium
                );
            }
        }
        Err(e) => println!("Error fetching funding history: {}", e),
    }

    // 8. Test meta_and_asset_ctxs endpoint
    println!("\n=== Testing meta_and_asset_ctxs endpoint ===");
    match info.meta_and_asset_ctxs().await {
        Ok(data) => {
            println!("Meta: {} assets in universe", data.meta.universe.len());
            println!("Asset contexts: {} entries", data.asset_ctxs.len());
            for ctx in data.asset_ctxs.iter().take(3) {
                println!(
                    "  Funding: {}, Open Interest: {}, Mark Px: {}",
                    ctx.funding, ctx.open_interest, ctx.mark_px
                );
            }
        }
        Err(e) => println!("Error fetching meta_and_asset_ctxs: {}", e),
    }

    // ==================== Phase 2 Info Endpoints ====================

    println!("\n=== Phase 2 Info Endpoints ===");

    // 9. Test portfolio endpoint
    println!("\n=== Testing portfolio endpoint ===");
    match info.portfolio(user).await {
        Ok(portfolio) => {
            println!(
                "Portfolio data retrieved ({} time periods):",
                portfolio.len()
            );
            for (period, data) in portfolio.iter().take(3) {
                println!("  Period: {}", period);
                println!("    Volume: {}", data.vlm);
                println!(
                    "    Account Value History: {} entries",
                    data.account_value_history.len()
                );
                println!("    PnL History: {} entries", data.pnl_history.len());
            }
        }
        Err(e) => println!(
            "Error fetching portfolio (expected if user doesn't exist): {}",
            e
        ),
    }

    // 10. Test user_non_funding_ledger_updates endpoint
    println!("\n=== Testing user_non_funding_ledger_updates endpoint ===");
    let one_week_ago = now - (7 * 24 * 60 * 60 * 1000);
    match info
        .user_non_funding_ledger_updates(user, one_week_ago, Some(now))
        .await
    {
        Ok(updates) => {
            println!("Found {} ledger updates", updates.len());
            for update in updates.iter().take(5) {
                println!("  Update: {:?}", update);
            }
        }
        Err(e) => println!("Error fetching ledger updates: {}", e),
    }

    // 11. Test extra_agents endpoint
    println!("\n=== Testing extra_agents endpoint ===");
    match info.extra_agents(user).await {
        Ok(agents) => {
            println!("Found {} extra agents", agents.len());
            for agent in agents.iter().take(5) {
                println!("  Agent: {:?}", agent);
            }
        }
        Err(e) => println!("Error fetching extra agents: {}", e),
    }

    // 12. Test user_role endpoint
    println!("\n=== Testing user_role endpoint ===");
    match info.user_role(user).await {
        Ok(role) => {
            println!("User role: {:?}", role);
        }
        Err(e) => println!("Error fetching user role: {}", e),
    }

    // 13. Test token_details endpoint
    println!("\n=== Testing token_details endpoint ===");
    match info
        .token_details("0xc1fb593aeffbeb02f85e0308e9956a90")
        .await
    {
        Ok(details) => {
            println!("Token details: {:?}", details);
        }
        Err(e) => println!("Error fetching token details: {}", e),
    }

    // ==================== Phase 3 Info Endpoints ====================

    println!("\n=== Phase 3 Info Endpoints ===");

    // --- Staking/Delegation Queries ---

    // 14. Test delegator_summary endpoint
    println!("\n=== Testing delegator_summary endpoint ===");
    match info.delegator_summary(user).await {
        Ok(summary) => {
            println!("Delegator summary:");
            println!("  Total delegated: {:?}", summary.delegated);
            println!("  Total undelegating: {:?}", summary.undelegating);
            println!("  Total rewards: {:?}", summary.total_rewards);
        }
        Err(e) => println!("Error fetching delegator summary: {}", e),
    }

    // 15. Test delegations endpoint
    println!("\n=== Testing delegations endpoint ===");
    match info.delegations(user).await {
        Ok(delegations) => {
            println!("Found {} delegations", delegations.len());
            for delegation in delegations.iter().take(5) {
                println!(
                    "  Validator: {}, Amount: {}, Locked until: {:?}",
                    delegation.validator, delegation.amount, delegation.locked_until
                );
            }
        }
        Err(e) => println!("Error fetching delegations: {}", e),
    }

    // 16. Test delegator_rewards endpoint
    println!("\n=== Testing delegator_rewards endpoint ===");
    match info.delegator_rewards(user).await {
        Ok(rewards) => {
            println!("Found {} reward entries", rewards.len());
            for reward in rewards.iter().take(5) {
                println!("  Reward: {:?}", reward);
            }
        }
        Err(e) => println!("Error fetching delegator rewards: {}", e),
    }

    // 17. Test delegator_history endpoint
    println!("\n=== Testing delegator_history endpoint ===");
    match info.delegator_history(user).await {
        Ok(history) => {
            println!("Found {} history entries", history.len());
            for entry in history.iter().take(5) {
                println!("  Entry: {:?}", entry);
            }
        }
        Err(e) => println!("Error fetching delegator history: {}", e),
    }

    // --- Deployment Queries ---

    // 18. Test perp_deploy_auction_status endpoint
    println!("\n=== Testing perp_deploy_auction_status endpoint ===");
    match info.perp_deploy_auction_status().await {
        Ok(status) => {
            println!("Perp deploy auction status: {:?}", status);
        }
        Err(e) => println!("Error fetching perp deploy auction status: {}", e),
    }

    // 19. Test spot_deploy_state endpoint
    println!("\n=== Testing spot_deploy_state endpoint ===");
    match info.spot_deploy_state(user).await {
        Ok(state) => {
            println!("Spot deploy state: {:?}", state);
        }
        Err(e) => println!("Error fetching spot deploy state: {}", e),
    }

    // 20. Test spot_pair_deploy_auction_status endpoint
    println!("\n=== Testing spot_pair_deploy_auction_status endpoint ===");
    match info.spot_pair_deploy_auction_status("PURR", "USDC").await {
        Ok(status) => {
            println!("Spot pair deploy auction status: {:?}", status);
        }
        Err(e) => println!("Error fetching spot pair deploy auction status: {}", e),
    }

    // --- Other Queries ---

    // 21. Test perp_dexs endpoint
    println!("\n=== Testing perp_dexs endpoint ===");
    match info.perp_dexs().await {
        Ok(dexs) => {
            println!("Found {} perp DEXs", dexs.len());
            for dex in dexs.iter().take(5) {
                println!("  DEX: {:?}", dex);
            }
        }
        Err(e) => println!("Error fetching perp DEXs: {}", e),
    }

    // 22. Test user_dex_abstraction endpoint
    println!("\n=== Testing user_dex_abstraction endpoint ===");
    match info.user_dex_abstraction(user).await {
        Ok(abstraction) => {
            println!("User DEX abstraction: {:?}", abstraction);
        }
        Err(e) => println!("Error fetching user DEX abstraction: {}", e),
    }

    // 23. Test user_to_multi_sig_signers endpoint
    println!("\n=== Testing user_to_multi_sig_signers endpoint ===");
    match info.user_to_multi_sig_signers(user).await {
        Ok(signers) => {
            println!("Multi-sig signers: {:?}", signers);
        }
        Err(e) => println!("Error fetching multi-sig signers: {}", e),
    }

    // 24. Test user_twap_slice_fills endpoint
    println!("\n=== Testing user_twap_slice_fills endpoint ===");
    match info.user_twap_slice_fills(user).await {
        Ok(fills) => {
            println!("Found {} TWAP slice fills", fills.len());
            for fill in fills.iter().take(5) {
                println!("  Fill: {:?}", fill);
            }
        }
        Err(e) => println!("Error fetching TWAP slice fills: {}", e),
    }

    println!("\n=== All tests completed ===");
    Ok(())
}
