//! Tests for OrderBuilder fluent API
//!
//! Tests cover:
//! - Builder pattern methods (buy, sell, limit_px, size, etc.)
//! - Convenience methods (limit_buy, limit_sell, trigger_buy, trigger_sell)
//! - Build validation and error handling
//! - Order type defaults

#[cfg(test)]
mod tests {
    use alloy::signers::local::PrivateKeySigner;
    use hyperliquid_sdk_rs::{
        constants::{TIF_GTC, TIF_IOC},
        signers::AlloySigner,
        types::requests::{Limit, OrderType, Trigger},
        ExchangeProvider,
    };
    use std::sync::Once;
    use uuid::Uuid;

    static INIT: Once = Once::new();

    fn init_crypto() {
        INIT.call_once(|| {
            rustls::crypto::CryptoProvider::install_default(
                rustls::crypto::aws_lc_rs::default_provider(),
            )
            .expect("Failed to install rustls crypto provider");
        });
    }

    fn create_test_exchange() -> ExchangeProvider<AlloySigner<PrivateKeySigner>> {
        init_crypto();
        let private_key =
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let signer = private_key.parse::<PrivateKeySigner>().unwrap();
        let alloy_signer = AlloySigner { inner: signer };

        ExchangeProvider::testnet(alloy_signer)
    }

    // ==================== Basic Builder Tests ====================

    #[test]
    fn test_builder_buy_direction() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .build()
            .unwrap();

