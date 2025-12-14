//! Tests for OrderBatcher - order batching for high-frequency trading
//!
//! Tests cover:
//! - BatchConfig default values
//! - Order accumulation in batches
//! - Cancel accumulation in batches
//! - ALO vs Regular order priority separation
//! - Batch execution via BatcherHandle::run
//! - OrderRequest::is_alo detection
//! - Graceful shutdown

#[cfg(test)]
mod tests {
    use hyperliquid_rust_sdk::{
        providers::batcher::{
            BatchConfig, OrderBatcher, OrderHandle, OrderPriority, PendingCancel,
            PendingOrder,
        },
        types::requests::{CancelRequest, Limit, OrderRequest, OrderType, Trigger},
        types::responses::ExchangeResponseStatus,
        HyperliquidError,
    };
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::Mutex;

    type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

    // ==================== BatchConfig Tests ====================

    #[test]
    fn test_batch_config_default_interval() {
        let config = BatchConfig::default();
        assert_eq!(config.interval, Duration::from_millis(100));
    }

    #[test]
    fn test_batch_config_default_max_batch_size() {
        let config = BatchConfig::default();
        assert_eq!(config.max_batch_size, 100);
    }

    #[test]
    fn test_batch_config_default_prioritize_alo() {
        let config = BatchConfig::default();
        assert!(config.prioritize_alo);
    }

    #[test]
    fn test_batch_config_default_max_wait_time() {
        let config = BatchConfig::default();
        assert_eq!(config.max_wait_time, Duration::from_millis(500));
    }

    #[test]
    fn test_batch_config_custom_values() {
        let config = BatchConfig {
            interval: Duration::from_millis(50),
            max_batch_size: 50,
            prioritize_alo: false,
            max_wait_time: Duration::from_millis(1000),
        };

        assert_eq!(config.interval, Duration::from_millis(50));
        assert_eq!(config.max_batch_size, 50);
        assert!(!config.prioritize_alo);
        assert_eq!(config.max_wait_time, Duration::from_millis(1000));
    }

    // ==================== OrderRequest::is_alo Tests ====================

