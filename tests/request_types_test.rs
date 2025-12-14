//! Tests for request types (OrderRequest, CancelRequest, ModifyRequest)
//!
//! Tests cover:
//! - OrderRequest construction (limit and trigger orders)
//! - CancelRequest construction
//! - ModifyRequest construction
//! - Serialization format validation
//! - Builder pattern methods

#[cfg(test)]
mod tests {
    use hyperliquid_sdk_rs::{
        constants::{TIF_ALO, TIF_GTC, TIF_IOC},
        types::requests::{
            BuilderInfo, CancelRequest, CancelRequestCloid, Limit, ModifyRequest,
            OrderRequest, OrderType, Trigger,
        },
    };
    use uuid::Uuid;

    // ==================== OrderRequest Creation Tests ====================

    #[test]
    fn test_limit_order_creation() {
        let order = OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC);

        assert_eq!(order.asset, 0);
        assert!(order.is_buy);
        assert_eq!(order.limit_px, "50000");
        assert_eq!(order.sz, "0.01");
        assert!(!order.reduce_only);
        assert!(order.cloid.is_none());

        match &order.order_type {
            OrderType::Limit(limit) => assert_eq!(limit.tif, TIF_GTC),
            _ => panic!("Expected Limit order type"),
        }
    }

    #[test]
    fn test_limit_order_with_different_tifs() {
        let gtc = OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC);
        let ioc = OrderRequest::limit(0, true, "50000", "0.01", TIF_IOC);
        let alo = OrderRequest::limit(0, true, "50000", "0.01", TIF_ALO);

        match &gtc.order_type {
            OrderType::Limit(l) => assert_eq!(l.tif, "Gtc"),
            _ => panic!("Expected Limit"),
        }
        match &ioc.order_type {
            OrderType::Limit(l) => assert_eq!(l.tif, "Ioc"),
            _ => panic!("Expected Limit"),
        }
        match &alo.order_type {
            OrderType::Limit(l) => assert_eq!(l.tif, "Alo"),
            _ => panic!("Expected Limit"),
        }
    }

    #[test]
    fn test_trigger_order_creation_stop_loss() {
        let order = OrderRequest::trigger(0, false, "48000", "0.01", "sl", true);

        assert_eq!(order.asset, 0);
        assert!(!order.is_buy);
        assert_eq!(order.limit_px, "0"); // Trigger orders don't use limit_px
        assert_eq!(order.sz, "0.01");
        assert!(!order.reduce_only);

        match &order.order_type {
            OrderType::Trigger(trigger) => {
                assert!(trigger.is_market);
                assert_eq!(trigger.trigger_px, "48000");
                assert_eq!(trigger.tpsl, "sl");
            }
            _ => panic!("Expected Trigger order type"),
        }
    }

    #[test]
    fn test_trigger_order_creation_take_profit() {
        let order = OrderRequest::trigger(0, true, "55000", "0.01", "tp", true);

        assert!(order.is_buy);
        match &order.order_type {
            OrderType::Trigger(trigger) => {
                assert_eq!(trigger.tpsl, "tp");
            }
            _ => panic!("Expected Trigger order type"),
        }
    }

    #[test]
    fn test_order_with_cloid() {
        let cloid = Uuid::new_v4();
        let order = OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC)
            .with_cloid(Some(cloid));

        assert!(order.cloid.is_some());
        // Verify cloid is formatted as 32-char hex
        let cloid_str = order.cloid.unwrap();
        assert_eq!(cloid_str.len(), 32);
        assert!(cloid_str.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_order_with_none_cloid() {
        let order =
            OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC).with_cloid(None);
        assert!(order.cloid.is_none());
    }

    #[test]
    fn test_order_reduce_only() {
        let order =
            OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC).reduce_only(true);
        assert!(order.reduce_only);

        let order =
            OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC).reduce_only(false);
        assert!(!order.reduce_only);
    }

    #[test]
    fn test_sell_order() {
        let order = OrderRequest::limit(0, false, "50000", "0.01", TIF_GTC);
        assert!(!order.is_buy);
    }

    #[test]
    fn test_different_assets() {
        let btc = OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC);
        let eth = OrderRequest::limit(1, true, "3000", "0.1", TIF_GTC);

        assert_eq!(btc.asset, 0);
        assert_eq!(eth.asset, 1);
    }

    // ==================== OrderRequest Serialization Tests ====================

    #[test]
    fn test_order_serialization_field_names() {
        let order = OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC);
        let json = serde_json::to_string(&order).unwrap();

        // Check that fields use short names per serde rename
        assert!(json.contains("\"a\":0")); // asset -> a
        assert!(json.contains("\"b\":true")); // is_buy -> b
        assert!(json.contains("\"p\":\"50000\"")); // limit_px -> p
        assert!(json.contains("\"s\":\"0.01\"")); // sz -> s
        assert!(json.contains("\"r\":false")); // reduce_only -> r
    }

    #[test]
    fn test_order_serialization_without_cloid() {
        let order = OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC);
        let json = serde_json::to_string(&order).unwrap();

        // cloid should be omitted when None (skip_serializing_if)
        assert!(!json.contains("\"c\":"));
    }

    #[test]
    fn test_order_serialization_with_cloid() {
        let cloid = Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef1234567890").unwrap();
        let order = OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC)
            .with_cloid(Some(cloid));
        let json = serde_json::to_string(&order).unwrap();

        // cloid should be present
        assert!(json.contains("\"c\":"));
    }

    #[test]
    fn test_order_deserialization() {
        let json = r#"{
            "a": 0,
            "b": true,
            "p": "50000",
            "s": "0.01",
            "r": false,
            "t": {"limit": {"tif": "Gtc"}}
        }"#;

        let order: OrderRequest = serde_json::from_str(json).unwrap();
        assert_eq!(order.asset, 0);
        assert!(order.is_buy);
        assert_eq!(order.limit_px, "50000");
        assert_eq!(order.sz, "0.01");
    }

    // ==================== is_alo Detection Tests ====================

    #[test]
    fn test_is_alo_true() {
        let order = OrderRequest::limit(0, true, "50000", "0.01", TIF_ALO);
        assert!(order.is_alo());
    }

    #[test]
    fn test_is_alo_false_gtc() {
        let order = OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC);
        assert!(!order.is_alo());
    }

    #[test]
    fn test_is_alo_false_trigger() {
        let order = OrderRequest::trigger(0, true, "50000", "0.01", "tp", true);
        assert!(!order.is_alo());
    }

    // ==================== CancelRequest Tests ====================

    #[test]
    fn test_cancel_request_creation() {
        let cancel = CancelRequest::new(0, 12345);

        assert_eq!(cancel.asset, 0);
        assert_eq!(cancel.oid, 12345);
    }

    #[test]
    fn test_cancel_request_serialization() {
        let cancel = CancelRequest::new(0, 12345);
        let json = serde_json::to_string(&cancel).unwrap();

        // Uses short names per serde rename
        assert!(json.contains("\"a\":0"));
        assert!(json.contains("\"o\":12345"));
    }

    // ==================== CancelRequestCloid Tests ====================

    #[test]
    fn test_cancel_request_cloid_creation() {
        let cloid = Uuid::new_v4();
        let cancel = CancelRequestCloid::new(0, cloid);

        assert_eq!(cancel.asset, 0);
        // Verify cloid is formatted as 32-char hex
        assert_eq!(cancel.cloid.len(), 32);
    }

    #[test]
    fn test_cancel_request_cloid_format() {
        let cloid = Uuid::parse_str("a1b2c3d4-e5f6-7890-abcd-ef1234567890").unwrap();
        let cancel = CancelRequestCloid::new(0, cloid);

        // Should be formatted without dashes
        assert!(!cancel.cloid.contains('-'));
        assert_eq!(cancel.cloid.len(), 32);
    }

    // ==================== ModifyRequest Tests ====================

    #[test]
    fn test_modify_request_creation() {
        let order = OrderRequest::limit(0, true, "51000", "0.02", TIF_GTC);
        let modify = ModifyRequest { oid: 12345, order };

        assert_eq!(modify.oid, 12345);
        assert_eq!(modify.order.limit_px, "51000");
        assert_eq!(modify.order.sz, "0.02");
    }

    #[test]
    fn test_modify_request_serialization() {
        let order = OrderRequest::limit(0, true, "51000", "0.02", TIF_GTC);
        let modify = ModifyRequest { oid: 12345, order };
        let json = serde_json::to_string(&modify).unwrap();

        assert!(json.contains("\"oid\":12345"));
        assert!(json.contains("\"order\":"));
    }

    // ==================== BuilderInfo Tests ====================

    #[test]
    fn test_builder_info_creation() {
        let builder = BuilderInfo {
            builder: "0x1234567890123456789012345678901234567890".to_string(),
            fee: 100,
        };

        assert_eq!(
            builder.builder,
            "0x1234567890123456789012345678901234567890"
        );
        assert_eq!(builder.fee, 100);
    }

    #[test]
    fn test_builder_info_serialization() {
        let builder = BuilderInfo {
            builder: "0x1234".to_string(),
            fee: 50,
        };
        let json = serde_json::to_string(&builder).unwrap();

        // Uses short names per serde rename
        assert!(json.contains("\"b\":\"0x1234\""));
        assert!(json.contains("\"f\":50"));
    }

    // ==================== OrderType Tests ====================

    #[test]
    fn test_limit_order_type() {
        let limit = OrderType::Limit(Limit {
            tif: "Gtc".to_string(),
        });

        match limit {
            OrderType::Limit(l) => assert_eq!(l.tif, "Gtc"),
            _ => panic!("Expected Limit"),
        }
    }

    #[test]
    fn test_trigger_order_type() {
        let trigger = OrderType::Trigger(Trigger {
            is_market: true,
            trigger_px: "50000".to_string(),
            tpsl: "sl".to_string(),
        });

        match trigger {
            OrderType::Trigger(t) => {
                assert!(t.is_market);
                assert_eq!(t.trigger_px, "50000");
                assert_eq!(t.tpsl, "sl");
            }
            _ => panic!("Expected Trigger"),
        }
    }

    #[test]
    fn test_trigger_limit_order() {
        // Trigger with is_market = false is a limit trigger
        let trigger = OrderType::Trigger(Trigger {
            is_market: false,
            trigger_px: "50000".to_string(),
            tpsl: "tp".to_string(),
        });

        match trigger {
            OrderType::Trigger(t) => assert!(!t.is_market),
            _ => panic!("Expected Trigger"),
        }
    }

    // ==================== String Input Tests ====================

    #[test]
    fn test_order_with_string_inputs() {
        let order = OrderRequest::limit(
            0,
            true,
            String::from("50000.50"),
            String::from("0.015"),
            String::from("Gtc"),
        );

        assert_eq!(order.limit_px, "50000.50");
        assert_eq!(order.sz, "0.015");
    }

    #[test]
    fn test_order_with_str_inputs() {
        let order = OrderRequest::limit(0, true, "50000", "0.01", "Ioc");

        assert_eq!(order.limit_px, "50000");
        match &order.order_type {
            OrderType::Limit(l) => assert_eq!(l.tif, "Ioc"),
            _ => panic!("Expected Limit"),
        }
    }

    // ==================== Chained Builder Tests ====================

    #[test]
    fn test_chained_builders() {
        let cloid = Uuid::new_v4();
        let order = OrderRequest::limit(0, true, "50000", "0.01", TIF_GTC)
            .reduce_only(true)
            .with_cloid(Some(cloid));

        assert!(order.reduce_only);
        assert!(order.cloid.is_some());
    }
}
