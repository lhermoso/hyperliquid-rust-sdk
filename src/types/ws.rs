//! WebSocket message types for Hyperliquid

use std::collections::HashMap;

use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

// Subscription types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Subscription {
    AllMids,
    Notification { user: Address },
    WebData2 { user: Address },
    Candle { coin: String, interval: String },
    L2Book { coin: String },
    Trades { coin: String },
    OrderUpdates { user: Address },
    UserEvents { user: Address },
    UserFills { user: Address },
    UserFundings { user: Address },
    UserNonFundingLedgerUpdates { user: Address },
    // Phase 1 new subscriptions
    Bbo { coin: String },
    OpenOrders { user: Address },
    ClearinghouseState { user: Address },
    // Phase 2 new subscriptions
    WebData3 { user: Address },
    TwapStates { user: Address },
    ActiveAssetCtx { coin: String },
    ActiveAssetData { user: Address, coin: String },
    UserTwapSliceFills { user: Address },
    UserTwapHistory { user: Address },
}

// Incoming message types
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "channel", rename_all = "camelCase")]
pub enum Message {
    AllMids(AllMids),
    Trades(Trades),
    L2Book(L2Book),
    Candle(Candle),
    OrderUpdates(OrderUpdates),
    UserFills(UserFills),
    UserFundings(UserFundings),
    UserNonFundingLedgerUpdates(UserNonFundingLedgerUpdates),
    Notification(Notification),
    WebData2(WebData2),
    User(User),
    SubscriptionResponse,
    Pong,
    // Phase 1 new message types
    Bbo(Bbo),
    OpenOrders(OpenOrdersWs),
    ClearinghouseState(ClearinghouseStateWs),
    // Phase 2 new message types
    WebData3(WebData3Ws),
    TwapStates(TwapStatesWs),
    ActiveAssetCtx(ActiveAssetCtxWs),
    ActiveAssetData(ActiveAssetDataWs),
    UserTwapSliceFills(UserTwapSliceFillsWs),
    UserTwapHistory(UserTwapHistoryWs),
}

