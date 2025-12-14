//! Tests for WebSocket types (subscriptions and messages)
//!
//! Tests cover:
//! - Subscription serialization
//! - Message deserialization
//! - Data structure validation

#[cfg(test)]
mod tests {
    use alloy::primitives::address;
    use hyperliquid_sdk_rs::types::ws::{
        AllMids, AllMidsData, BookLevel, Candle, CandleData, L2Book, L2BookData, Message,
        Subscription, Trade, Trades,
    };
    use std::collections::HashMap;

    // ==================== Subscription Serialization Tests ====================

    #[test]
    fn test_subscription_all_mids() {
        let sub = Subscription::AllMids;
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"allMids\""));
    }

    #[test]
    fn test_subscription_l2_book() {
        let sub = Subscription::L2Book {
            coin: "BTC".to_string(),
        };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"l2Book\""));
        assert!(json.contains("\"coin\":\"BTC\""));
    }

    #[test]
    fn test_subscription_trades() {
        let sub = Subscription::Trades {
            coin: "ETH".to_string(),
        };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"trades\""));
        assert!(json.contains("\"coin\":\"ETH\""));
    }

    #[test]
    fn test_subscription_candle() {
        let sub = Subscription::Candle {
            coin: "BTC".to_string(),
            interval: "1h".to_string(),
        };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"candle\""));
        assert!(json.contains("\"coin\":\"BTC\""));
        assert!(json.contains("\"interval\":\"1h\""));
    }

    #[test]
    fn test_subscription_bbo() {
        let sub = Subscription::Bbo {
            coin: "SOL".to_string(),
        };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"bbo\""));
        assert!(json.contains("\"coin\":\"SOL\""));
    }

    #[test]
    fn test_subscription_order_updates() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::OrderUpdates { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"orderUpdates\""));
        assert!(json.contains("\"user\":"));
    }

    #[test]
    fn test_subscription_user_fills() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::UserFills { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"userFills\""));
    }

    #[test]
    fn test_subscription_user_fundings() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::UserFundings { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"userFundings\""));
    }

    #[test]
    fn test_subscription_open_orders() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::OpenOrders { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"openOrders\""));
    }

    #[test]
    fn test_subscription_clearinghouse_state() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::ClearinghouseState { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"clearinghouseState\""));
    }

    #[test]
    fn test_subscription_notification() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::Notification { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"notification\""));
    }

    #[test]
    fn test_subscription_web_data2() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::WebData2 { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"webData2\""));
    }

    // ==================== Phase 2 Subscription Tests ====================

    #[test]
    fn test_subscription_web_data3() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::WebData3 { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"webData3\""));
    }

    #[test]
    fn test_subscription_twap_states() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::TwapStates { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"twapStates\""));
    }

    #[test]
    fn test_subscription_active_asset_ctx() {
        let sub = Subscription::ActiveAssetCtx {
            coin: "BTC".to_string(),
        };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"activeAssetCtx\""));
    }

    #[test]
    fn test_subscription_active_asset_data() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::ActiveAssetData {
            user: addr,
            coin: "ETH".to_string(),
        };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"activeAssetData\""));
        assert!(json.contains("\"coin\":\"ETH\""));
    }

    #[test]
    fn test_subscription_user_twap_slice_fills() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::UserTwapSliceFills { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"userTwapSliceFills\""));
    }

    #[test]
    fn test_subscription_user_twap_history() {
        let addr = address!("1234567890123456789012345678901234567890");
        let sub = Subscription::UserTwapHistory { user: addr };
        let json = serde_json::to_string(&sub).unwrap();

        assert!(json.contains("\"type\":\"userTwapHistory\""));
    }

    // ==================== Message Deserialization Tests ====================

    #[test]
    fn test_message_all_mids_deserialization() {
        let json = r#"{
            "channel": "allMids",
            "data": {
                "mids": {
                    "BTC": "50000.5",
                    "ETH": "3000.25"
                }
            }
        }"#;

        let msg: Message = serde_json::from_str(json).unwrap();

        match msg {
            Message::AllMids(all_mids) => {
                assert_eq!(all_mids.data.mids.get("BTC").unwrap(), "50000.5");
                assert_eq!(all_mids.data.mids.get("ETH").unwrap(), "3000.25");
            }
            _ => panic!("Expected AllMids message"),
        }
    }

    #[test]
    fn test_message_trades_deserialization() {
        let json = r#"{
            "channel": "trades",
            "data": [{
                "coin": "BTC",
                "side": "B",
                "px": "50000",
                "sz": "0.01",
                "time": 1690393044548,
                "hash": "0xabc123",
                "tid": 12345
            }]
        }"#;

        let msg: Message = serde_json::from_str(json).unwrap();

        match msg {
            Message::Trades(trades) => {
                assert_eq!(trades.data.len(), 1);
                assert_eq!(trades.data[0].coin, "BTC");
                assert_eq!(trades.data[0].side, "B");
                assert_eq!(trades.data[0].px, "50000");
                assert_eq!(trades.data[0].sz, "0.01");
            }
            _ => panic!("Expected Trades message"),
        }
    }

    #[test]
    fn test_message_l2_book_deserialization() {
        let json = r#"{
            "channel": "l2Book",
            "data": {
                "coin": "BTC",
                "time": 1690393044548,
                "levels": [
                    [{"px": "50000", "sz": "1.0", "n": 5}],
                    [{"px": "50010", "sz": "0.5", "n": 3}]
                ]
            }
        }"#;

        let msg: Message = serde_json::from_str(json).unwrap();

        match msg {
            Message::L2Book(book) => {
                assert_eq!(book.data.coin, "BTC");
                assert_eq!(book.data.levels.len(), 2);
                assert_eq!(book.data.levels[0][0].px, "50000");
                assert_eq!(book.data.levels[0][0].sz, "1.0");
                assert_eq!(book.data.levels[0][0].n, 5);
            }
            _ => panic!("Expected L2Book message"),
        }
    }

    #[test]
    fn test_message_candle_deserialization() {
        let json = r#"{
            "channel": "candle",
            "data": {
                "T": 1690393044548,
                "c": "50100",
                "h": "50500",
                "i": "1h",
                "l": "49500",
                "n": 1000,
                "o": "50000",
                "s": "BTC",
                "t": 1690389444548,
                "v": "100.5"
            }
        }"#;

        let msg: Message = serde_json::from_str(json).unwrap();

        match msg {
            Message::Candle(candle) => {
                assert_eq!(candle.data.coin, "BTC");
                assert_eq!(candle.data.open, "50000");
                assert_eq!(candle.data.high, "50500");
                assert_eq!(candle.data.low, "49500");
                assert_eq!(candle.data.close, "50100");
                assert_eq!(candle.data.volume, "100.5");
                assert_eq!(candle.data.num_trades, 1000);
                assert_eq!(candle.data.interval, "1h");
            }
            _ => panic!("Expected Candle message"),
        }
    }

    #[test]
    fn test_message_bbo_deserialization() {
        let json = r#"{
            "channel": "bbo",
            "data": {
                "coin": "BTC",
                "time": 1690393044548,
                "bbo": {
                    "bid": {"px": "49999", "sz": "1.5"},
                    "ask": {"px": "50001", "sz": "2.0"}
                }
            }
        }"#;

        let msg: Message = serde_json::from_str(json).unwrap();

        match msg {
            Message::Bbo(bbo) => {
                assert_eq!(bbo.data.coin, "BTC");
                assert_eq!(bbo.data.bbo.bid.px, "49999");
                assert_eq!(bbo.data.bbo.ask.px, "50001");
            }
            _ => panic!("Expected Bbo message"),
        }
    }

    #[test]
    fn test_message_subscription_response() {
        let json = r#"{"channel": "subscriptionResponse"}"#;

        let msg: Message = serde_json::from_str(json).unwrap();

        match msg {
            Message::SubscriptionResponse => {}
            _ => panic!("Expected SubscriptionResponse"),
        }
    }

    #[test]
    fn test_message_pong() {
        let json = r#"{"channel": "pong"}"#;

        let msg: Message = serde_json::from_str(json).unwrap();

        match msg {
            Message::Pong => {}
            _ => panic!("Expected Pong"),
        }
    }

    // ==================== Data Structure Tests ====================

    #[test]
    fn test_all_mids_data_structure() {
        let mut mids = HashMap::new();
        mids.insert("BTC".to_string(), "50000".to_string());
        mids.insert("ETH".to_string(), "3000".to_string());

        let data = AllMidsData { mids: mids.clone() };
        let all_mids = AllMids { data };

        assert_eq!(all_mids.data.mids.len(), 2);
        assert_eq!(all_mids.data.mids.get("BTC").unwrap(), "50000");
    }

    #[test]
    fn test_trade_data_structure() {
        let trade = Trade {
            coin: "BTC".to_string(),
            side: "B".to_string(),
            px: "50000".to_string(),
            sz: "0.01".to_string(),
            time: 1690393044548,
            hash: "0xabc".to_string(),
            tid: 12345,
        };

        assert_eq!(trade.coin, "BTC");
        assert_eq!(trade.side, "B");
        assert_eq!(trade.tid, 12345);
    }

    #[test]
    fn test_trades_multiple() {
        let trades = Trades {
            data: vec![
                Trade {
                    coin: "BTC".to_string(),
                    side: "B".to_string(),
                    px: "50000".to_string(),
                    sz: "0.01".to_string(),
                    time: 1690393044548,
                    hash: "0xabc".to_string(),
                    tid: 12345,
                },
                Trade {
                    coin: "BTC".to_string(),
                    side: "A".to_string(),
                    px: "50010".to_string(),
                    sz: "0.02".to_string(),
                    time: 1690393044549,
                    hash: "0xdef".to_string(),
                    tid: 12346,
                },
            ],
        };

        assert_eq!(trades.data.len(), 2);
        assert_eq!(trades.data[0].side, "B");
        assert_eq!(trades.data[1].side, "A");
    }

    #[test]
    fn test_book_level_structure() {
        let level = BookLevel {
            px: "50000".to_string(),
            sz: "1.5".to_string(),
            n: 10,
        };

        assert_eq!(level.px, "50000");
        assert_eq!(level.sz, "1.5");
        assert_eq!(level.n, 10);
    }

    #[test]
    fn test_l2_book_structure() {
        let book = L2Book {
            data: L2BookData {
                coin: "BTC".to_string(),
                time: 1690393044548,
                levels: vec![
                    vec![BookLevel {
                        px: "49999".to_string(),
                        sz: "1.0".to_string(),
                        n: 5,
                    }],
                    vec![BookLevel {
                        px: "50001".to_string(),
                        sz: "0.8".to_string(),
                        n: 3,
                    }],
                ],
            },
        };

        assert_eq!(book.data.coin, "BTC");
        assert_eq!(book.data.levels.len(), 2);
    }

    #[test]
    fn test_candle_data_structure() {
        let candle = Candle {
            data: CandleData {
                time_close: 1690393044548,
                close: "50100".to_string(),
                high: "50500".to_string(),
                interval: "1h".to_string(),
                low: "49500".to_string(),
                num_trades: 1000,
                open: "50000".to_string(),
                coin: "BTC".to_string(),
                time_open: 1690389444548,
                volume: "100.5".to_string(),
            },
        };

        // Verify OHLCV data
        assert_eq!(candle.data.open, "50000");
        assert_eq!(candle.data.high, "50500");
        assert_eq!(candle.data.low, "49500");
        assert_eq!(candle.data.close, "50100");
        assert_eq!(candle.data.volume, "100.5");
    }

    // ==================== Subscription Round-Trip Tests ====================

    #[test]
    fn test_subscription_round_trip_l2_book() {
        let original = Subscription::L2Book {
            coin: "BTC".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let restored: Subscription = serde_json::from_str(&json).unwrap();

        match restored {
            Subscription::L2Book { coin } => assert_eq!(coin, "BTC"),
            _ => panic!("Round-trip failed"),
        }
    }

    #[test]
    fn test_subscription_round_trip_candle() {
        let original = Subscription::Candle {
            coin: "ETH".to_string(),
            interval: "15m".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let restored: Subscription = serde_json::from_str(&json).unwrap();

        match restored {
            Subscription::Candle { coin, interval } => {
                assert_eq!(coin, "ETH");
                assert_eq!(interval, "15m");
            }
            _ => panic!("Round-trip failed"),
        }
    }

    #[test]
    fn test_subscription_round_trip_user_fills() {
        let addr = address!("1234567890123456789012345678901234567890");
        let original = Subscription::UserFills { user: addr };

        let json = serde_json::to_string(&original).unwrap();
        let restored: Subscription = serde_json::from_str(&json).unwrap();

        match restored {
            Subscription::UserFills { user } => assert_eq!(user, addr),
            _ => panic!("Round-trip failed"),
        }
    }
}