        assert!(order.is_buy);
    }

    #[test]
    fn test_builder_sell_direction() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .sell()
            .limit_px("50000")
            .size("0.01")
            .build()
            .unwrap();

        assert!(!order.is_buy);
    }

    #[test]
    fn test_builder_asset() {
        let exchange = create_test_exchange();

        let btc_order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .build()
            .unwrap();
        assert_eq!(btc_order.asset, 0);

        let eth_order = exchange
            .order(1)
            .buy()
            .limit_px("3000")
            .size("0.1")
            .build()
            .unwrap();
        assert_eq!(eth_order.asset, 1);
    }

    #[test]
    fn test_builder_price_and_size() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000.50")
            .size("0.015")
            .build()
            .unwrap();

        // Builder formats prices, so check they're parsed correctly
        assert!(!order.limit_px.is_empty());
        assert!(!order.sz.is_empty());
    }

    #[test]
    fn test_builder_reduce_only() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .reduce_only(true)
            .build()
            .unwrap();

        assert!(order.reduce_only);
    }

    #[test]
    fn test_builder_reduce_only_false() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .reduce_only(false)
            .build()
            .unwrap();

        assert!(!order.reduce_only);
    }

    #[test]
    fn test_builder_default_reduce_only() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .build()
            .unwrap();

        assert!(!order.reduce_only); // Default is false
    }

    // ==================== Order Type Tests ====================

    #[test]
    fn test_builder_default_order_type() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .build()
            .unwrap();

        // Default is GTC limit
        match &order.order_type {
            OrderType::Limit(limit) => assert_eq!(limit.tif, TIF_GTC),
            _ => panic!("Expected Limit order type"),
        }
    }

    #[test]
    fn test_builder_custom_order_type() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .order_type(OrderType::Limit(Limit {
                tif: TIF_IOC.to_string(),
            }))
            .build()
            .unwrap();

        match &order.order_type {
            OrderType::Limit(limit) => assert_eq!(limit.tif, TIF_IOC),
            _ => panic!("Expected Limit order type"),
        }
    }

    #[test]
    fn test_builder_trigger_order_type() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .order_type(OrderType::Trigger(Trigger {
                is_market: true,
                trigger_px: "48000".to_string(),
                tpsl: "sl".to_string(),
            }))
            .build()
            .unwrap();

        match &order.order_type {
            OrderType::Trigger(trigger) => {
                assert!(trigger.is_market);
                assert_eq!(trigger.trigger_px, "48000");
                assert_eq!(trigger.tpsl, "sl");
            }
            _ => panic!("Expected Trigger order type"),
        }
    }

    // ==================== CLOID Tests ====================

    #[test]
    fn test_builder_with_cloid() {
        let exchange = create_test_exchange();
        let cloid = Uuid::new_v4();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .cloid(cloid)
            .build()
            .unwrap();

        assert!(order.cloid.is_some());
        let cloid_str = order.cloid.unwrap();
        assert_eq!(cloid_str.len(), 32); // 128-bit UUID as hex
    }

    #[test]
    fn test_builder_without_cloid() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .build()
            .unwrap();

        assert!(order.cloid.is_none());
    }

    // ==================== Convenience Method Tests ====================

    #[test]
    fn test_limit_buy_convenience() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .limit_buy("50000", "0.01")
            .build()
            .unwrap();

        assert!(order.is_buy);
        assert!(!order.limit_px.is_empty());
        assert!(!order.sz.is_empty());
    }

    #[test]
    fn test_limit_sell_convenience() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .limit_sell("50000", "0.01")
            .build()
            .unwrap();

        assert!(!order.is_buy);
    }

    #[test]
    fn test_trigger_buy_convenience() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .trigger_buy("48000", "0.01", "sl")
            .build()
            .unwrap();

        assert!(order.is_buy);
        match &order.order_type {
            OrderType::Trigger(trigger) => {
                assert!(trigger.is_market);
                assert_eq!(trigger.tpsl, "sl");
            }
            _ => panic!("Expected Trigger order type"),
        }
    }

    #[test]
    fn test_trigger_sell_convenience() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .trigger_sell("55000", "0.01", "tp")
            .build()
            .unwrap();

        assert!(!order.is_buy);
        match &order.order_type {
            OrderType::Trigger(trigger) => {
                assert!(trigger.is_market);
                assert_eq!(trigger.tpsl, "tp");
            }
            _ => panic!("Expected Trigger order type"),
        }
    }

    // ==================== Validation Tests ====================

    #[test]
    fn test_builder_missing_direction_fails() {
        let exchange = create_test_exchange();
        let result = exchange.order(0).limit_px("50000").size("0.01").build();

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_missing_price_fails() {
        let exchange = create_test_exchange();
        let result = exchange.order(0).buy().size("0.01").build();

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_missing_size_fails() {
        let exchange = create_test_exchange();
        let result = exchange.order(0).buy().limit_px("50000").build();

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_invalid_price_fails() {
        let exchange = create_test_exchange();
        let result = exchange
            .order(0)
            .buy()
            .limit_px("not_a_number")
            .size("0.01")
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_invalid_size_fails() {
        let exchange = create_test_exchange();
        let result = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("invalid")
            .build();

        assert!(result.is_err());
    }

    // ==================== Price Formatting Tests ====================

    #[test]
    fn test_builder_formats_trailing_zeros() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000.00000000")
            .size("0.01000000")
            .build()
            .unwrap();

        // Should strip trailing zeros
        assert!(!order.limit_px.ends_with('0') || order.limit_px == "50000");
        assert!(!order.sz.ends_with('0') || order.sz == "0.01" || order.sz == "0");
    }

    #[test]
    fn test_builder_integer_input() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px(50000)
            .size(1)
            .build()
            .unwrap();

        // Should accept integer inputs via ToString
        assert!(!order.limit_px.is_empty());
        assert!(!order.sz.is_empty());
    }

    #[test]
    fn test_builder_float_input() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px(50000.5)
            .size(0.015)
            .build()
            .unwrap();

        assert!(!order.limit_px.is_empty());
        assert!(!order.sz.is_empty());
    }

    // ==================== Chain Tests ====================

    #[test]
    fn test_builder_full_chain() {
        let exchange = create_test_exchange();
        let cloid = Uuid::new_v4();

        let order = exchange
            .order(0)
            .buy()
            .limit_px("50000")
            .size("0.01")
            .reduce_only(true)
            .order_type(OrderType::Limit(Limit {
                tif: TIF_IOC.to_string(),
            }))
            .cloid(cloid)
            .build()
            .unwrap();

        assert!(order.is_buy);
        assert!(order.reduce_only);
        assert!(order.cloid.is_some());
        match &order.order_type {
            OrderType::Limit(limit) => assert_eq!(limit.tif, TIF_IOC),
            _ => panic!("Expected Limit"),
        }
    }

    #[test]
    fn test_builder_direction_override() {
        let exchange = create_test_exchange();

        // If you call buy() then sell(), sell() should win
        let order = exchange
            .order(0)
            .buy()
            .sell()
            .limit_px("50000")
            .size("0.01")
            .build()
            .unwrap();

        assert!(!order.is_buy);
    }

    // ==================== Different Input Types ====================

    #[test]
    fn test_builder_string_inputs() {
        let exchange = create_test_exchange();
        let order = exchange
            .order(0)
            .buy()
            .limit_px(String::from("50000"))
            .size(String::from("0.01"))
            .build()
            .unwrap();

        assert!(!order.limit_px.is_empty());
    }

    #[test]
    fn test_builder_str_ref_inputs() {
        let exchange = create_test_exchange();
        let price = "50000";
        let size = "0.01";

        let order = exchange
            .order(0)
            .buy()
            .limit_px(price)
            .size(size)
            .build()
            .unwrap();

        assert!(!order.limit_px.is_empty());
    }
}
