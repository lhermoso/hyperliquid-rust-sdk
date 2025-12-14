//! Managed exchange provider with safety features and optimizations.

use std::sync::Arc;

use alloy::primitives::Address;
use tokio::sync::Mutex as TokioMutex;

use crate::{
    constants::Network,
    errors::HyperliquidError,
    providers::{
        agent::{AgentConfig, AgentManager, AgentWallet},
        batcher::{BatchConfig, OrderBatcher, OrderHandle},
        nonce::NonceManager,
    },
    signers::HyperliquidSigner,
    types::{
        requests::{CancelRequest, OrderRequest},
        responses::ExchangeResponseStatus,
    },
};

use super::RawExchangeProvider;

type Result<T> = std::result::Result<T, HyperliquidError>;

/// Configuration for managed exchange provider.
#[derive(Clone, Debug)]
pub struct ManagedExchangeConfig {
    /// Enable automatic order batching
    pub batch_orders: bool,
    /// Batch configuration
    pub batch_config: BatchConfig,

    /// Agent lifecycle management
    pub auto_rotate_agents: bool,
    /// Agent configuration
    pub agent_config: AgentConfig,

    /// Nonce isolation per subaccount
    pub isolate_subaccount_nonces: bool,

    /// Safety features
    pub prevent_agent_address_queries: bool,
    pub warn_on_high_nonce_velocity: bool,
}

impl Default for ManagedExchangeConfig {
    fn default() -> Self {
        Self {
            batch_orders: false,
            batch_config: BatchConfig::default(),
            auto_rotate_agents: true,
            agent_config: AgentConfig::default(),
            isolate_subaccount_nonces: true,
            prevent_agent_address_queries: true,
            warn_on_high_nonce_velocity: true,
        }
    }
}

/// Managed exchange provider with safety features and optimizations.
///
/// This provider wraps `RawExchangeProvider` and adds:
/// - Automatic agent rotation for security
/// - Order batching for performance
/// - Nonce management for correctness
///
/// # Example
/// ```ignore
/// let provider = ManagedExchangeProvider::builder(signer)
///     .with_network(Network::Mainnet)
///     .with_auto_batching(Duration::from_millis(100))
///     .build()
///     .await?;
/// ```
pub struct ManagedExchangeProvider<S: HyperliquidSigner> {
    /// Inner raw provider
    inner: Arc<RawExchangeProvider<S>>,

    /// Agent manager for lifecycle
    agent_manager: Option<Arc<AgentManager<S>>>,

    /// Nonce tracking
    nonce_manager: Arc<NonceManager>,

    /// Order batching
    batcher: Option<Arc<OrderBatcher>>,
    batcher_handle: Option<Arc<TokioMutex<Option<tokio::task::JoinHandle<()>>>>>,

    /// Configuration
    config: ManagedExchangeConfig,
}

