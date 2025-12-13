use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

// ==================== Request Types ====================

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandleSnapshotRequest {
    pub coin: String,
    pub interval: String,
    pub start_time: u64,
    pub end_time: u64,
}

// ==================== Common Response Types ====================

// Note: AllMids returns HashMap<String, String> directly, not wrapped

// ==================== Position & Margin Types ====================

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetPosition {
    pub position: PositionData,
    #[serde(rename = "type")]
    pub type_string: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicOrderInfo {
    pub coin: String,
    pub side: String,
    pub limit_px: String,
    pub sz: String,
    pub oid: u64,
    pub timestamp: u64,
    pub trigger_condition: String,
    pub is_trigger: bool,
    pub trigger_px: String,
    pub is_position_tpsl: bool,
    pub reduce_only: bool,
    pub order_type: String,
    pub orig_sz: String,
    pub tif: String,
    pub cloid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CandlesSnapshotResponse {
    #[serde(rename = "t")]
    pub time_open: u64,
    #[serde(rename = "T")]
    pub time_close: u64,
    #[serde(rename = "s")]
    pub coin: String,
    #[serde(rename = "i")]
    pub candle_interval: String,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub vlm: String,
    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CumulativeFunding {
    pub all_time: String,
    pub since_open: String,
    pub since_change: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyUserVlm {
    pub date: String,
    pub exchange: String,
    pub user_add: String,
    pub user_cross: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Delta {
    #[serde(rename = "type")]
    pub type_string: String,
    pub coin: String,
    pub usdc: String,
    pub szi: String,
    pub funding_rate: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeeSchedule {
    pub add: String,
    pub cross: String,
    pub referral_discount: String,
    pub tiers: Tiers,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingHistoryResponse {
    pub coin: String,
    pub funding_rate: String,
    pub premium: String,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct L2SnapshotResponse {
    pub coin: String,
    pub levels: Vec<Vec<Level>>,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub n: u64,
    pub px: String,
    pub sz: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Leverage {
    #[serde(rename = "type")]
    pub type_string: String,
    pub value: u32,
    pub raw_usd: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginSummary {
    pub account_value: String,
    pub total_margin_used: String,
    pub total_ntl_pos: String,
    pub total_raw_usd: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mm {
    pub add: String,
    pub maker_fraction_cutoff: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersResponse {
    pub coin: String,
    pub limit_px: String,
    pub oid: u64,
    pub side: String,
    pub sz: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub order: BasicOrderInfo,
    pub status: String,
    pub status_timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatusResponse {
    pub status: String,
    /// `None` if the order is not found
    #[serde(default)]
    pub order: Option<OrderInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
    pub coin: String,
    pub entry_px: Option<String>,
    pub leverage: Leverage,
    pub liquidation_px: Option<String>,
    pub margin_used: String,
    pub position_value: String,
    pub return_on_equity: String,
    pub szi: String,
    pub unrealized_pnl: String,
    pub max_leverage: u32,
    pub cum_funding: CumulativeFunding,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesResponse {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub time: u64,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Referrer {
    pub referrer: Address,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferralResponse {
    pub referred_by: Option<Referrer>,
    pub cum_vlm: String,
    pub unclaimed_rewards: String,
    pub claimed_rewards: String,
    pub referrer_state: ReferrerState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferrerData {
    pub required: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferrerState {
    pub stage: String,
    pub data: ReferrerData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tiers {
    pub mm: Vec<Mm>,
    pub vip: Vec<Vip>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserFeesResponse {
    pub active_referral_discount: String,
    pub daily_user_vlm: Vec<DailyUserVlm>,
    pub fee_schedule: FeeSchedule,
    pub user_add_rate: String,
    pub user_cross_rate: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserFillsResponse {
    pub closed_pnl: String,
    pub coin: String,
    pub crossed: bool,
    pub dir: String,
    pub hash: String,
    pub oid: u64,
    pub px: String,
    pub side: String,
    pub start_position: String,
    pub sz: String,
    pub time: u64,
    pub fee: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFundingResponse {
    pub time: u64,
    pub hash: String,
    pub delta: Delta,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserStateResponse {
    pub asset_positions: Vec<AssetPosition>,
    pub cross_margin_summary: MarginSummary,
    pub margin_summary: MarginSummary,
    pub withdrawable: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserTokenBalance {
    pub coin: String,
    pub hold: String,
    pub total: String,
    pub entry_ntl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTokenBalanceResponse {
    pub balances: Vec<UserTokenBalance>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vip {
    pub add: String,
    pub cross: String,
    pub ntl_cutoff: String,
}

// ==================== Metadata Types ====================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub universe: Vec<AssetMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetMeta {
    pub name: String,
    pub sz_decimals: u32,
    pub max_leverage: u32,
    #[serde(default)]
    pub only_isolated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_margin_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_margin_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_table_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_delisted: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotMeta {
    pub universe: Vec<SpotPairMeta>,
    pub tokens: Vec<TokenMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotPairMeta {
    pub name: String,
    pub tokens: [u32; 2],
    pub index: u32,
    pub is_canonical: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EvmContract {
    String(String),
    Object {
        address: String,
        evm_extra_wei_decimals: i32,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenMeta {
    pub name: String,
    pub sz_decimals: u32,
    pub wei_decimals: u32,
    pub index: u32,
    pub token_id: String, // Actually a hex string, not Address
    pub is_canonical: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_contract: Option<EvmContract>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployer_trading_fee_share: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotMetaAndAssetCtxs {
    pub universe: Vec<SpotPairMeta>,
    pub tokens: Vec<TokenMeta>,
    pub asset_ctxs: Vec<AssetContext>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetContext {
    pub day_ntl_vlm: String,
    pub funding: String,
    pub impact_pxs: Vec<String>,
    pub mark_px: String,
    pub mid_px: String,
    pub open_interest: String,
    pub oracle_px: String,
    pub premium: String,
    pub prev_day_px: String,
}

// ==================== Phase 1 New Types ====================

/// Response for metaAndAssetCtxs - perp metadata with asset contexts
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetaAndAssetCtxs {
    pub meta: Meta,
    pub asset_ctxs: Vec<PerpAssetContext>,
}

/// Asset context for perpetuals (different from spot AssetContext)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerpAssetContext {
    pub day_ntl_vlm: String,
    pub funding: String,
    pub impact_pxs: Option<Vec<String>>,
    pub mark_px: String,
    pub mid_px: Option<String>,
    pub open_interest: String,
    pub oracle_px: String,
    pub premium: Option<String>,
    pub prev_day_px: String,
}

/// Response for frontendOpenOrders - open orders with extra frontend metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FrontendOpenOrder {
    pub coin: String,
    pub side: String,
    pub limit_px: String,
    pub sz: String,
    pub oid: u64,
    pub timestamp: u64,
    pub orig_sz: String,
    pub cloid: Option<String>,
    pub reduce_only: bool,
    pub order_type: String,
    pub tif: String,
    pub trigger_condition: String,
    pub is_trigger: bool,
    pub trigger_px: String,
    pub is_position_tpsl: bool,
    #[serde(default)]
    pub children: Option<Vec<FrontendOpenOrder>>,
}

/// Response for userFillsByTime - fills with aggregation options
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserFillByTime {
    pub closed_pnl: String,
    pub coin: String,
    pub crossed: bool,
    pub dir: String,
    pub hash: String,
    pub oid: u64,
    pub px: String,
    pub side: String,
    pub start_position: String,
    pub sz: String,
    pub time: u64,
    pub fee: String,
    pub fee_token: String,
    pub tid: u64,
    pub cloid: Option<String>,
}

/// Response for historicalOrders
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalOrder {
    pub order: BasicOrderInfo,
    pub status: String,
    pub status_timestamp: u64,
}

/// Response for subAccounts
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubAccount {
    pub sub_account_user: Address,
    pub name: String,
    pub master: Address,
    pub clearinghouse_state: Option<SubAccountClearinghouseState>,
}

/// Clearinghouse state for sub-account
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountClearinghouseState {
    pub margin_summary: MarginSummary,
    pub cross_margin_summary: MarginSummary,
    pub withdrawable: String,
}

/// Response for userRateLimit
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRateLimit {
    pub cum_vlm: String,
    pub n_request_ids: u32,
    pub n_request_weights: u32,
    pub n_request_ids_limit: u32,
    pub n_request_weights_limit: u32,
}

/// Response for userVaultEquities
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultEquity {
    pub vault_address: Address,
    pub equity: String,
}

// ==================== Phase 2 New Types ====================

/// Response for portfolio - comprehensive portfolio performance data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Portfolio {
    /// Account value in USD
    pub account_value: String,
    /// Total notional position value
    pub total_ntl_pos: String,
    /// Total margin used
    pub total_margin_used: String,
    /// Total raw USD
    pub total_raw_usd: String,
    /// Withdrawable amount
    pub withdrawable: String,
    /// Cumulative PnL
    #[serde(default)]
    pub cum_pnl: Option<String>,
    /// Cumulative funding
    #[serde(default)]
    pub cum_funding: Option<String>,
    /// Cumulative volume
    #[serde(default)]
    pub cum_vlm: Option<String>,
}

/// Response for userNonFundingLedgerUpdates - ledger activity
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NonFundingLedgerUpdate {
    pub time: u64,
    pub hash: String,
    pub delta: NonFundingDelta,
}

/// Delta type for non-funding ledger updates
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NonFundingDelta {
    /// Deposit to account
    Deposit { usdc: String },
    /// Withdrawal from account
    Withdraw {
        usdc: String,
        nonce: u64,
        fee: String,
    },
    /// Internal transfer between accounts
    InternalTransfer {
        usdc: String,
        user: Address,
        destination: Address,
        fee: String,
    },
    /// Sub-account transfer
    SubAccountTransfer {
        usdc: String,
        user: Address,
        destination: Address,
    },
    /// Spot token transfer
    SpotTransfer {
        token: String,
        amount: String,
        user: Address,
        destination: Address,
        fee: String,
    },
    /// Liquidation
    Liquidation {
        liquidated_user: Address,
        #[serde(default)]
        leveraged_ntl: Option<String>,
    },
    /// Account class transfer (perp <-> spot)
    AccountClassTransfer { usdc: String, to_perp: bool },
    /// Spot genesis
    SpotGenesis { token: String, amount: String },
    /// Rewards claim
    RewardsClaim { amount: String },
    /// Vault deposit
    VaultDeposit { vault: Address, usdc: String },
    /// Vault withdrawal
    VaultWithdraw {
        vault: Address,
        usdc: String,
        #[serde(default)]
        fee: Option<String>,
    },
    /// Vault leader commission
    VaultLeaderCommission { usdc: String },
}

/// Response for extraAgents - list of additional authorized agents
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtraAgent {
    pub address: Address,
    pub name: Option<String>,
    #[serde(default)]
    pub valid_until: Option<u64>,
}

/// Response for userRole - user role and account type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRole {
    /// Role type (e.g., "user", "vault", "subAccount")
    pub role: String,
    /// Additional role data
    #[serde(default)]
    pub data: Option<UserRoleData>,
}

/// Additional data for user role
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleData {
    /// Master account for sub-accounts
    #[serde(default)]
    pub master: Option<Address>,
    /// Vault address for vault accounts
    #[serde(default)]
    pub vault: Option<Address>,
}

/// Response for tokenDetails - detailed token information
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenDetails {
    /// Token name/symbol
    pub name: String,
    /// Size decimals for trading
    pub sz_decimals: u32,
    /// Wei decimals for on-chain representation
    pub wei_decimals: u32,
    /// Token index
    pub index: u32,
    /// Token ID (hex string)
    pub token_id: String,
    /// Whether this is a canonical token
    pub is_canonical: bool,
    /// Full name of the token
    #[serde(default)]
    pub full_name: Option<String>,
    /// EVM contract information
    #[serde(default)]
    pub evm_contract: Option<EvmContract>,
    /// Deployer trading fee share
    #[serde(default)]
    pub deployer_trading_fee_share: Option<String>,
    /// Total supply
    #[serde(default)]
    pub total_supply: Option<String>,
    /// Circulating supply
    #[serde(default)]
    pub circulating_supply: Option<String>,
    /// Market cap
    #[serde(default)]
    pub market_cap: Option<String>,
}

// ==================== Phase 3 New Types ====================

// --- Staking/Delegation Types ---

/// Response for delegatorSummary - staking summary
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorSummary {
    /// Total delegated amount in wei
    #[serde(default)]
    pub delegated: Option<String>,
    /// Total undelegating amount in wei
    #[serde(default)]
    pub undelegating: Option<String>,
    /// Total rewards earned
    #[serde(default)]
    pub total_rewards: Option<String>,
    /// Pending rewards to claim
    #[serde(default)]
    pub pending_rewards: Option<String>,
    /// Number of validators delegated to
    #[serde(default)]
    pub n_validators: Option<u32>,
}

/// Response for delegations - list of staking delegations
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Delegation {
    /// Validator address
    pub validator: Address,
    /// Delegated amount in wei
    pub amount: String,
    /// Locked until timestamp (for undelegating)
    #[serde(default)]
    pub locked_until: Option<u64>,
    /// Pending rewards
    #[serde(default)]
    pub pending_rewards: Option<String>,
}

/// Response for delegatorRewards - historic staking rewards
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorReward {
    /// Timestamp of reward
    pub time: u64,
    /// Validator address
    pub validator: Address,
    /// Reward amount
    pub amount: String,
    /// Transaction hash
    #[serde(default)]
    pub hash: Option<String>,
}

/// Response for delegatorHistory - comprehensive staking history
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorHistoryEntry {
    /// Timestamp of action
    pub time: u64,
    /// Type of action (delegate, undelegate, claim, etc.)
    #[serde(rename = "type")]
    pub action_type: String,
    /// Validator address
    #[serde(default)]
    pub validator: Option<Address>,
    /// Amount in wei
    #[serde(default)]
    pub amount: Option<String>,
    /// Transaction hash
    #[serde(default)]
    pub hash: Option<String>,
}

// --- Deployment Types ---

/// Response for perpDeployAuctionStatus
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerpDeployAuctionStatus {
    /// Current auction state
    #[serde(default)]
    pub state: Option<String>,
    /// Auction info
    #[serde(default)]
    pub auction: Option<PerpDeployAuction>,
    /// DEX info
    #[serde(default)]
    pub dexes: Option<Vec<PerpDex>>,
}

/// Perp deployment auction info
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerpDeployAuction {
    /// Coin being auctioned
    #[serde(default)]
    pub coin: Option<String>,
    /// Starting price
    #[serde(default)]
    pub start_px: Option<String>,
    /// Current bid
    #[serde(default)]
    pub current_bid: Option<String>,
    /// End time
    #[serde(default)]
    pub end_time: Option<u64>,
}

/// Response for spotDeployState
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployState {
    /// Token deploy state
    #[serde(default)]
    pub tokens: Option<Vec<SpotTokenDeployState>>,
    /// User's deploy state
    #[serde(default)]
    pub user_state: Option<SpotUserDeployState>,
}

/// Spot token deployment state
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotTokenDeployState {
    /// Token name
    pub token: String,
    /// Deployment state
    pub state: String,
    /// Genesis info
    #[serde(default)]
    pub genesis: Option<SpotGenesisInfo>,
}

/// Spot genesis info
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotGenesisInfo {
    /// Max supply
    #[serde(default)]
    pub max_supply: Option<String>,
    /// Hyperliquidity enabled
    #[serde(default)]
    pub hyperliquidity: Option<bool>,
}

/// Spot user deployment state
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotUserDeployState {
    /// Tokens being deployed by user
    #[serde(default)]
    pub deploying: Option<Vec<String>>,
    /// Tokens deployed by user
    #[serde(default)]
    pub deployed: Option<Vec<String>>,
}

/// Response for spotPairDeployAuctionStatus
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotPairDeployAuctionStatus {
    /// Base token
    #[serde(default)]
    pub base: Option<String>,
    /// Quote token
    #[serde(default)]
    pub quote: Option<String>,
    /// Auction state
    #[serde(default)]
    pub state: Option<String>,
    /// Current bid
    #[serde(default)]
    pub current_bid: Option<String>,
    /// End time
    #[serde(default)]
    pub end_time: Option<u64>,
}

// --- Other Types ---

/// Response for perpDexs - available perpetual DEXs
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerpDex {
    /// DEX identifier
    pub dex: u32,
    /// DEX name
    pub name: String,
    /// Coins listed on this DEX
    #[serde(default)]
    pub coins: Option<Vec<String>>,
}

/// Response for userDexAbstraction
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDexAbstraction {
    /// Whether DEX abstraction is enabled
    pub enabled: bool,
    /// Agent address
    #[serde(default)]
    pub agent: Option<Address>,
    /// Enabled dexes
    #[serde(default)]
    pub dexes: Option<Vec<u32>>,
}

/// Response for userToMultiSigSigners
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiSigSignerInfo {
    /// Signer address
    pub address: Address,
    /// Signer weight
    pub weight: u32,
}

/// Multi-sig user info
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiSigUserInfo {
    /// Threshold required for approval
    pub threshold: u32,
    /// List of signers
    pub signers: Vec<MultiSigSignerInfo>,
}

/// Response for userTwapSliceFills - TWAP execution fills
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TwapSliceFill {
    /// TWAP order ID
    pub twap_id: u64,
    /// Asset index
    pub asset: u32,
    /// Coin name
    pub coin: String,
    /// Fill price
    pub px: String,
    /// Fill size
    pub sz: String,
    /// Side (buy/sell)
    pub side: String,
    /// Fill time
    pub time: u64,
    /// Transaction hash
    #[serde(default)]
    pub hash: Option<String>,
}
