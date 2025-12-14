//! Tests for action types and EIP-712 signing
//!
//! Tests cover:
//! - EIP-712 type hashes for various action types
//! - Serialization format for actions
//! - Struct hash computation

#[cfg(test)]
mod tests {
    use alloy::primitives::{address, keccak256, B256};
    use hyperliquid_rust_sdk::types::actions::{
        ApproveAgent, ApproveBuilderFee, BulkCancel, BulkModify, BulkOrder,
        ClassTransfer, CreateSubAccount, ScheduleCancel, SetReferrer, SpotSend, SpotUser,
        SubAccountSpotTransfer, SubAccountTransfer, TwapCancel, TwapOrder,
        UpdateIsolatedMargin, UpdateLeverage, UsdSend, VaultTransfer, Withdraw,
    };
    use hyperliquid_rust_sdk::types::eip712::HyperliquidAction;
    use hyperliquid_rust_sdk::types::requests::{
        CancelRequest, ModifyRequest, OrderRequest,
    };

    // ==================== UsdSend Tests ====================

    #[test]
    fn test_usd_send_type_hash() {
        let expected = keccak256(
            "HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)",
        );
        assert_eq!(UsdSend::type_hash(), expected);
    }

    #[test]
    fn test_usd_send_serialization() {
        let action = UsdSend {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0x1234567890123456789012345678901234567890".to_string(),
            amount: "100.5".to_string(),
            time: 1690393044548,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["hyperliquidChain"].as_str().unwrap(), "Testnet");
        assert_eq!(
            parsed["destination"].as_str().unwrap(),
            "0x1234567890123456789012345678901234567890"
        );
        assert_eq!(parsed["amount"].as_str().unwrap(), "100.5");
        assert_eq!(parsed["time"].as_u64().unwrap(), 1690393044548);
        // Chain ID is serialized as hex
        assert!(parsed["signatureChainId"]
            .as_str()
            .unwrap()
            .starts_with("0x"));
    }

    #[test]
    fn test_usd_send_struct_hash() {
        let action = UsdSend {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0x1234567890123456789012345678901234567890".to_string(),
            amount: "100".to_string(),
            time: 1690393044548,
        };

        let struct_hash = action.struct_hash();
        assert_ne!(struct_hash, B256::ZERO);
    }

    // ==================== Withdraw Tests ====================

    #[test]
    fn test_withdraw_type_hash() {
        let expected = keccak256(
            "HyperliquidTransaction:Withdraw(string hyperliquidChain,string destination,string amount,uint64 time)",
        );
        assert_eq!(Withdraw::type_hash(), expected);
    }

    #[test]
    fn test_withdraw_serialization() {
        let action = Withdraw {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0xabcdef".to_string(),
            amount: "50.0".to_string(),
            time: 1234567890,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["destination"].as_str().unwrap(), "0xabcdef");
        assert_eq!(parsed["amount"].as_str().unwrap(), "50.0");
    }

    #[test]
    fn test_withdraw_struct_hash() {
        let action = Withdraw {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0xabcdef".to_string(),
            amount: "50.0".to_string(),
            time: 1234567890,
        };

        let struct_hash = action.struct_hash();
        assert_ne!(struct_hash, B256::ZERO);
    }

    // ==================== SpotSend Tests ====================

    #[test]
    fn test_spot_send_type_hash() {
        let expected = keccak256(
            "HyperliquidTransaction:SpotSend(string hyperliquidChain,string destination,string token,string amount,uint64 time)",
        );
        assert_eq!(SpotSend::type_hash(), expected);
    }

    #[test]
    fn test_spot_send_serialization() {
        let action = SpotSend {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0xdestination".to_string(),
            token: "HYPE".to_string(),
            amount: "1000".to_string(),
            time: 1234567890,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["token"].as_str().unwrap(), "HYPE");
        assert_eq!(parsed["amount"].as_str().unwrap(), "1000");
    }

    // ==================== ApproveAgent Tests (expanded from existing) ====================

    #[test]
    fn test_approve_agent_type_hash() {
        let expected = keccak256(
            "HyperliquidTransaction:ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)",
        );
        assert_eq!(ApproveAgent::type_hash(), expected);
    }

    #[test]
    fn test_approve_agent_with_name() {
        let action = ApproveAgent {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            agent_address: address!("1234567890123456789012345678901234567890"),
            agent_name: Some("My Agent".to_string()),
            nonce: 1234567890,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["agentName"].as_str().unwrap(), "My Agent");
    }

    #[test]
    fn test_approve_agent_without_name() {
        let action = ApproveAgent {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            agent_address: address!("1234567890123456789012345678901234567890"),
            agent_name: None,
            nonce: 1234567890,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed["agentName"].is_null());
    }

