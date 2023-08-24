// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

/// Chain ID of the current chain
pub const X_DIEM_CHAIN_ID: &str = "X-Diem-Chain-Id";
/// Current epoch of the chain
pub const X_DIEM_EPOCH: &str = "X-Diem-Epoch";
/// Current ledger version of the chain
pub const X_DIEM_LEDGER_VERSION: &str = "X-Diem-Ledger-Version";
/// Oldest non-pruned ledger version of the chain
pub const X_DIEM_LEDGER_OLDEST_VERSION: &str = "X-Diem-Ledger-Oldest-Version";
/// Current block height of the chain
pub const X_DIEM_BLOCK_HEIGHT: &str = "X-Diem-Block-Height";
/// Oldest non-pruned block height of the chain
pub const X_DIEM_OLDEST_BLOCK_HEIGHT: &str = "X-Diem-Oldest-Block-Height";
/// Current timestamp of the chain
pub const X_DIEM_LEDGER_TIMESTAMP: &str = "X-Diem-Ledger-TimestampUsec";
/// Cursor used for pagination.
pub const X_DIEM_CURSOR: &str = "X-Diem-Cursor";
/// Provided by the client to identify what client it is.
pub const X_DIEM_CLIENT: &str = "x-diem-client";
