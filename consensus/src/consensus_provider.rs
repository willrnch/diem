// Copyright © Diem Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    counters,
    epoch_manager::EpochManager,
    network::NetworkTask,
    network_interface::{ConsensusMsg, ConsensusNetworkClient},
    persistent_liveness_storage::StorageWriteProxy,
    quorum_store::quorum_store_db::QuorumStoreDB,
    state_computer::ExecutionProxy,
    txn_notifier::MempoolNotifier,
    util::time_service::ClockTimeService,
};
use diem_bounded_executor::BoundedExecutor;
use diem_config::config::NodeConfig;
use diem_consensus_notifications::ConsensusNotificationSender;
use diem_event_notifications::ReconfigNotificationListener;
use diem_executor::block_executor::BlockExecutor;
use diem_logger::prelude::*;
use diem_mempool::QuorumStoreRequest;
use diem_network::application::interface::{NetworkClient, NetworkServiceEvents};
use diem_storage_interface::DbReaderWriter;
use diem_vm::DiemVM;
use futures::channel::mpsc;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Helper function to start consensus based on configuration and return the runtime
pub fn start_consensus(
    node_config: &NodeConfig,
    network_client: NetworkClient<ConsensusMsg>,
    network_service_events: NetworkServiceEvents<ConsensusMsg>,
    state_sync_notifier: Arc<dyn ConsensusNotificationSender>,
    consensus_to_mempool_sender: mpsc::Sender<QuorumStoreRequest>,
    diem_db: DbReaderWriter,
    reconfig_events: ReconfigNotificationListener,
) -> Runtime {
    let runtime = diem_runtimes::spawn_named_runtime("consensus".into(), None);
    let storage = Arc::new(StorageWriteProxy::new(node_config, diem_db.reader.clone()));
    let quorum_store_db = Arc::new(QuorumStoreDB::new(node_config.storage.dir()));

    let txn_notifier = Arc::new(MempoolNotifier::new(
        consensus_to_mempool_sender.clone(),
        node_config.consensus.mempool_executed_txn_timeout_ms,
    ));

    let state_computer = Arc::new(ExecutionProxy::new(
        Arc::new(BlockExecutor::<DiemVM>::new(diem_db)),
        txn_notifier,
        state_sync_notifier,
        runtime.handle(),
    ));

    let time_service = Arc::new(ClockTimeService::new(runtime.handle().clone()));

    let (timeout_sender, timeout_receiver) =
        diem_channels::new(1_024, &counters::PENDING_ROUND_TIMEOUTS);
    let (self_sender, self_receiver) = diem_channels::new(1_024, &counters::PENDING_SELF_MESSAGES);

    let consensus_network_client = ConsensusNetworkClient::new(network_client);
    let bounded_executor = BoundedExecutor::new(8, runtime.handle().clone());
    let epoch_mgr = EpochManager::new(
        node_config,
        time_service,
        self_sender,
        consensus_network_client,
        timeout_sender,
        consensus_to_mempool_sender,
        state_computer,
        storage,
        quorum_store_db,
        reconfig_events,
        bounded_executor,
    );

    let (network_task, network_receiver) = NetworkTask::new(network_service_events, self_receiver);

    runtime.spawn(network_task.start());
    runtime.spawn(epoch_mgr.start(timeout_receiver, network_receiver));

    debug!("Consensus started.");
    runtime
}
