//! Tests for NonceManager - nonce management for Hyperliquid's sliding window system
//!
//! Tests cover:
//! - Nonce generation and uniqueness
//! - Address isolation vs global counter
//! - Counter reset functionality
//! - Nonce validity time bounds
//! - Concurrent access safety
//! - Counter monitoring

#[cfg(test)]
mod tests {
    use alloy::primitives::Address;
    use hyperliquid_rust_sdk::providers::nonce::NonceManager;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};

    // ==================== Basic Creation Tests ====================

    #[test]
    fn test_nonce_manager_new_without_isolation() {
        let manager = NonceManager::new(false);
        // Should be created successfully
        assert_eq!(manager.get_counter(None), 0);
    }

    #[test]
    fn test_nonce_manager_new_with_isolation() {
        let manager = NonceManager::new(true);
        let addr = Address::new([1u8; 20]);
        // New address should have counter 0
        assert_eq!(manager.get_counter(Some(addr)), 0);
    }

    // ==================== Nonce Generation Tests ====================

    #[test]
    fn test_nonce_is_time_based() {
        let manager = NonceManager::new(false);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis() as u64;

        let nonce = manager.next_nonce(None);

        // Nonce should be close to current time (within a second)
        let diff = nonce.abs_diff(now);
        assert!(diff < 1000, "Nonce should be within 1 second of now");
    }

    #[test]
    fn test_nonce_uniqueness_rapid_calls() {
        let manager = NonceManager::new(false);

        let mut nonces = HashSet::new();

        // Generate 100 nonces rapidly
        for _ in 0..100 {
            let nonce = manager.next_nonce(None);
            assert!(nonces.insert(nonce), "Nonce should be unique");
        }

        assert_eq!(nonces.len(), 100);
    }

    #[test]
    fn test_nonce_monotonically_increasing() {
        let manager = NonceManager::new(false);

        let mut prev = manager.next_nonce(None);
        for _ in 0..50 {
            let current = manager.next_nonce(None);
            assert!(current > prev, "Nonces should be monotonically increasing");
            prev = current;
        }
    }

    // ==================== Address Isolation Tests ====================

    #[test]
    fn test_address_isolation_enabled() {
        let manager = NonceManager::new(true);
        let addr1 = Address::new([1u8; 20]);
        let addr2 = Address::new([2u8; 20]);

        // Generate nonces for addr1
        let _ = manager.next_nonce(Some(addr1));
        let _ = manager.next_nonce(Some(addr1));

        // addr1 should have counter 2
        assert_eq!(manager.get_counter(Some(addr1)), 2);

        // addr2 should still have counter 0 (no nonces generated)
        assert_eq!(manager.get_counter(Some(addr2)), 0);
    }

    #[test]
    fn test_address_isolation_disabled() {
        let manager = NonceManager::new(false);
        let addr1 = Address::new([1u8; 20]);
        let addr2 = Address::new([2u8; 20]);

        // Generate nonces for addr1
        let _ = manager.next_nonce(Some(addr1));
        let _ = manager.next_nonce(Some(addr1));

        // With isolation disabled, should use global counter
        // When checking addr1/addr2 with isolation disabled, it returns global counter
        assert_eq!(manager.get_counter(Some(addr1)), 2);
        assert_eq!(manager.get_counter(Some(addr2)), 2);
        assert_eq!(manager.get_counter(None), 2);
    }

    #[test]
    fn test_global_counter_without_address() {
        let manager = NonceManager::new(false);

        let _ = manager.next_nonce(None);
        let _ = manager.next_nonce(None);
        let _ = manager.next_nonce(None);

        assert_eq!(manager.get_counter(None), 3);
    }

    #[test]
    fn test_isolation_with_none_address() {
        let manager = NonceManager::new(true);

        // With isolation enabled but None address, should use global counter
        let _ = manager.next_nonce(None);
        let _ = manager.next_nonce(None);

        assert_eq!(manager.get_counter(None), 2);
    }

    // ==================== Counter Reset Tests ====================

    #[test]
    fn test_reset_address_counter() {
        let manager = NonceManager::new(true);
        let addr = Address::new([1u8; 20]);

        // Generate some nonces
        let _ = manager.next_nonce(Some(addr));
        let _ = manager.next_nonce(Some(addr));
        let _ = manager.next_nonce(Some(addr));

        assert_eq!(manager.get_counter(Some(addr)), 3);

        // Reset
        manager.reset_address(addr);

        assert_eq!(manager.get_counter(Some(addr)), 0);
    }

    #[test]
    fn test_reset_nonexistent_address() {
        let manager = NonceManager::new(true);
        let addr = Address::new([99u8; 20]);

        // Resetting an address that was never used should not panic
        manager.reset_address(addr);

        assert_eq!(manager.get_counter(Some(addr)), 0);
    }

    #[test]
    fn test_reset_does_not_affect_other_addresses() {
        let manager = NonceManager::new(true);
        let addr1 = Address::new([1u8; 20]);
        let addr2 = Address::new([2u8; 20]);

        // Generate nonces for both addresses
        let _ = manager.next_nonce(Some(addr1));
        let _ = manager.next_nonce(Some(addr1));
        let _ = manager.next_nonce(Some(addr2));
        let _ = manager.next_nonce(Some(addr2));
        let _ = manager.next_nonce(Some(addr2));

        // Reset only addr1
        manager.reset_address(addr1);

        assert_eq!(manager.get_counter(Some(addr1)), 0);
        assert_eq!(manager.get_counter(Some(addr2)), 3); // Unchanged
    }

    // ==================== Nonce Validity Tests ====================

    #[test]
    fn test_current_time_nonce_is_valid() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis() as u64;

        assert!(NonceManager::is_valid_nonce(now));
    }

    #[test]
    fn test_one_day_ago_nonce_is_valid() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis() as u64;

        let one_day_ms = 24 * 60 * 60 * 1000;
        let one_day_ago = now.saturating_sub(one_day_ms);

        assert!(NonceManager::is_valid_nonce(one_day_ago));
    }

    #[test]
    fn test_almost_two_days_ago_nonce_is_valid() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis() as u64;

        // 47 hours ago (just under 2 days)
        let almost_two_days_ms = 47 * 60 * 60 * 1000;
        let nonce = now.saturating_sub(almost_two_days_ms);

        assert!(NonceManager::is_valid_nonce(nonce));
    }

    #[test]
    fn test_three_days_ago_nonce_is_invalid() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis() as u64;

        let three_days_ms = 3 * 24 * 60 * 60 * 1000;
        let three_days_ago = now.saturating_sub(three_days_ms);

        assert!(!NonceManager::is_valid_nonce(three_days_ago));
    }

    #[test]
    fn test_12_hours_future_nonce_is_valid() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis() as u64;

        let twelve_hours_ms = 12 * 60 * 60 * 1000;
        let future_nonce = now.saturating_add(twelve_hours_ms);

        assert!(NonceManager::is_valid_nonce(future_nonce));
    }

    #[test]
    fn test_two_days_future_nonce_is_invalid() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis() as u64;

        let two_days_ms = 2 * 24 * 60 * 60 * 1000;
        let far_future_nonce = now.saturating_add(two_days_ms);

        assert!(!NonceManager::is_valid_nonce(far_future_nonce));
    }

    #[test]
    fn test_zero_nonce_is_invalid() {
        // Zero is way in the past (1970)
        assert!(!NonceManager::is_valid_nonce(0));
    }

    // ==================== Generated Nonce Validity Tests ====================

    #[test]
    fn test_generated_nonce_is_valid() {
        let manager = NonceManager::new(false);
        let nonce = manager.next_nonce(None);

        assert!(NonceManager::is_valid_nonce(nonce));
    }

    #[test]
    fn test_all_generated_nonces_are_valid() {
        let manager = NonceManager::new(false);

        for _ in 0..100 {
            let nonce = manager.next_nonce(None);
            assert!(
                NonceManager::is_valid_nonce(nonce),
                "Generated nonce should always be valid"
            );
        }
    }

    // ==================== Concurrent Access Tests ====================

    #[tokio::test]
    async fn test_concurrent_nonce_generation() {
        let manager = Arc::new(NonceManager::new(false));
        let mut handles = Vec::new();

        // Spawn 10 tasks each generating 100 nonces
        for _ in 0..10 {
            let manager_clone = manager.clone();
            let handle = tokio::spawn(async move {
                let mut nonces = Vec::new();
                for _ in 0..100 {
                    nonces.push(manager_clone.next_nonce(None));
                }
                nonces
            });
            handles.push(handle);
        }

        // Collect all nonces
        let mut all_nonces = HashSet::new();
        for handle in handles {
            let nonces = handle.await.unwrap();
            for nonce in nonces {
                all_nonces.insert(nonce);
            }
        }

        // All 1000 nonces should be unique
        assert_eq!(all_nonces.len(), 1000);
    }

    #[tokio::test]
    async fn test_concurrent_nonce_generation_with_isolation() {
        let manager = Arc::new(NonceManager::new(true));
        let addr = Address::new([1u8; 20]);
        let mut handles = Vec::new();

        // Spawn 5 tasks each generating 50 nonces for the same address
        for _ in 0..5 {
            let manager_clone = manager.clone();
            let handle = tokio::spawn(async move {
                let mut nonces = Vec::new();
                for _ in 0..50 {
                    nonces.push(manager_clone.next_nonce(Some(addr)));
                }
                nonces
            });
            handles.push(handle);
        }

        // Collect all nonces
        let mut all_nonces = HashSet::new();
        for handle in handles {
            let nonces = handle.await.unwrap();
            for nonce in nonces {
                all_nonces.insert(nonce);
            }
        }

        // All 250 nonces should be unique
        assert_eq!(all_nonces.len(), 250);

        // Counter should be 250
        assert_eq!(manager.get_counter(Some(addr)), 250);
    }

    #[tokio::test]
    async fn test_concurrent_different_addresses() {
        let manager = Arc::new(NonceManager::new(true));
        let mut handles = Vec::new();

        // Spawn tasks for different addresses
        for i in 0..5 {
            let manager_clone = manager.clone();
            let addr = Address::new([i as u8; 20]);
            let handle = tokio::spawn(async move {
                for _ in 0..20 {
                    manager_clone.next_nonce(Some(addr));
                }
                addr
            });
            handles.push(handle);
        }

        // Wait for all to complete
        let mut addresses = Vec::new();
        for handle in handles {
            addresses.push(handle.await.unwrap());
        }

        // Each address should have counter 20
        for addr in addresses {
            assert_eq!(manager.get_counter(Some(addr)), 20);
        }
    }

    // ==================== Debug Trait Test ====================

    #[test]
    fn test_nonce_manager_debug() {
        let manager = NonceManager::new(true);
        let debug_str = format!("{:?}", manager);

        // Should contain expected fields
        assert!(debug_str.contains("NonceManager"));
        assert!(debug_str.contains("isolate_per_address"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_get_counter_for_unused_address_with_isolation() {
        let manager = NonceManager::new(true);
        let unused_addr = Address::new([99u8; 20]);

        // Should return 0 for unused address
        assert_eq!(manager.get_counter(Some(unused_addr)), 0);
    }

    #[test]
    fn test_submillisecond_uniqueness() {
        let manager = NonceManager::new(false);

        // The NonceManager adds counter % 1000 to ensure uniqueness
        // within the same millisecond
        let mut nonces = Vec::new();
        for _ in 0..1000 {
            nonces.push(manager.next_nonce(None));
        }

        // Check all are unique
        let unique: HashSet<_> = nonces.iter().collect();
        assert_eq!(unique.len(), nonces.len());
    }

    #[test]
    fn test_nonce_after_reset_counter_resets() {
        let manager = NonceManager::new(true);
        let addr = Address::new([1u8; 20]);

        // Generate some nonces
        let _ = manager.next_nonce(Some(addr));
        let _ = manager.next_nonce(Some(addr));
        let _ = manager.next_nonce(Some(addr));

        // Counter should be 3
        assert_eq!(manager.get_counter(Some(addr)), 3);

        // Reset
        manager.reset_address(addr);

        // Counter should be 0
        assert_eq!(manager.get_counter(Some(addr)), 0);

        // Generate more nonces
        let _ = manager.next_nonce(Some(addr));
        let _ = manager.next_nonce(Some(addr));

        // Counter should be 2 again
        assert_eq!(manager.get_counter(Some(addr)), 2);
    }

    // ==================== Boundary Time Tests ====================

    #[test]
    fn test_nonce_exactly_at_two_day_boundary_is_invalid() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis() as u64;

        // Exactly 2 days ago should be invalid (> not >=)
        let two_days_ms = 2 * 24 * 60 * 60 * 1000;
        let exactly_two_days_ago = now.saturating_sub(two_days_ms);

        // The check is nonce > (now - 2 days), so exactly 2 days ago is invalid
        assert!(!NonceManager::is_valid_nonce(exactly_two_days_ago));
    }

    #[test]
    fn test_nonce_exactly_at_one_day_future_boundary_is_invalid() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis() as u64;

        // Exactly 1 day in future should be invalid (< not <=)
        let one_day_ms = 24 * 60 * 60 * 1000;
        let exactly_one_day_future = now.saturating_add(one_day_ms);

        // The check is nonce < (now + 1 day), so exactly 1 day future is invalid
        assert!(!NonceManager::is_valid_nonce(exactly_one_day_future));
    }
}
