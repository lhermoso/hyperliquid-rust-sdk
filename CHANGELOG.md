# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Info API (Phase 1)
- `meta_and_asset_ctxs()` - Get perpetual metadata with asset contexts
- `frontend_open_orders(user)` - Get open orders with extra frontend metadata
- `user_fills_by_time(user, start, end, aggregate)` - Get fills within a time range
- `historical_orders(user)` - Get user's historical orders
- `sub_accounts(user)` - Get sub-account information
- `user_rate_limit(user)` - Get API rate limit configuration
- `user_vault_equities(user)` - Get vault equity positions

#### Info API (Phase 2)
- `portfolio(user)` - Get comprehensive portfolio performance data
- `user_non_funding_ledger_updates(user, start, end)` - Get ledger activity (deposits, withdrawals, transfers)
- `extra_agents(user)` - Get list of additional authorized agents
- `user_role(user)` - Get user role and account type information
- `token_details(token_id)` - Get detailed token information

#### Exchange API (Phase 1)
- `schedule_cancel(time)` - Schedule automatic order cancellation
- `create_sub_account(name)` - Create a sub-account
- `sub_account_transfer(sub_account, is_deposit, usd)` - Transfer USD to/from sub-account
- `sub_account_spot_transfer(sub_account, is_deposit, token, amount)` - Transfer spot tokens to/from sub-account
- `usd_class_transfer(amount, to_perp)` - Transfer USD between perp and spot classes

#### Exchange API (Phase 2)
- `twap_order(asset, is_buy, sz, reduce_only, duration, randomize)` - Place a TWAP order
- `twap_cancel(asset, twap_id)` - Cancel a TWAP order
- `convert_to_multi_sig_user(users, threshold)` - Convert account to multi-sig
- `multi_sig(user, action, signatures)` - Execute a multi-sig transaction
- `agent_enable_dex_abstraction()` - Enable DEX abstraction for agent

#### WebSocket (Phase 1)
- `subscribe_bbo(coin)` - Subscribe to best bid/offer updates
- `subscribe_open_orders(user)` - Subscribe to real-time open orders
- `subscribe_clearinghouse_state(user)` - Subscribe to real-time clearinghouse state

#### WebSocket (Phase 2)
- `subscribe_web_data3(user)` - Subscribe to aggregate user information (newer version)
- `subscribe_twap_states(user)` - Subscribe to TWAP order states
- `subscribe_active_asset_ctx(coin)` - Subscribe to active asset context updates
- `subscribe_active_asset_data(user, coin)` - Subscribe to active asset data (perps)
- `subscribe_user_twap_slice_fills(user)` - Subscribe to TWAP slice fills
- `subscribe_user_twap_history(user)` - Subscribe to TWAP order history

#### Exchange API (Phase 3) - Spot Deployment
- `spot_deploy_register_token(name, sz_decimals, wei_decimals, max_gas, full_name)` - Register a new spot token
- `spot_deploy_user_genesis(token, user_and_wei, existing_token_and_wei)` - User genesis for spot deployment
- `spot_deploy_freeze_user(token, user, freeze)` - Freeze/unfreeze user in spot deployment
- `spot_deploy_enable_freeze_privilege(token)` - Enable freeze privilege for token
- `spot_deploy_revoke_freeze_privilege(token)` - Revoke freeze privilege for token
- `spot_deploy_enable_quote_token(token)` - Enable quote token for spot deployment
- `spot_deploy_genesis(token, max_supply, no_hyperliquidity)` - Genesis for spot deployment
- `spot_deploy_register_spot(base_token, quote_token)` - Register a spot trading pair
- `spot_deploy_register_hyperliquidity(spot, start_px, order_sz, n_orders, n_seeded_levels)` - Register hyperliquidity
- `spot_deploy_set_deployer_trading_fee_share(token, share)` - Set deployer trading fee share

#### Exchange API (Phase 3) - Perp Deployment
- `perp_deploy_register_asset(dex, max_gas, coin, sz_decimals, oracle_px, ...)` - Register a perpetual asset
- `perp_deploy_set_oracle(dex, oracle_pxs, all_mark_pxs, external_perp_pxs)` - Set oracle for perp asset

#### Exchange API (Phase 3) - Validator/Staking
- `c_signer_unjail_self()` - Unjail self (signer)
- `c_signer_jail_self()` - Jail self (signer)
- `c_validator_register(node_ip, name, description, ...)` - Register as a validator
- `c_validator_change_profile(...)` - Change validator profile
- `c_validator_unregister()` - Unregister as a validator
- `token_delegate(validator, wei, is_undelegate)` - Delegate/undelegate tokens to validator