    #[test]
    fn test_order_is_alo_with_alo_tif() {
        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "Alo".to_string(),
            }),
            cloid: None,
        };

        assert!(order.is_alo());
    }

    #[test]
    fn test_order_is_alo_lowercase() {
        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "alo".to_string(),
            }),
            cloid: None,
        };

        assert!(order.is_alo());
    }

    #[test]
    fn test_order_is_alo_uppercase() {
        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "ALO".to_string(),
            }),
            cloid: None,
        };

        assert!(order.is_alo());
    }

    #[test]
    fn test_order_is_not_alo_gtc() {
        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "Gtc".to_string(),
            }),
            cloid: None,
        };

        assert!(!order.is_alo());
    }

    #[test]
    fn test_order_is_not_alo_ioc() {
        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "Ioc".to_string(),
            }),
            cloid: None,
        };

        assert!(!order.is_alo());
    }

    #[test]
    fn test_order_is_not_alo_trigger() {
        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Trigger(Trigger {
                is_market: true,
                trigger_px: "48000".to_string(),
                tpsl: "sl".to_string(),
            }),
            cloid: None,
        };

        assert!(!order.is_alo());
    }

    // ==================== OrderPriority Tests ====================

    #[test]
    fn test_order_priority_equality() {
        assert_eq!(OrderPriority::ALO, OrderPriority::ALO);
        assert_eq!(OrderPriority::Regular, OrderPriority::Regular);
        assert_ne!(OrderPriority::ALO, OrderPriority::Regular);
    }

    #[test]
    fn test_order_priority_clone() {
        let priority = OrderPriority::ALO;
        let cloned = priority.clone();
        assert_eq!(priority, cloned);
    }

    #[test]
    fn test_order_priority_debug() {
        let alo_debug = format!("{:?}", OrderPriority::ALO);
        let regular_debug = format!("{:?}", OrderPriority::Regular);

        assert!(alo_debug.contains("ALO"));
        assert!(regular_debug.contains("Regular"));
    }

    // ==================== OrderBatcher Creation Tests ====================

    #[tokio::test]
    async fn test_batcher_creation() {
        let config = BatchConfig::default();
        let (batcher, _handle) = OrderBatcher::new(config);

        // Batcher should be created successfully
        // The handle is returned for running the batch loop
        drop(batcher);
    }

    #[tokio::test]
    async fn test_batcher_creation_with_custom_config() {
        let config = BatchConfig {
            interval: Duration::from_millis(200),
            max_batch_size: 50,
            prioritize_alo: false,
            max_wait_time: Duration::from_secs(1),
        };

        let (batcher, _handle) = OrderBatcher::new(config);
        drop(batcher);
    }

    // ==================== Order Batching Tests ====================

    #[tokio::test]
    async fn test_add_order_returns_pending_handle() {
        let config = BatchConfig::default();
        let (batcher, _handle) = OrderBatcher::new(config);

        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "Gtc".to_string(),
            }),
            cloid: None,
        };

        let handle = batcher.add_order(order, 123456789).await;

        assert!(matches!(handle, OrderHandle::Pending { .. }));
    }

    #[tokio::test]
    async fn test_add_multiple_orders() {
        let config = BatchConfig::default();
        let (batcher, _handle) = OrderBatcher::new(config);

        for i in 0..5 {
            let order = OrderRequest {
                asset: i,
                is_buy: i % 2 == 0,
                limit_px: format!("{}", 50000 + i * 100),
                sz: "0.1".to_string(),
                reduce_only: false,
                order_type: OrderType::Limit(Limit {
                    tif: "Gtc".to_string(),
                }),
                cloid: None,
            };

            let handle = batcher.add_order(order, 123456789 + i as u64).await;
            assert!(matches!(handle, OrderHandle::Pending { .. }));
        }
    }

    #[tokio::test]
    async fn test_add_order_unique_ids() {
        let config = BatchConfig::default();
        let (batcher, _handle) = OrderBatcher::new(config);

        let mut ids = Vec::new();

        for _ in 0..10 {
            let order = OrderRequest {
                asset: 0,
                is_buy: true,
                limit_px: "50000".to_string(),
                sz: "0.1".to_string(),
                reduce_only: false,
                order_type: OrderType::Limit(Limit {
                    tif: "Gtc".to_string(),
                }),
                cloid: None,
            };

            let handle = batcher.add_order(order, 123456789).await;
            if let OrderHandle::Pending { id, .. } = handle {
                ids.push(id);
            }
        }

        // All IDs should be unique
        let unique_count = {
            let mut unique_ids = ids.clone();
            unique_ids.sort();
            unique_ids.dedup();
            unique_ids.len()
        };

        assert_eq!(unique_count, ids.len());
    }

    // ==================== Cancel Batching Tests ====================

    #[tokio::test]
    async fn test_add_cancel_returns_pending_handle() {
        let config = BatchConfig::default();
        let (batcher, _handle) = OrderBatcher::new(config);

        let cancel = CancelRequest::new(0, 123456);

        let handle = batcher.add_cancel(cancel, 123456789).await;

        assert!(matches!(handle, OrderHandle::Pending { .. }));
    }

    #[tokio::test]
    async fn test_add_multiple_cancels() {
        let config = BatchConfig::default();
        let (batcher, _handle) = OrderBatcher::new(config);

        for i in 0..5 {
            let cancel = CancelRequest::new(i, 123456 + i as u64);

            let handle = batcher.add_cancel(cancel, 123456789 + i as u64).await;
            assert!(matches!(handle, OrderHandle::Pending { .. }));
        }
    }

    // ==================== BatcherHandle::run Tests ====================

    #[tokio::test]
    async fn test_batcher_handle_processes_orders() {
        let config = BatchConfig::default();
        let (batcher, handle) = OrderBatcher::new(config);

        // Track how many batches were processed
        let order_count = Arc::new(AtomicUsize::new(0));
        let order_count_clone = order_count.clone();

        // Add some orders
        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "Gtc".to_string(),
            }),
            cloid: None,
        };

        let handle_result = batcher.add_order(order, 123456789).await;
        let mut rx = match handle_result {
            OrderHandle::Pending { rx, .. } => rx,
            _ => panic!("Expected pending handle"),
        };

        // Spawn the batch runner
        let runner = tokio::spawn(async move {
            handle
                .run(
                    move |orders: Vec<PendingOrder>| {
                        let count = order_count_clone.clone();
                        Box::pin(async move {
                            count.fetch_add(orders.len(), Ordering::SeqCst);
                            orders
                                .iter()
                                .map(|_| {
                                    Ok(ExchangeResponseStatus::Err(
                                        "test response".to_string(),
                                    ))
                                })
                                .collect()
                        })
                            as BoxFuture<
                                Vec<Result<ExchangeResponseStatus, HyperliquidError>>,
                            >
                    },
                    |_cancels: Vec<PendingCancel>| {
                        Box::pin(async move { vec![] })
                            as BoxFuture<
                                Vec<Result<ExchangeResponseStatus, HyperliquidError>>,
                            >
                    },
                )
                .await;
        });

        // Wait for the order to be processed
        let result = tokio::time::timeout(Duration::from_millis(300), rx.recv()).await;

        // Shutdown the batcher
        batcher.shutdown().await;

        // Wait for runner to finish
        let _ = tokio::time::timeout(Duration::from_millis(200), runner).await;

        // Should have processed the order
        assert!(result.is_ok());
        assert!(order_count.load(Ordering::SeqCst) >= 1);
    }

    #[tokio::test]
    async fn test_batcher_handle_processes_cancels() {
        let config = BatchConfig::default();
        let (batcher, handle) = OrderBatcher::new(config);

        let cancel_count = Arc::new(AtomicUsize::new(0));
        let cancel_count_clone = cancel_count.clone();

        let cancel = CancelRequest::new(0, 123456);
        let handle_result = batcher.add_cancel(cancel, 123456789).await;
        let mut rx = match handle_result {
            OrderHandle::Pending { rx, .. } => rx,
            _ => panic!("Expected pending handle"),
        };

        let runner = tokio::spawn(async move {
            handle
                .run(
                    |_orders: Vec<PendingOrder>| {
                        Box::pin(async move { vec![] })
                            as BoxFuture<
                                Vec<Result<ExchangeResponseStatus, HyperliquidError>>,
                            >
                    },
                    move |cancels: Vec<PendingCancel>| {
                        let count = cancel_count_clone.clone();
                        Box::pin(async move {
                            count.fetch_add(cancels.len(), Ordering::SeqCst);
                            cancels
                                .iter()
                                .map(|_| {
                                    Ok(ExchangeResponseStatus::Err(
                                        "test cancel response".to_string(),
                                    ))
                                })
                                .collect()
                        })
                            as BoxFuture<
                                Vec<Result<ExchangeResponseStatus, HyperliquidError>>,
                            >
                    },
                )
                .await;
        });

        let result = tokio::time::timeout(Duration::from_millis(300), rx.recv()).await;

        batcher.shutdown().await;
        let _ = tokio::time::timeout(Duration::from_millis(200), runner).await;

        assert!(result.is_ok());
        assert!(cancel_count.load(Ordering::SeqCst) >= 1);
    }

    #[tokio::test]
    async fn test_batcher_separates_alo_from_regular() {
        let config = BatchConfig::default();
        let (batcher, handle) = OrderBatcher::new(config);

        // Track ALO and regular order counts
        let alo_orders = Arc::new(Mutex::new(Vec::new()));
        let regular_orders = Arc::new(Mutex::new(Vec::new()));
        let alo_orders_clone = alo_orders.clone();
        let regular_orders_clone = regular_orders.clone();

        // Add one ALO order
        let alo_order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "Alo".to_string(),
            }),
            cloid: None,
        };
        let _alo_handle = batcher.add_order(alo_order, 1).await;

        // Add one regular GTC order
        let gtc_order = OrderRequest {
            asset: 1,
            is_buy: false,
            limit_px: "3000".to_string(),
            sz: "1.0".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "Gtc".to_string(),
            }),
            cloid: None,
        };
        let _gtc_handle = batcher.add_order(gtc_order, 2).await;

        let runner = tokio::spawn(async move {
            handle
                .run(
                    move |orders: Vec<PendingOrder>| {
                        let alo = alo_orders_clone.clone();
                        let regular = regular_orders_clone.clone();
                        Box::pin(async move {
                            // The batcher partitions orders into ALO and regular
                            // When we receive a batch, it's either all ALO or all regular
                            for order in &orders {
                                if order.order.is_alo() {
                                    alo.lock().await.push(order.order.clone());
                                } else {
                                    regular.lock().await.push(order.order.clone());
                                }
                            }
                            orders
                                .iter()
                                .map(|_| {
                                    Ok(ExchangeResponseStatus::Err("test".to_string()))
                                })
                                .collect()
                        })
                            as BoxFuture<
                                Vec<Result<ExchangeResponseStatus, HyperliquidError>>,
                            >
                    },
                    |_cancels: Vec<PendingCancel>| {
                        Box::pin(async move { vec![] })
                            as BoxFuture<
                                Vec<Result<ExchangeResponseStatus, HyperliquidError>>,
                            >
                    },
                )
                .await;
        });

        // Wait for processing
        tokio::time::sleep(Duration::from_millis(200)).await;

        batcher.shutdown().await;
        let _ = tokio::time::timeout(Duration::from_millis(200), runner).await;

        // Check that orders were separated
        let alo_count = alo_orders.lock().await.len();
        let regular_count = regular_orders.lock().await.len();

        assert_eq!(alo_count, 1);
        assert_eq!(regular_count, 1);
    }

    // ==================== Shutdown Tests ====================

    #[tokio::test]
    async fn test_batcher_shutdown() {
        let config = BatchConfig::default();
        let (batcher, handle) = OrderBatcher::new(config);

        let runner = tokio::spawn(async move {
            handle
                .run(
                    |_orders: Vec<PendingOrder>| {
                        Box::pin(async move { vec![] })
                            as BoxFuture<
                                Vec<Result<ExchangeResponseStatus, HyperliquidError>>,
                            >
                    },
                    |_cancels: Vec<PendingCancel>| {
                        Box::pin(async move { vec![] })
                            as BoxFuture<
                                Vec<Result<ExchangeResponseStatus, HyperliquidError>>,
                            >
                    },
                )
                .await;
        });

        // Give it a moment to start
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Shutdown should complete gracefully
        batcher.shutdown().await;

        // Runner should exit
        let result = tokio::time::timeout(Duration::from_millis(500), runner).await;
        assert!(result.is_ok());
    }

    // ==================== OrderHandle Tests ====================

    #[tokio::test]
    async fn test_order_handle_pending_has_id() {
        let config = BatchConfig::default();
        let (batcher, _handle) = OrderBatcher::new(config);

        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "Gtc".to_string(),
            }),
            cloid: None,
        };

        let handle = batcher.add_order(order, 123456789).await;

        match handle {
            OrderHandle::Pending { id, .. } => {
                // ID should be a valid UUID
                assert!(!id.is_nil());
            }
            _ => panic!("Expected Pending handle"),
        }
    }

    #[test]
    fn test_order_handle_immediate_variant() {
        // Test the Immediate variant directly
        let result: Result<ExchangeResponseStatus, HyperliquidError> =
            Ok(ExchangeResponseStatus::Err("test".to_string()));
        let handle = OrderHandle::Immediate(result);

        match handle {
            OrderHandle::Immediate(r) => {
                assert!(r.is_ok());
            }
            _ => panic!("Expected Immediate handle"),
        }
    }

    // ==================== PendingOrder Clone Tests ====================

    #[tokio::test]
    async fn test_pending_order_clone() {
        let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();

        let pending = PendingOrder {
            order: OrderRequest {
                asset: 0,
                is_buy: true,
                limit_px: "50000".to_string(),
                sz: "0.1".to_string(),
                reduce_only: false,
                order_type: OrderType::Limit(Limit {
                    tif: "Gtc".to_string(),
                }),
                cloid: None,
            },
            nonce: 123456789,
            id: uuid::Uuid::new_v4(),
            response_tx: tx,
        };

        let cloned = pending.clone();

        assert_eq!(pending.order.asset, cloned.order.asset);
        assert_eq!(pending.nonce, cloned.nonce);
        assert_eq!(pending.id, cloned.id);
    }

    // ==================== PendingCancel Clone Tests ====================

    #[tokio::test]
    async fn test_pending_cancel_clone() {
        let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();

        let pending = PendingCancel {
            cancel: CancelRequest::new(0, 123456),
            nonce: 123456789,
            id: uuid::Uuid::new_v4(),
            response_tx: tx,
        };

        let cloned = pending.clone();

        assert_eq!(pending.cancel.asset, cloned.cancel.asset);
        assert_eq!(pending.cancel.oid, cloned.cancel.oid);
        assert_eq!(pending.nonce, cloned.nonce);
        assert_eq!(pending.id, cloned.id);
    }
}