    #[test]
    fn test_approve_agent_address_format() {
        let action = ApproveAgent {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            agent_address: address!("1234567890123456789012345678901234567890"),
            agent_name: None,
            nonce: 1234567890,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Should be lowercase hex with 0x prefix
        let addr = parsed["agentAddress"].as_str().unwrap();
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42); // 0x + 40 hex chars
    }

    // ==================== ApproveBuilderFee Tests ====================

    #[test]
    fn test_approve_builder_fee_type_hash() {
        let expected = keccak256(
            "HyperliquidTransaction:ApproveBuilderFee(string hyperliquidChain,string maxFeeRate,string builder,uint64 nonce)",
        );
        assert_eq!(ApproveBuilderFee::type_hash(), expected);
    }

    #[test]
    fn test_approve_builder_fee_serialization() {
        let action = ApproveBuilderFee {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            max_fee_rate: "0.001".to_string(),
            builder: "0xbuilder".to_string(),
            nonce: 1234567890,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["maxFeeRate"].as_str().unwrap(), "0.001");
        assert_eq!(parsed["builder"].as_str().unwrap(), "0xbuilder");
    }

    // ==================== UpdateLeverage Tests ====================

    #[test]
    fn test_update_leverage_serialization() {
        let action = UpdateLeverage {
            asset: 0,
            is_cross: true,
            leverage: 10,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["asset"].as_u64().unwrap(), 0);
        assert!(parsed["isCross"].as_bool().unwrap());
        assert_eq!(parsed["leverage"].as_u64().unwrap(), 10);
    }

    #[test]
    fn test_update_leverage_isolated() {
        let action = UpdateLeverage {
            asset: 1,
            is_cross: false,
            leverage: 5,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(!parsed["isCross"].as_bool().unwrap());
    }

    // ==================== UpdateIsolatedMargin Tests ====================

    #[test]
    fn test_update_isolated_margin_serialization() {
        let action = UpdateIsolatedMargin {
            asset: 0,
            is_buy: true,
            ntli: 1000,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["asset"].as_u64().unwrap(), 0);
        assert!(parsed["isBuy"].as_bool().unwrap());
        assert_eq!(parsed["ntli"].as_i64().unwrap(), 1000);
    }

    #[test]
    fn test_update_isolated_margin_negative() {
        let action = UpdateIsolatedMargin {
            asset: 0,
            is_buy: false,
            ntli: -500,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["ntli"].as_i64().unwrap(), -500);
    }

    // ==================== VaultTransfer Tests ====================

    #[test]
    fn test_vault_transfer_deposit() {
        let action = VaultTransfer {
            vault_address: "0xvault".to_string(),
            is_deposit: true,
            usd: 10000,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["vaultAddress"].as_str().unwrap(), "0xvault");
        assert!(parsed["isDeposit"].as_bool().unwrap());
        assert_eq!(parsed["usd"].as_u64().unwrap(), 10000);
    }

    #[test]
    fn test_vault_transfer_withdraw() {
        let action = VaultTransfer {
            vault_address: "0xvault".to_string(),
            is_deposit: false,
            usd: 5000,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(!parsed["isDeposit"].as_bool().unwrap());
    }

    // ==================== SpotUser / ClassTransfer Tests ====================

    #[test]
    fn test_spot_user_class_transfer() {
        let action = SpotUser {
            class_transfer: ClassTransfer {
                usd_size: 1000,
                to_perp: true,
            },
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        let transfer = &parsed["classTransfer"];
        assert_eq!(transfer["usdSize"].as_u64().unwrap(), 1000);
        assert!(transfer["toPerp"].as_bool().unwrap());
    }

    // ==================== SetReferrer Tests ====================

    #[test]
    fn test_set_referrer_serialization() {
        let action = SetReferrer {
            code: "MYCODE123".to_string(),
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["code"].as_str().unwrap(), "MYCODE123");
    }

    // ==================== BulkOrder Tests ====================

    #[test]
    fn test_bulk_order_serialization() {
        let orders = vec![OrderRequest::limit(0, true, "50000", "0.01", "Gtc")];

        let action = BulkOrder {
            orders,
            grouping: "na".to_string(),
            builder: None,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed["orders"].is_array());
        assert_eq!(parsed["grouping"].as_str().unwrap(), "na");
        // builder should be omitted when None
        assert!(parsed.get("builder").is_none() || parsed["builder"].is_null());
    }

    #[test]
    fn test_bulk_order_multiple_orders() {
        let orders = vec![
            OrderRequest::limit(0, true, "50000", "0.01", "Gtc"),
            OrderRequest::limit(0, false, "51000", "0.02", "Gtc"),
            OrderRequest::limit(1, true, "3000", "0.1", "Ioc"),
        ];

        let action = BulkOrder {
            orders,
            grouping: "na".to_string(),
            builder: None,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["orders"].as_array().unwrap().len(), 3);
    }

    // ==================== BulkCancel Tests ====================

    #[test]
    fn test_bulk_cancel_serialization() {
        let cancels = vec![CancelRequest::new(0, 12345), CancelRequest::new(0, 12346)];

        let action = BulkCancel { cancels };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed["cancels"].is_array());
        assert_eq!(parsed["cancels"].as_array().unwrap().len(), 2);
    }

    // ==================== BulkModify Tests ====================

    #[test]
    fn test_bulk_modify_serialization() {
        let order = OrderRequest::limit(0, true, "51000", "0.02", "Gtc");
        let modifies = vec![ModifyRequest { oid: 12345, order }];

        let action = BulkModify { modifies };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed["modifies"].is_array());
    }

    // ==================== ScheduleCancel Tests ====================

    #[test]
    fn test_schedule_cancel_with_time() {
        let action = ScheduleCancel {
            time: Some(1690393044548),
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["time"].as_u64().unwrap(), 1690393044548);
    }

    #[test]
    fn test_schedule_cancel_without_time() {
        let action = ScheduleCancel { time: None };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed["time"].is_null());
    }