#### Exchange API (Phase 3) - Other
- `use_big_blocks(enable)` - Enable/disable large block mode
- `noop(nonce)` - No-operation action

#### Info API (Phase 3) - Staking/Delegation
- `delegator_summary(user)` - Get staking summary
- `delegations(user)` - Get staking delegations
- `delegator_rewards(user)` - Get historic staking rewards
- `delegator_history(user)` - Get comprehensive staking history

#### Info API (Phase 3) - Deployment
- `perp_deploy_auction_status()` - Get perp deployment auction status
- `spot_deploy_state(user)` - Get spot deployment state
- `spot_pair_deploy_auction_status(base, quote)` - Get spot pair deployment auction status

#### Info API (Phase 3) - Other
- `perp_dexs()` - Get available perpetual DEXs
- `user_dex_abstraction(user)` - Get DEX abstraction state
- `user_to_multi_sig_signers(multi_sig_user)` - Get multi-sig signers
- `user_twap_slice_fills(user)` - Get TWAP slice fills

#### Documentation
- Created `docs/API_AUDIT.md` with comprehensive API coverage analysis
- Documented Phase 2 and Phase 3 implementation plans for future work
- Updated README with fork attribution to ControlCplusControlV/ferrofluid and new package name ([#2](https://github.com/lhermoso/hyperliquid-rust-sdk/pull/2))
- Updated API_AUDIT.md to reflect Phase 2 completion status

#### Examples
- Updated `02_info_provider.rs` with Phase 2 & 3 Info API endpoints coverage
- Updated `03_exchange_provider.rs` with Phase 2 & 3 Exchange API examples
- Updated `04_websocket.rs` with Phase 2 WebSocket subscriptions (TWAP, webData3, active asset)
- Added `06_twap_orders.rs` - Complete TWAP order workflow example (placing, monitoring, cancelling)
- Added `07_multi_sig.rs` - Multi-signature account operations and configuration
- Added `08_staking_delegation.rs` - Token staking, delegation, and validator operations
- Added `09_spot_deployment.rs` - Complete spot token deployment workflow
- Added `10_perp_deployment.rs` - Perpetual asset deployment and oracle configuration
- Added `11_order_tracking.rs` - Order tracking system with automatic CLOID management

#### Tests
- Added comprehensive test coverage for SDK public API (from 11 to 210 tests)
- Added `tests/symbol_test.rs` - Symbol type creation, type detection, serialization (25 tests)
- Added `tests/request_types_test.rs` - OrderRequest, CancelRequest, ModifyRequest (30 tests)
- Added `tests/order_builder_test.rs` - OrderBuilder fluent API, validation (28 tests)
- Added `tests/action_types_test.rs` - EIP-712 type hashes, action serialization (36 tests)
- Added `tests/websocket_types_test.rs` - Subscription and message serialization (34 tests)
- Added `tests/info_provider_test.rs` - RateLimiter unit tests and optional live API tests (24 tests)

### Changed
- Added `Clone` derive to `MarginSummary` type
- Added `Clone` derive to `EvmContract` type
- Aligned library import name with crate name - now import as `hyperliquid_rust_sdk` instead of `ferrofluid` ([#10](https://github.com/lhermoso/hyperliquid-rust-sdk/pull/10))
- **BREAKING**: `Portfolio` type changed from a struct to `Vec<(String, PortfolioPeriodData)>` to match actual API response format which returns time-series data per period (day, week, month, allTime, etc.)
- **BREAKING**: `SpotMetaAndAssetCtxs` type restructured to correctly deserialize the API's `[{universe, tokens}, [...assetCtxs]]` tuple format; now has `meta: SpotMeta` and `asset_ctxs: Vec<SpotAssetContext>` fields
- Added `Clone` derive to `SpotMeta`, `SpotPairMeta`, and `TokenMeta` types

### Fixed
- Fixed `portfolio(user)` endpoint deserialization - API returns array of time period tuples, not a flat object
- Fixed `spot_meta_and_asset_ctxs()` endpoint deserialization - API returns a 2-element tuple array, not a single object with all fields
- Replaced `is_some()` + `unwrap()` with idiomatic `match` pattern in `NonceManager::next_nonce()` ([#18](https://github.com/lhermoso/hyperliquid-rust-sdk/pull/18))

## [0.1.1] - 2024-XX-XX

### Fixed
- Fixed trigger order limit_px field for trigger orders
- Corrected field order in trigger order serialization
