// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    smoke_test_environment::SwarmBuilder,
    test_utils::{MAX_CONNECTIVITY_WAIT_SECS, MAX_HEALTHY_WAIT_SECS},
};
use diem_config::config::NodeConfig;
use diem_forge::{NodeExt, Swarm};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

/// Bring up a swarm normally, then run get_bin, and bring up a VFN.
/// Previously get_bin triggered a rebuild of diem-node, which caused issues that were only seen
/// during parallel execution of tests.
/// This test should make regressions obvious.
#[tokio::test]
async fn test_diem_node_after_get_bin() {
    let mut swarm = SwarmBuilder::new_local(1)
        .with_diem()
        .with_init_config(Arc::new(|_, conf, _| {
            conf.api.failpoints_enabled = true;
        }))
        .build()
        .await;
    let version = swarm.versions().max().unwrap();
    let validator_peer_ids = swarm.validators().map(|v| v.peer_id()).collect::<Vec<_>>();

    // Before #5308 this re-compiled diem-node and caused a panic on the vfn.
    let _diem_cli = crate::workspace_builder::get_bin("diem");

    let validator = validator_peer_ids[0];
    let _vfn = swarm
        .add_validator_fullnode(&version, NodeConfig::get_default_vfn_config(), validator)
        .unwrap();

    for fullnode in swarm.full_nodes_mut() {
        fullnode
            .wait_until_healthy(Instant::now() + Duration::from_secs(MAX_HEALTHY_WAIT_SECS))
            .await
            .unwrap();
        fullnode
            .wait_for_connectivity(Instant::now() + Duration::from_secs(MAX_CONNECTIVITY_WAIT_SECS))
            .await
            .unwrap();
    }
}