    // ==================== CreateSubAccount Tests ====================

    #[test]
    fn test_create_sub_account_with_name() {
        let action = CreateSubAccount {
            name: Some("Trading Account".to_string()),
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["name"].as_str().unwrap(), "Trading Account");
    }

    #[test]
    fn test_create_sub_account_without_name() {
        let action = CreateSubAccount { name: None };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed["name"].is_null());
    }

    // ==================== SubAccountTransfer Tests ====================

    #[test]
    fn test_sub_account_transfer_deposit() {
        let action = SubAccountTransfer {
            sub_account_user: "0xsubaccount".to_string(),
            is_deposit: true,
            usd: 5000,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["subAccountUser"].as_str().unwrap(), "0xsubaccount");
        assert!(parsed["isDeposit"].as_bool().unwrap());
        assert_eq!(parsed["usd"].as_u64().unwrap(), 5000);
    }

    // ==================== SubAccountSpotTransfer Tests ====================

    #[test]
    fn test_sub_account_spot_transfer() {
        let action = SubAccountSpotTransfer {
            sub_account_user: "0xsubaccount".to_string(),
            is_deposit: true,
            token: "HYPE".to_string(),
            amount: "100.5".to_string(),
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["token"].as_str().unwrap(), "HYPE");
        assert_eq!(parsed["amount"].as_str().unwrap(), "100.5");
    }

    // ==================== TwapOrder Tests ====================

    #[test]
    fn test_twap_order_serialization() {
        let action = TwapOrder {
            asset: 0,
            is_buy: true,
            sz: "1.0".to_string(),
            reduce_only: false,
            duration_minutes: 60,
            randomize: true,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Uses short field names
        assert_eq!(parsed["a"].as_u64().unwrap(), 0);
        assert!(parsed["b"].as_bool().unwrap());
        assert_eq!(parsed["s"].as_str().unwrap(), "1.0");
        assert!(!parsed["r"].as_bool().unwrap());
        assert_eq!(parsed["m"].as_u64().unwrap(), 60);
        assert!(parsed["t"].as_bool().unwrap());
    }

    // ==================== TwapCancel Tests ====================

    #[test]
    fn test_twap_cancel_serialization() {
        let action = TwapCancel {
            asset: 0,
            twap_id: 12345,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["a"].as_u64().unwrap(), 0);
        assert_eq!(parsed["t"].as_u64().unwrap(), 12345);
    }

    // ==================== Chain ID Serialization Tests ====================

    #[test]
    fn test_chain_id_hex_format_mainnet() {
        let action = UsdSend {
            signature_chain_id: 42161, // Arbitrum mainnet
            hyperliquid_chain: "Mainnet".to_string(),
            destination: "0xtest".to_string(),
            amount: "100".to_string(),
            time: 1234567890,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        let chain_id = parsed["signatureChainId"].as_str().unwrap();
        assert_eq!(chain_id, "0xa4b1"); // 42161 in hex
    }

    #[test]
    fn test_chain_id_hex_format_testnet() {
        let action = UsdSend {
            signature_chain_id: 421614, // Arbitrum Sepolia
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0xtest".to_string(),
            amount: "100".to_string(),
            time: 1234567890,
        };

        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        let chain_id = parsed["signatureChainId"].as_str().unwrap();
        assert_eq!(chain_id, "0x66eee"); // 421614 in hex
    }
}
