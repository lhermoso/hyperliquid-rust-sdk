use alloy::primitives::B256;
use serde;

use crate::l1_action;
use crate::types::requests::{
    BuilderInfo, CancelRequest, CancelRequestCloid, ModifyRequest, OrderRequest,
};

// User Actions (with HyperliquidTransaction: prefix)

// UsdSend needs custom serialization for signature_chain_id
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsdSend {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

impl crate::types::eip712::HyperliquidAction for UsdSend {
    const TYPE_STRING: &'static str =
        "UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.destination)[..]);
        encoded.extend_from_slice(&encode_value(&self.amount)[..]);
        encoded.extend_from_slice(&encode_value(&self.time)[..]);
        encoded
    }
}

// Withdraw needs custom serialization for signature_chain_id
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

impl crate::types::eip712::HyperliquidAction for Withdraw {
    const TYPE_STRING: &'static str =
        "Withdraw(string hyperliquidChain,string destination,string amount,uint64 time)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.destination)[..]);
        encoded.extend_from_slice(&encode_value(&self.amount)[..]);
        encoded.extend_from_slice(&encode_value(&self.time)[..]);
        encoded
    }
}

// SpotSend needs custom serialization for signature_chain_id
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotSend {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub token: String,
    pub amount: String,
    pub time: u64,
}

impl crate::types::eip712::HyperliquidAction for SpotSend {
    const TYPE_STRING: &'static str = "SpotSend(string hyperliquidChain,string destination,string token,string amount,uint64 time)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.destination)[..]);
        encoded.extend_from_slice(&encode_value(&self.token)[..]);
        encoded.extend_from_slice(&encode_value(&self.amount)[..]);
        encoded.extend_from_slice(&encode_value(&self.time)[..]);
        encoded
    }
}

// ApproveAgent needs custom serialization for the address field
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveAgent {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    #[serde(serialize_with = "serialize_address")]
    pub agent_address: alloy::primitives::Address,
    pub agent_name: Option<String>,
    pub nonce: u64,
}

pub(crate) fn serialize_address<S>(
    address: &alloy::primitives::Address,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{:#x}", address))
}

pub(crate) fn serialize_chain_id<S>(
    chain_id: &u64,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Serialize as hex string to match SDK format
    serializer.serialize_str(&format!("{:#x}", chain_id))
}

impl crate::types::eip712::HyperliquidAction for ApproveAgent {
    const TYPE_STRING: &'static str = "ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.agent_address)[..]);
        // SDK uses unwrap_or_default() for agent_name
        let agent_name = self.agent_name.clone().unwrap_or_default();
        encoded.extend_from_slice(&encode_value(&agent_name)[..]);
        encoded.extend_from_slice(&encode_value(&self.nonce)[..]);
        encoded
    }
}

// ApproveBuilderFee needs custom serialization for signature_chain_id
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveBuilderFee {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub max_fee_rate: String,
    pub builder: String,
    pub nonce: u64,
}

impl crate::types::eip712::HyperliquidAction for ApproveBuilderFee {
    const TYPE_STRING: &'static str = "ApproveBuilderFee(string hyperliquidChain,string maxFeeRate,string builder,uint64 nonce)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.max_fee_rate)[..]);
        encoded.extend_from_slice(&encode_value(&self.builder)[..]);
        encoded.extend_from_slice(&encode_value(&self.nonce)[..]);
        encoded
    }
}

// L1 Actions (use Exchange domain)

l1_action! {
    /// Agent connection action
    struct Agent {
        pub source: String,
        pub connection_id: B256,
    }
    => "Agent(string source,bytes32 connectionId)"
    => encode(source, connection_id)
}

