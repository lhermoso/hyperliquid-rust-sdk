//! Example showing how to use the info types with minimal implementation
//! This is just to demonstrate the flat type structure

use std::collections::HashMap;

use hyperliquid_sdk_rs::types::{L2SnapshotResponse, UserStateResponse};

fn main() {
    // Example of deserializing API responses into our flat types

    // All mids response - returns HashMap directly
    let mids_json = r#"{"BTC": "45000.0", "ETH": "3000.0"}"#;
    let mids: HashMap<String, String> = serde_json::from_str(mids_json).unwrap();
    println!("BTC mid price: {:?}", mids.get("BTC"));

    // L2 book response - levels[0] = bids, levels[1] = asks
    let l2_json = r#"{
        "coin": "BTC",
        "levels": [
            [{"n": 1, "px": "44999", "sz": "0.5"}],
            [{"n": 1, "px": "45001", "sz": "0.5"}]
        ],
        "time": 1234567890
    }"#;
    let l2: L2SnapshotResponse = serde_json::from_str(l2_json).unwrap();
    println!("Best bid: {:?}", l2.levels[0].first());
    println!("Best ask: {:?}", l2.levels[1].first());

    // User state - direct field access
    let user_json = r#"{
        "assetPositions": [],
        "crossMarginSummary": {
            "accountValue": "10000.0",
            "totalMarginUsed": "0.0",
            "totalNtlPos": "0.0",
            "totalRawUsd": "10000.0"
        },
        "marginSummary": {
            "accountValue": "10000.0",
            "totalMarginUsed": "0.0",
            "totalNtlPos": "0.0",
            "totalRawUsd": "10000.0"
        },
        "withdrawable": "10000.0"
    }"#;
    let user_state: UserStateResponse = serde_json::from_str(user_json).unwrap();
    println!(
        "Account value: {}",
        user_state.cross_margin_summary.account_value
    );

    // Finding a position - user does it themselves
    let btc_position = user_state
        .asset_positions
        .iter()
        .find(|ap| ap.position.coin == "BTC")
        .map(|ap| &ap.position);
    println!("BTC position: {:?}", btc_position);
}
