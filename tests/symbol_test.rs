//! Tests for Symbol type operations
//!
//! Tests cover:
//! - Symbol creation (static and owned)
//! - Type detection (is_perp, is_spot)
//! - Conversions and equality
//! - Serialization/deserialization

#[cfg(test)]
mod tests {
    use hyperliquid_rust_sdk::types::Symbol;

    // ==================== Creation Tests ====================

    #[test]
    fn test_symbol_from_static_str() {
        let sym = Symbol::from_static("BTC");
        assert_eq!(sym.as_str(), "BTC");
    }

    #[test]
    fn test_symbol_from_string() {
        let sym = Symbol::from(String::from("ETH"));
        assert_eq!(sym.as_str(), "ETH");
    }

    #[test]
    fn test_symbol_from_str_ref() {
        let sym: Symbol = "SOL".into();
        assert_eq!(sym.as_str(), "SOL");
    }

    #[test]
    fn test_symbol_from_string_ref() {
        let s = String::from("DOGE");
        let sym: Symbol = (&s).into();
        assert_eq!(sym.as_str(), "DOGE");
    }

    #[test]
    fn test_symbol_clone_from_ref() {
        let sym1 = Symbol::from_static("BTC");
        let sym2: Symbol = (&sym1).into();
        assert_eq!(sym1, sym2);
    }

    // ==================== Type Detection Tests ====================

    #[test]
    fn test_perp_symbol_detection() {
        let btc = Symbol::from_static("BTC");
        assert!(btc.is_perp());
        assert!(!btc.is_spot());

        let eth = Symbol::from_static("ETH");
        assert!(eth.is_perp());
        assert!(!eth.is_spot());
    }

    #[test]
    fn test_spot_symbol_detection() {
        // Spot symbols start with @
        let spot = Symbol::from_static("@107");
        assert!(spot.is_spot());
        assert!(!spot.is_perp());

        let hype_usdc = Symbol::from_static("@1");
        assert!(hype_usdc.is_spot());
        assert!(!hype_usdc.is_perp());
    }

    #[test]
    fn test_spot_with_longer_id() {
        let spot = Symbol::from_static("@12345");
        assert!(spot.is_spot());
        assert!(!spot.is_perp());
    }

    #[test]
    fn test_symbol_with_at_in_middle_is_perp() {
        // Symbol with @ not at start should be perp (edge case)
        let weird = Symbol::from_static("BTC@PERP");
        assert!(weird.is_perp());
        assert!(!weird.is_spot());
    }

    // ==================== Equality Tests ====================

    #[test]
    fn test_static_and_owned_equality() {
        let static_sym = Symbol::from_static("BTC");
        let owned_sym = Symbol::from(String::from("BTC"));
        assert_eq!(static_sym, owned_sym);
    }

    #[test]
    fn test_different_symbols_not_equal() {
        let btc = Symbol::from_static("BTC");
        let eth = Symbol::from_static("ETH");
        assert_ne!(btc, eth);
    }

    #[test]
    fn test_symbol_case_sensitivity() {
        let upper = Symbol::from_static("BTC");
        let lower = Symbol::from(String::from("btc"));
        assert_ne!(upper, lower);
    }

    // ==================== Display and AsRef Tests ====================

    #[test]
    fn test_symbol_display() {
        let sym = Symbol::from_static("BTC");
        assert_eq!(format!("{}", sym), "BTC");
    }

    #[test]
    fn test_symbol_as_ref() {
        let sym = Symbol::from_static("ETH");
        let s: &str = sym.as_ref();
        assert_eq!(s, "ETH");
    }

    // ==================== Serialization Tests ====================

    #[test]
    fn test_symbol_serialization() {
        let sym = Symbol::from_static("BTC");
        let json = serde_json::to_string(&sym).unwrap();
        assert_eq!(json, "\"BTC\"");
    }

    #[test]
    fn test_symbol_deserialization() {
        let sym: Symbol = serde_json::from_str("\"ETH\"").unwrap();
        assert_eq!(sym.as_str(), "ETH");
    }

    #[test]
    fn test_symbol_round_trip() {
        let original = Symbol::from_static("SOL");
        let json = serde_json::to_string(&original).unwrap();
        let restored: Symbol = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn test_spot_symbol_serialization() {
        let spot = Symbol::from_static("@107");
        let json = serde_json::to_string(&spot).unwrap();
        assert_eq!(json, "\"@107\"");

        let restored: Symbol = serde_json::from_str(&json).unwrap();
        assert!(restored.is_spot());
    }

    // ==================== Hash Tests ====================

    #[test]
    fn test_symbol_hashable() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Symbol::from_static("BTC"));
        set.insert(Symbol::from(String::from("BTC"))); // Same symbol, different creation
        set.insert(Symbol::from_static("ETH"));

        // Should have 2 unique symbols (BTC and ETH)
        assert_eq!(set.len(), 2);
        assert!(set.contains(&Symbol::from_static("BTC")));
        assert!(set.contains(&Symbol::from_static("ETH")));
    }

    #[test]
    fn test_symbol_in_hashmap() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(Symbol::from_static("BTC"), 50000.0);
        map.insert(Symbol::from_static("ETH"), 3000.0);

        assert_eq!(map.get(&Symbol::from_static("BTC")), Some(&50000.0));
        assert_eq!(map.get(&Symbol::from(String::from("ETH"))), Some(&3000.0));
    }

    // ==================== Common Symbol Tests ====================

    #[test]
    fn test_common_perp_symbols() {
        let symbols = vec!["BTC", "ETH", "SOL", "DOGE", "AVAX", "MATIC", "LINK"];

        for name in symbols {
            let sym = Symbol::from_static(name);
            assert!(sym.is_perp(), "{} should be perp", name);
            assert!(!sym.is_spot(), "{} should not be spot", name);
        }
    }

    #[test]
    fn test_spot_symbol_variations() {
        // Test various spot symbol formats
        let spots = vec!["@1", "@10", "@100", "@1000", "@107"];

        for name in spots {
            let sym = Symbol::from_static(name);
            assert!(sym.is_spot(), "{} should be spot", name);
            assert!(!sym.is_perp(), "{} should not be perp", name);
        }
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_empty_symbol() {
        let sym = Symbol::from(String::new());
        assert_eq!(sym.as_str(), "");
        assert!(sym.is_perp()); // Empty doesn't start with @
    }

    #[test]
    fn test_single_at_symbol() {
        let sym = Symbol::from_static("@");
        assert!(sym.is_spot());
    }

    #[test]
    fn test_whitespace_symbol() {
        let sym = Symbol::from(String::from(" BTC "));
        assert_eq!(sym.as_str(), " BTC ");
        assert!(sym.is_perp()); // Doesn't start with @
    }
}
