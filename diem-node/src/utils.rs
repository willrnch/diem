// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use diem_config::config::NodeConfig;
use diem_state_view::account_with_state_view::AsAccountWithStateView;
use diem_storage_interface::{state_view::LatestDbStateCheckpointView, DbReaderWriter};
use diem_types::{
    account_config::CORE_CODE_ADDRESS, account_view::AccountView, chain_id::ChainId,
};
use diem_vm::DiemVM;

/// Error message to display when non-production features are enabled
pub const ERROR_MSG_BAD_FEATURE_FLAGS: &str = r#"
diem-node was compiled with feature flags that shouldn't be enabled.

This is caused by cargo's feature unification.
When you compile two crates with a shared dependency, if one enables a feature flag for the dependency, then it is also enabled for the other crate.

PLEASE RECOMPILE DIEM-NODE SEPARATELY using the following command:
    cargo build --package diem-node

"#;

/// Initializes a global rayon thread pool iff `create_global_rayon_pool` is true
pub fn create_global_rayon_pool(create_global_rayon_pool: bool) {
    if create_global_rayon_pool {
        rayon::ThreadPoolBuilder::new()
            .thread_name(|index| format!("rayon-global-{}", index))
            .build_global()
            .expect("Failed to build rayon global thread pool.");
    }
}

/// Fetches the chain ID from on-chain resources
pub fn fetch_chain_id(db: &DbReaderWriter) -> anyhow::Result<ChainId> {
    let db_state_view = db
        .reader
        .latest_state_checkpoint_view()
        .map_err(|err| anyhow!("[diem-node] failed to create db state view {}", err))?;
    Ok(db_state_view
        .as_account_with_state_view(&CORE_CODE_ADDRESS)
        .get_chain_id_resource()
        .map_err(|err| anyhow!("[diem-node] failed to get chain id resource {}", err))?
        .expect("[diem-node] missing chain ID resource")
        .chain_id())
}

/// Sets the Diem VM configuration based on the node configurations
pub fn set_diem_vm_configurations(node_config: &NodeConfig) {
    DiemVM::set_paranoid_type_checks(node_config.execution.paranoid_type_verification);
    DiemVM::set_concurrency_level_once(node_config.execution.concurrency_level as usize);
    DiemVM::set_num_proof_reading_threads_once(
        node_config.execution.num_proof_reading_threads as usize,
    );

    if node_config
        .execution
        .processed_transactions_detailed_counters
    {
        DiemVM::set_processed_transactions_detailed_counters();
    }
}
