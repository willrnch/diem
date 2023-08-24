// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use diem_api::context::Context;
use diem_config::config::NodeConfig;
use diem_mempool::mocks::MockSharedMempool;
use diem_storage_interface::mock::MockDbReaderWriter;
use diem_types::chain_id::ChainId;
use std::sync::Arc;

// This is necessary for building the API with how the code is structured currently.
pub fn get_fake_context() -> Context {
    let mempool = MockSharedMempool::new();
    Context::new(
        ChainId::test(),
        Arc::new(MockDbReaderWriter),
        mempool.ac_client,
        NodeConfig::default(),
    )
}