// Market data structures
#[derive(Debug, Clone, Deserialize)]
pub struct AllMids {
    pub data: AllMidsData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllMidsData {
    pub mids: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Trades {
    pub data: Vec<Trade>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub time: u64,
    pub hash: String,
    pub tid: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct L2Book {
    pub data: L2BookData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct L2BookData {
    pub coin: String,
    pub time: u64,
    pub levels: Vec<Vec<BookLevel>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookLevel {
    pub px: String,
    pub sz: String,
    pub n: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Candle {
    pub data: CandleData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CandleData {
    #[serde(rename = "T")]
    pub time_close: u64,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "n")]
    pub num_trades: u64,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "s")]
    pub coin: String,
    #[serde(rename = "t")]
    pub time_open: u64,
    #[serde(rename = "v")]
    pub volume: String,
}

// User event structures
#[derive(Debug, Clone, Deserialize)]
pub struct OrderUpdates {
    pub data: Vec<OrderUpdate>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdate {
    pub order: BasicOrder,
    pub status: String,
    pub status_timestamp: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicOrder {
    pub coin: String,
    pub side: String,
    pub limit_px: String,
    pub sz: String,
    pub oid: u64,
    pub timestamp: u64,
    pub orig_sz: String,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserFills {
    pub data: UserFillsData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFillsData {
    pub is_snapshot: Option<bool>,
    pub user: Address,
    pub fills: Vec<TradeInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeInfo {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub time: u64,
    pub hash: String,
    pub start_position: String,
    pub dir: String,
    pub closed_pnl: String,
    pub oid: u64,
    pub cloid: Option<String>,
    pub crossed: bool,
    pub fee: String,
    pub fee_token: String,
    pub tid: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserFundings {
    pub data: UserFundingsData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFundingsData {
    pub is_snapshot: Option<bool>,
    pub user: Address,
    pub fundings: Vec<UserFunding>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFunding {
    pub time: u64,
    pub coin: String,
    pub usdc: String,
    pub szi: String,
    pub funding_rate: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserNonFundingLedgerUpdates {
    pub data: UserNonFundingLedgerUpdatesData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserNonFundingLedgerUpdatesData {
    pub is_snapshot: Option<bool>,
    pub user: Address,
    pub non_funding_ledger_updates: Vec<LedgerUpdateData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LedgerUpdateData {
    pub time: u64,
    pub hash: String,
    pub delta: LedgerUpdate,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum LedgerUpdate {
    Deposit {
        usdc: String,
    },
    Withdraw {
        usdc: String,
        nonce: u64,
        fee: String,
    },
    InternalTransfer {
        usdc: String,
        user: Address,
        destination: Address,
        fee: String,
    },
    SubAccountTransfer {
        usdc: String,
        user: Address,
        destination: Address,
    },
    SpotTransfer {
        token: String,
        amount: String,
        user: Address,
        destination: Address,
        fee: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Notification {
    pub data: NotificationData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationData {
    pub notification: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebData2 {
    pub data: WebData2Data,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebData2Data {
    pub user: Address,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub data: UserData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum UserData {
    Fills(Vec<TradeInfo>),
    Funding(UserFunding),
}

// WebSocket protocol messages
#[derive(Debug, Serialize)]
pub struct WsRequest {
    pub method: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}

impl WsRequest {
    pub fn subscribe(subscription: Subscription) -> Self {
        Self {
            method: "subscribe",
            subscription: Some(subscription),
        }
    }

    pub fn unsubscribe(subscription: Subscription) -> Self {
        Self {
            method: "unsubscribe",
            subscription: Some(subscription),
        }
    }

    pub fn ping() -> Self {
        Self {
            method: "ping",
            subscription: None,
        }
    }
}

// ==================== Phase 1 New Message Types ====================

/// Best bid/offer update
#[derive(Debug, Clone, Deserialize)]
pub struct Bbo {
    pub data: BboData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BboData {
    pub coin: String,
    pub time: u64,
    pub bbo: BboLevel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BboLevel {
    pub bid: PriceLevel,
    pub ask: PriceLevel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceLevel {
    pub px: String,
    pub sz: String,
}

/// Real-time open orders
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrdersWs {
    pub data: OpenOrdersWsData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersWsData {
    pub user: Address,
    pub is_snapshot: Option<bool>,
    pub orders: Vec<BasicOrder>,
}

/// Real-time clearinghouse state
#[derive(Debug, Clone, Deserialize)]
pub struct ClearinghouseStateWs {
    pub data: ClearinghouseStateWsData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearinghouseStateWsData {
    pub user: Address,
    pub margin_summary: MarginSummaryWs,
    pub cross_margin_summary: MarginSummaryWs,
    pub withdrawable: String,
    pub asset_positions: Vec<AssetPositionWs>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginSummaryWs {
    pub account_value: String,
    pub total_margin_used: String,
    pub total_ntl_pos: String,
    pub total_raw_usd: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPositionWs {
    pub position: PositionWs,
    #[serde(rename = "type")]
    pub type_string: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionWs {
    pub coin: String,
    pub entry_px: Option<String>,
    pub liquidation_px: Option<String>,
    pub margin_used: String,
    pub position_value: String,
    pub return_on_equity: String,
    pub szi: String,
    pub unrealized_pnl: String,
}

// ==================== Phase 2 New Message Types ====================

/// WebData3 - Aggregate user information (newer version)
#[derive(Debug, Clone, Deserialize)]
pub struct WebData3Ws {
    pub data: WebData3Data,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebData3Data {
    pub user: Address,
    #[serde(default)]
    pub is_snapshot: Option<bool>,
    #[serde(default)]
    pub clearinghouse_state: Option<ClearinghouseStateWsData>,
    #[serde(default)]
    pub open_orders: Option<Vec<BasicOrder>>,
    #[serde(default)]
    pub fills: Option<Vec<TradeInfo>>,
    #[serde(default)]
    pub fundings: Option<Vec<UserFunding>>,
}

/// TWAP order states
#[derive(Debug, Clone, Deserialize)]
pub struct TwapStatesWs {
    pub data: TwapStatesData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapStatesData {
    pub user: Address,
    #[serde(default)]
    pub is_snapshot: Option<bool>,
    pub twap_states: Vec<TwapState>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapState {
    pub twap_id: u64,
    pub coin: String,
    pub side: String,
    pub sz: String,
    pub sz_filled: String,
    pub duration_minutes: u32,
    pub start_time: u64,
    pub end_time: u64,
    pub status: String,
    #[serde(default)]
    pub randomize: Option<bool>,
}

/// Active asset context
#[derive(Debug, Clone, Deserialize)]
pub struct ActiveAssetCtxWs {
    pub data: ActiveAssetCtxData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveAssetCtxData {
    pub coin: String,
    pub ctx: AssetCtx,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetCtx {
    pub funding: String,
    pub open_interest: String,
    pub prev_day_px: String,
    pub day_ntl_vlm: String,
    pub premium: Option<String>,
    pub oracle_px: String,
    pub mark_px: String,
    pub mid_px: Option<String>,
    pub impact_pxs: Option<Vec<String>>,
}

/// Active asset data (perps only)
#[derive(Debug, Clone, Deserialize)]
pub struct ActiveAssetDataWs {
    pub data: ActiveAssetDataData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveAssetDataData {
    pub user: Address,
    pub coin: String,
    pub leverage: LeverageWs,
    #[serde(default)]
    pub max_trade_szs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageWs {
    #[serde(rename = "type")]
    pub leverage_type: String,
    pub value: u32,
    #[serde(default)]
    pub raw_usd: Option<String>,
}

/// User TWAP slice fills
#[derive(Debug, Clone, Deserialize)]
pub struct UserTwapSliceFillsWs {
    pub data: UserTwapSliceFillsData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTwapSliceFillsData {
    pub user: Address,
    #[serde(default)]
    pub is_snapshot: Option<bool>,
    pub twap_slice_fills: Vec<TwapSliceFill>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapSliceFill {
    pub twap_id: u64,
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub time: u64,
    pub fee: String,
    pub oid: u64,
    pub hash: String,
}

/// User TWAP history
#[derive(Debug, Clone, Deserialize)]
pub struct UserTwapHistoryWs {
    pub data: UserTwapHistoryData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTwapHistoryData {
    pub user: Address,
    #[serde(default)]
    pub is_snapshot: Option<bool>,
    pub twap_history: Vec<TwapHistoryEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapHistoryEntry {
    pub twap_id: u64,
    pub coin: String,
    pub side: String,
    pub sz: String,
    pub sz_filled: String,
    pub avg_px: Option<String>,
    pub duration_minutes: u32,
    pub start_time: u64,
    pub end_time: u64,
    pub status: String,
    #[serde(default)]
    pub randomize: Option<bool>,
}
