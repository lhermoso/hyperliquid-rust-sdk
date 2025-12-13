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

#### Documentation
- Created `docs/API_AUDIT.md` with comprehensive API coverage analysis
- Documented Phase 2 and Phase 3 implementation plans for future work
- Updated README with fork attribution to ControlCplusControlV/ferrofluid and new package name ([#2](https://github.com/lhermoso/hyperliquid-rust-sdk/pull/2))
- Updated API_AUDIT.md to reflect Phase 2 completion status

### Changed
- Added `Clone` derive to `MarginSummary` type
- Added `Clone` derive to `EvmContract` type

## [0.1.1] - 2024-XX-XX

### Fixed
- Fixed trigger order limit_px field for trigger orders
- Corrected field order in trigger order serialization
