//! Order builder pattern for constructing orders fluently.

use uuid::Uuid;

use crate::{
    constants::TIF_GTC,
    errors::HyperliquidError,
    signers::HyperliquidSigner,
    types::{
        requests::{Limit, OrderRequest, OrderType, Trigger},
        responses::ExchangeResponseStatus,
    },
};

use super::{format_float_string, RawExchangeProvider};

type Result<T> = std::result::Result<T, HyperliquidError>;

/// Builder pattern for constructing orders fluently.
///
/// # Example
/// ```ignore
/// let response = provider
///     .order(0)  // asset index
///     .buy()
///     .limit_px("50000")
///     .size("0.1")
///     .send()
///     .await?;
/// ```
pub struct OrderBuilder<'a, S: HyperliquidSigner> {
    provider: &'a RawExchangeProvider<S>,
    asset: u32,
    is_buy: Option<bool>,
    limit_px: Option<String>,
    sz: Option<String>,
    reduce_only: bool,
    order_type: Option<OrderType>,
    cloid: Option<Uuid>,
}

impl<'a, S: HyperliquidSigner> OrderBuilder<'a, S> {
    /// Create a new order builder for the given asset.
    pub fn new(provider: &'a RawExchangeProvider<S>, asset: u32) -> Self {
        Self {
            provider,
            asset,
            is_buy: None,
            limit_px: None,
            sz: None,
            reduce_only: false,
            order_type: None,
            cloid: None,
        }
    }

    /// Set this as a buy order.
    pub fn buy(mut self) -> Self {
        self.is_buy = Some(true);
        self
    }

    /// Set this as a sell order.
    pub fn sell(mut self) -> Self {
        self.is_buy = Some(false);
        self
    }

    /// Set the limit price.
    pub fn limit_px(mut self, price: impl ToString) -> Self {
        self.limit_px = Some(price.to_string());
        self
    }

    /// Set the order size.
    pub fn size(mut self, size: impl ToString) -> Self {
        self.sz = Some(size.to_string());
        self
    }

    /// Set whether this is a reduce-only order.
    pub fn reduce_only(mut self, reduce: bool) -> Self {
        self.reduce_only = reduce;
        self
    }

    /// Set the order type (limit, trigger, etc.).
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    /// Set a client order ID.
    pub fn cloid(mut self, id: Uuid) -> Self {
        self.cloid = Some(id);
        self
    }

    /// Convenience method for creating a limit buy order.
    pub fn limit_buy(self, price: impl ToString, size: impl ToString) -> Self {
        self.buy().limit_px(price).size(size)
    }

    /// Convenience method for creating a limit sell order.
    pub fn limit_sell(self, price: impl ToString, size: impl ToString) -> Self {
        self.sell().limit_px(price).size(size)
    }

    /// Convenience method for creating a trigger buy order.
    pub fn trigger_buy(
        self,
        trigger_px: impl ToString,
        size: impl ToString,
        tpsl: &str,
    ) -> Self {
        let trigger_px_str = trigger_px.to_string();
        self.buy()
            .limit_px(&trigger_px_str) // limit_px must equal trigger_px for trigger orders
            .size(size)
            .order_type(OrderType::Trigger(Trigger {
                is_market: true,
                trigger_px: trigger_px_str,
                tpsl: tpsl.to_string(),
            }))
    }

    /// Convenience method for creating a trigger sell order.
    pub fn trigger_sell(
        self,
        trigger_px: impl ToString,
        size: impl ToString,
        tpsl: &str,
    ) -> Self {
        let trigger_px_str = trigger_px.to_string();
        self.sell()
            .limit_px(&trigger_px_str) // limit_px must equal trigger_px for trigger orders
            .size(size)
            .order_type(OrderType::Trigger(Trigger {
                is_market: true,
                trigger_px: trigger_px_str,
                tpsl: tpsl.to_string(),
            }))
    }

    /// Build the order request without sending it.
    pub fn build(self) -> Result<OrderRequest> {
        let limit_px = self.limit_px.ok_or(HyperliquidError::InvalidRequest(
            "limit_px must be specified".to_string(),
        ))?;
        let sz = self.sz.ok_or(HyperliquidError::InvalidRequest(
            "sz must be specified".to_string(),
        ))?;

        // Parse and format the prices to match API expectations
        let limit_px_f64 = limit_px.parse::<f64>().map_err(|_| {
            HyperliquidError::InvalidRequest("Invalid limit_px format".to_string())
        })?;
        let sz_f64 = sz.parse::<f64>().map_err(|_| {
            HyperliquidError::InvalidRequest("Invalid sz format".to_string())
        })?;

        Ok(OrderRequest {
            asset: self.asset,
            is_buy: self.is_buy.ok_or(HyperliquidError::InvalidRequest(
                "is_buy must be specified".to_string(),
            ))?,
            limit_px: format_float_string(limit_px_f64),
            sz: format_float_string(sz_f64),
            reduce_only: self.reduce_only,
            order_type: self.order_type.unwrap_or(OrderType::Limit(Limit {
                tif: TIF_GTC.to_string(),
            })),
            cloid: self.cloid.map(|id| format!("{:032x}", id.as_u128())),
        })
    }

    /// Build and send the order.
    pub async fn send(self) -> Result<ExchangeResponseStatus> {
        let provider = self.provider;
        let order = self.build()?;
        provider.place_order(&order).await
    }
}

impl<S: HyperliquidSigner> RawExchangeProvider<S> {
    /// Create an order builder for the given asset.
    ///
    /// # Example
    /// ```ignore
    /// let response = provider
    ///     .order(0)
    ///     .buy()
    ///     .limit_px("50000")
    ///     .size("0.1")
    ///     .send()
    ///     .await?;
    /// ```
    pub fn order(&self, asset: u32) -> OrderBuilder<'_, S> {
        OrderBuilder::new(self, asset)
    }
}