impl<S: HyperliquidSigner + Clone + 'static> ManagedExchangeProvider<S> {
    /// Create a builder for managed provider.
    pub fn builder(signer: S) -> ManagedExchangeProviderBuilder<S> {
        ManagedExchangeProviderBuilder::new(signer)
    }

    /// Create with default configuration for mainnet.
    pub async fn mainnet(signer: S) -> Result<Arc<Self>> {
        Self::builder(signer)
            .with_network(Network::Mainnet)
            .build()
            .await
    }

    /// Create with default configuration for testnet.
    pub async fn testnet(signer: S) -> Result<Arc<Self>> {
        Self::builder(signer)
            .with_network(Network::Testnet)
            .build()
            .await
    }

    /// Place an order with all managed features.
    pub async fn place_order(&self, order: &OrderRequest) -> Result<OrderHandle> {
        // Get nonce based on configuration
        let nonce = if self.config.auto_rotate_agents {
            if let Some(agent_mgr) = &self.agent_manager {
                let agent = agent_mgr.get_or_rotate_agent("default").await?;
                // Use agent's nonce
                agent.next_nonce()
            } else {
                // Fallback to regular nonce
                self.nonce_manager.next_nonce(None)
            }
        } else {
            // Not using agents, use regular nonce
            if self.config.isolate_subaccount_nonces {
                // For subaccounts, we'd need to extract the address from somewhere
                // For now, just use global nonce
                self.nonce_manager.next_nonce(None)
            } else {
                self.nonce_manager.next_nonce(None)
            }
        };

        // Check nonce validity
        if !NonceManager::is_valid_nonce(nonce) {
            return Err(HyperliquidError::InvalidRequest(
                "Generated nonce is outside valid time bounds".to_string(),
            ));
        }

        // For now, we always use the main provider
        // In a full implementation, we'd need to handle agent signing differently
        // This is a limitation of the current design where we can't easily swap signers

        // Batch or direct execution
        if self.config.batch_orders {
            if let Some(batcher) = &self.batcher {
                Ok(batcher.add_order(order.clone(), nonce).await)
            } else {
                // Fallback to direct
                let result = self.inner.place_order(order).await?;
                Ok(OrderHandle::Immediate(Ok(result)))
            }
        } else {
            // Direct execution
            let result = self.inner.place_order(order).await?;
            Ok(OrderHandle::Immediate(Ok(result)))
        }
    }

    /// Place order immediately, bypassing batch.
    pub async fn place_order_immediate(
        &self,
        order: &OrderRequest,
    ) -> Result<ExchangeResponseStatus> {
        self.inner.place_order(order).await
    }

    /// Access the raw provider for advanced usage.
    pub fn raw(&self) -> &RawExchangeProvider<S> {
        &self.inner
    }

    /// Get current agent status.
    pub async fn get_agent_status(&self) -> Option<Vec<(String, AgentWallet)>> {
        if let Some(agent_mgr) = &self.agent_manager {
            Some(agent_mgr.get_active_agents().await)
        } else {
            None
        }
    }

    /// Shutdown the managed provider cleanly.
    pub async fn shutdown(self: Arc<Self>) {
        // Stop batcher if running
        if let Some(handle_mutex) = &self.batcher_handle {
            if let Some(handle) = handle_mutex.lock().await.take() {
                handle.abort();
            }
        }
    }
}

/// Builder for ManagedExchangeProvider.
pub struct ManagedExchangeProviderBuilder<S: HyperliquidSigner> {
    signer: S,
    network: Network,
    config: ManagedExchangeConfig,
    vault_address: Option<Address>,
    initial_agent: Option<String>,
    builder_address: Option<Address>,
}