// Exchange Actions (these don't need EIP-712 signing but are included for completeness)

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverage {
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u32,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIsolatedMargin {
    pub asset: u32,
    pub is_buy: bool,
    pub ntli: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransfer {
    pub vault_address: String,
    pub is_deposit: bool,
    pub usd: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotUser {
    pub class_transfer: ClassTransfer,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassTransfer {
    pub usd_size: u64,
    pub to_perp: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetReferrer {
    pub code: String,
}

// Bulk actions that contain other types

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder {
    pub orders: Vec<OrderRequest>,
    pub grouping: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancel {
    pub cancels: Vec<CancelRequest>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkModify {
    pub modifies: Vec<ModifyRequest>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancelCloid {
    pub cancels: Vec<CancelRequestCloid>,
}

// ==================== Phase 1 New Actions ====================

/// Schedule automatic order cancellation
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleCancel {
    pub time: Option<u64>,
}

/// Create a sub-account
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccount {
    pub name: Option<String>,
}

/// Transfer USD to/from a sub-account
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTransfer {
    pub sub_account_user: String,
    pub is_deposit: bool,
    pub usd: u64,
}

/// Transfer spot tokens to/from a sub-account
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountSpotTransfer {
    pub sub_account_user: String,
    pub is_deposit: bool,
    pub token: String,
    pub amount: String,
}

/// Transfer USD between perp and spot classes (different from spotUser classTransfer)
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsdClassTransfer {
    pub amount: String,
    pub to_perp: bool,
}

// ==================== Phase 2 New Actions ====================

/// TWAP order request
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapOrder {
    /// Asset index
    #[serde(rename = "a")]
    pub asset: u32,
    /// Is buy order
    #[serde(rename = "b")]
    pub is_buy: bool,
    /// Size to execute
    #[serde(rename = "s")]
    pub sz: String,
    /// Reduce only flag
    #[serde(rename = "r")]
    pub reduce_only: bool,
    /// Duration in minutes
    #[serde(rename = "m")]
    pub duration_minutes: u32,
    /// Randomize execution
    #[serde(rename = "t")]
    pub randomize: bool,
}

/// Bulk TWAP order wrapper
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkTwapOrder {
    pub twap: TwapOrder,
}

/// Cancel TWAP order
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapCancel {
    /// Asset index
    #[serde(rename = "a")]
    pub asset: u32,
    /// TWAP order ID
    #[serde(rename = "t")]
    pub twap_id: u64,
}

/// Convert account to multi-sig user
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertToMultiSigUser {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    /// Sorted list of authorized user addresses
    pub signers: Vec<MultiSigSigner>,
    /// Required number of signatures
    pub threshold: u32,
    pub nonce: u64,
}

/// Multi-sig signer information
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSigSigner {
    pub address: String,
    pub weight: u32,
}

impl crate::types::eip712::HyperliquidAction for ConvertToMultiSigUser {
    const TYPE_STRING: &'static str =
        "ConvertToMultiSigUser(string hyperliquidChain,address[] authorizedUsers,uint32 threshold,uint64 nonce)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        use alloy::primitives::keccak256;

        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);

        // Encode array of addresses
        let mut addresses_encoded = Vec::new();
        for signer in &self.signers {
            // Parse address and encode
            if let Ok(addr) = signer.address.parse::<alloy::primitives::Address>() {
                addresses_encoded.extend_from_slice(&encode_value(&addr)[..]);
            }
        }
        let addresses_hash = keccak256(&addresses_encoded);
        encoded.extend_from_slice(&addresses_hash[..]);

        encoded.extend_from_slice(&encode_value(&(self.threshold as u64))[..]);
        encoded.extend_from_slice(&encode_value(&self.nonce)[..]);
        encoded
    }
}

/// Execute a multi-sig transaction
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSig {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    /// The multi-sig user address
    pub multi_sig_user: String,
    /// The outer signer (one of the authorized users)
    pub outer_signer: String,
    /// The inner action to execute
    pub inner_action: serde_json::Value,
    /// Signatures from other authorized users
    pub signatures: Vec<MultiSigSignature>,
    pub nonce: u64,
}

/// Signature for multi-sig transaction
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSigSignature {
    pub r: String,
    pub s: String,
    pub v: u8,
}

/// Enable DEX abstraction for an agent
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentEnableDexAbstraction {
    // This action has no additional fields - just the type
}

// ==================== Phase 3 New Actions ====================

// --- Spot Deployment Actions ---

/// Register a new spot token
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployRegisterToken {
    /// Token name/symbol
    pub token_name: String,
    /// Size decimals for trading
    pub sz_decimals: u32,
    /// Wei decimals for on-chain representation
    pub wei_decimals: u32,
    /// Maximum gas for deployment
    pub max_gas: String,
    /// Full name of the token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

/// User genesis for spot deployment
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployUserGenesis {
    /// Token identifier
    pub token: String,
    /// List of (user address, wei amount) tuples for initial distribution
    pub user_and_wei: Vec<(String, String)>,
    /// Existing token and wei to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_token_and_wei: Option<(String, String)>,
}

/// Freeze or unfreeze a user in spot deployment
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployFreezeUser {
    /// Token identifier
    pub token: String,
    /// User address to freeze/unfreeze
    pub user: String,
    /// Whether to freeze (true) or unfreeze (false)
    pub freeze: bool,
}

/// Enable freeze privilege for a token
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployEnableFreezePrivilege {
    /// Token identifier
    pub token: String,
}

/// Revoke freeze privilege for a token
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployRevokeFreezePrivilege {
    /// Token identifier
    pub token: String,
}

/// Enable quote token for spot deployment
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployEnableQuoteToken {
    /// Token identifier to enable as quote
    pub token: String,
}

/// Genesis for spot deployment
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployGenesis {
    /// Token identifier
    pub token: String,
    /// Maximum supply
    pub max_supply: String,
    /// Whether to disable hyperliquidity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_hyperliquidity: Option<bool>,
}

/// Register a spot trading pair
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployRegisterSpot {
    /// Base token identifier
    pub base_token: String,
    /// Quote token identifier
    pub quote_token: String,
}

/// Register hyperliquidity for a spot pair
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeployRegisterHyperliquidity {
    /// Spot pair identifier
    pub spot: String,
    /// Starting price
    pub start_px: String,
    /// Order size
    pub order_sz: String,
    /// Number of orders
    pub n_orders: u32,
    /// Number of seeded levels
    pub n_seeded_levels: u32,
}

/// Set deployer trading fee share for a token
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotDeploySetDeployerTradingFeeShare {
    /// Token identifier
    pub token: String,
    /// Fee share (as decimal string, e.g., "0.001" for 0.1%)
    pub share: String,
}

// --- Perp Deployment Actions ---

/// Register a perpetual asset
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpDeployRegisterAsset {
    /// DEX identifier
    pub dex: u32,
    /// Maximum gas for deployment
    pub max_gas: String,
    /// Coin name/symbol
    pub coin: String,
    /// Size decimals for trading
    pub sz_decimals: u32,
    /// Oracle price
    pub oracle_px: String,
    /// Margin table ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_table_id: Option<u32>,
    /// Whether to use isolated margin only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_isolated: Option<bool>,
    /// Schema type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// Set oracle for perpetual asset
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpDeploySetOracle {
    /// DEX identifier
    pub dex: u32,
    /// Oracle prices
    pub oracle_pxs: Vec<String>,
    /// All mark prices
    pub all_mark_pxs: Vec<String>,
    /// External perp prices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_perp_pxs: Option<Vec<String>>,
}

// --- Validator/Staking Actions ---

/// Unjail self (signer)
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CSignerUnjailSelf {
    // No additional fields - just the action type
}

/// Jail self (signer)
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CSignerJailSelf {
    // No additional fields - just the action type
}

/// Register as a validator
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CValidatorRegister {
    /// Node IP address
    pub node_ip: String,
    /// Validator name
    pub name: String,
    /// Validator description
    pub description: String,
    /// Whether delegations are disabled
    pub delegations_disabled: bool,
    /// Commission in basis points
    pub commission_bps: u32,
    /// Signer address
    pub signer: String,
    /// Whether initially unjailed
    pub unjailed: bool,
    /// Initial wei stake
    pub initial_wei: String,
}

/// Change validator profile
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CValidatorChangeProfile {
    /// Node IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ip: Option<String>,
    /// Validator name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Validator description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether unjailed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unjailed: Option<bool>,
    /// Whether to disable delegations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_delegations: Option<bool>,
    /// Commission in basis points
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_bps: Option<u32>,
    /// Signer address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<String>,
}

/// Unregister as a validator
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CValidatorUnregister {
    // No additional fields - just the action type
}

/// Delegate tokens to a validator
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDelegate {
    /// Validator address to delegate to
    pub validator: String,
    /// Amount in wei
    pub wei: String,
    /// Whether this is an undelegation
    pub is_undelegate: bool,
}

// --- Other Actions ---

/// Enable or disable large block mode
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseBigBlocks {
    /// Whether to enable (true) or disable (false) big blocks
    pub enable: bool,
}

/// No-operation action (useful for testing or keeping connection alive)
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Noop {
    /// Nonce for the action
    pub nonce: u64,
}

// Types are now imported from requests.rs

// The macros don't handle signature_chain_id, so we need to remove the duplicate trait impls

#[cfg(test)]
mod tests {
    use alloy::primitives::keccak256;

    use super::*;
    use crate::types::eip712::HyperliquidAction;

    #[test]
    fn test_usd_send_type_hash() {
        let expected = keccak256(
            "HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)",
        );
        assert_eq!(UsdSend::type_hash(), expected);
    }

    #[test]
    fn test_agent_type_hash() {
        // L1 actions don't use the HyperliquidTransaction: prefix
        let expected = keccak256("Agent(string source,bytes32 connectionId)");
        assert_eq!(Agent::type_hash(), expected);
    }

    #[test]
    fn test_agent_domain() {
        let agent = Agent {
            source: "a".to_string(),
            connection_id: B256::default(),
        };

        // L1 actions use the Exchange domain
        let domain = agent.domain();
        let expected_domain = alloy::sol_types::eip712_domain! {
            name: "Exchange",
            version: "1",
            chain_id: 1337u64,
            verifying_contract: alloy::primitives::address!("0000000000000000000000000000000000000000"),
        };

        // Compare domain separators to verify they're the same
        assert_eq!(domain.separator(), expected_domain.separator());
    }
}