impl<S: HyperliquidSigner + Clone + 'static> ManagedExchangeProviderBuilder<S> {
    fn new(signer: S) -> Self {
        Self {
            signer,
            network: Network::Mainnet,
            config: ManagedExchangeConfig::default(),
            vault_address: None,
            initial_agent: None,
            builder_address: None,
        }
    }

    /// Set network.
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    /// Enable automatic order batching.
    pub fn with_auto_batching(mut self, interval: std::time::Duration) -> Self {
        self.config.batch_orders = true;
        self.config.batch_config.interval = interval;
        self
    }

    /// Configure agent rotation.
    pub fn with_agent_rotation(mut self, ttl: std::time::Duration) -> Self {
        self.config.auto_rotate_agents = true;
        self.config.agent_config.ttl = ttl;
        self
    }

    /// Start with an agent.
    pub fn with_agent(mut self, name: Option<String>) -> Self {
        self.initial_agent = name;
        self.config.auto_rotate_agents = true;
        self
    }

    /// Set vault address.
    pub fn with_vault(mut self, vault: Address) -> Self {
        self.vault_address = Some(vault);
        self
    }

    /// Set builder address.
    pub fn with_builder(mut self, builder: Address) -> Self {
        self.builder_address = Some(builder);
        self
    }

    /// Disable agent rotation.
    pub fn without_agent_rotation(mut self) -> Self {
        self.config.auto_rotate_agents = false;
        self
    }

    /// Build the provider.
    pub async fn build(self) -> Result<Arc<ManagedExchangeProvider<S>>> {
        // Create raw provider
        let raw = match self.network {
            Network::Mainnet => {
                if let Some(vault) = self.vault_address {
                    RawExchangeProvider::mainnet_vault(self.signer.clone(), vault)
                } else if let Some(builder) = self.builder_address {
                    RawExchangeProvider::mainnet_builder(self.signer.clone(), builder)
                } else {
                    RawExchangeProvider::mainnet(self.signer.clone())
                }
            }
            Network::Testnet => {
                if let Some(vault) = self.vault_address {
                    RawExchangeProvider::testnet_vault(self.signer.clone(), vault)
                } else if let Some(builder) = self.builder_address {
                    RawExchangeProvider::testnet_builder(self.signer.clone(), builder)
                } else {
                    RawExchangeProvider::testnet(self.signer.clone())
                }
            }
        };

        let inner = Arc::new(raw);

        // Create agent manager if needed
        let agent_manager = if self.config.auto_rotate_agents {
            Some(Arc::new(AgentManager::new(
                self.signer,
                self.config.agent_config.clone(),
                self.network,
            )))
        } else {
            None
        };

        // Create nonce manager
        let nonce_manager =
            Arc::new(NonceManager::new(self.config.isolate_subaccount_nonces));

        // Create batcher if needed
        let (batcher, batcher_handle) = if self.config.batch_orders {
            let (batcher, handle) = OrderBatcher::new(self.config.batch_config.clone());
            let batcher = Arc::new(batcher);

            // Spawn batch processing task
            let inner_clone = inner.clone();
            let inner_clone2 = inner.clone();
            let handle_future = tokio::spawn(async move {
                handle
                    .run(
                        move |orders| {
                            let inner = inner_clone.clone();
                            Box::pin(async move {
                                // Execute batch
                                let order_requests: Vec<OrderRequest> =
                                    orders.iter().map(|o| o.order.clone()).collect();

                                match inner.bulk_orders(order_requests).await {
                                    Ok(status) => {
                                        // Return same status for all orders in batch
                                        orders
                                            .iter()
                                            .map(|_| Ok(status.clone()))
                                            .collect()
                                    }
                                    Err(e) => {
                                        // Return same error for all orders in batch
                                        let err_str = e.to_string();
                                        orders
                                            .iter()
                                            .map(|_| {
                                                Err(HyperliquidError::InvalidResponse(
                                                    err_str.clone(),
                                                ))
                                            })
                                            .collect()
                                    }
                                }
                            })
                        },
                        move |cancels| {
                            let inner = inner_clone2.clone();
                            Box::pin(async move {
                                // Execute cancel batch
                                let cancel_requests: Vec<CancelRequest> =
                                    cancels.iter().map(|c| c.cancel.clone()).collect();

                                match inner.bulk_cancel(cancel_requests).await {
                                    Ok(status) => {
                                        // Return same status for all cancels in batch
                                        cancels
                                            .iter()
                                            .map(|_| Ok(status.clone()))
                                            .collect()
                                    }
                                    Err(e) => {
                                        // Return same error for all cancels in batch
                                        let err_str = e.to_string();
                                        cancels
                                            .iter()
                                            .map(|_| {
                                                Err(HyperliquidError::InvalidResponse(
                                                    err_str.clone(),
                                                ))
                                            })
                                            .collect()
                                    }
                                }
                            })
                        },
                    )
                    .await;
            });

            (
                Some(batcher),
                Some(Arc::new(TokioMutex::new(Some(handle_future)))),
            )
        } else {
            (None, None)
        };

        let provider = Arc::new(ManagedExchangeProvider {
            inner,
            agent_manager,
            nonce_manager,
            batcher,
            batcher_handle,
            config: self.config,
        });

        // Initialize agent if requested
        if let Some(agent_name) = self.initial_agent {
            if let Some(agent_mgr) = &provider.agent_manager {
                agent_mgr.get_or_rotate_agent(&agent_name).await?;
            }
        }

        Ok(provider)
    }
}
