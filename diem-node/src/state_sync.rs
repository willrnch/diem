// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::network::ApplicationNetworkInterfaces;
use diem_config::config::{NodeConfig, StateSyncConfig, StorageServiceConfig};
use diem_consensus_notifications::ConsensusNotifier;
use diem_data_client::client::DiemDataClient;
use diem_data_streaming_service::{
    streaming_client::{new_streaming_service_client_listener_pair, StreamingServiceClient},
    streaming_service::DataStreamingService,
};
use diem_event_notifications::{EventSubscriptionService, ReconfigNotificationListener};
use diem_executor::chunk_executor::ChunkExecutor;
use diem_infallible::RwLock;
use diem_mempool_notifications::MempoolNotificationListener;
use diem_network::application::{
    interface::{NetworkClient, NetworkClientInterface, NetworkServiceEvents},
    storage::PeersAndMetadata,
};
use diem_state_sync_driver::{
    driver_factory::{DriverFactory, StateSyncRuntimes},
    metadata_storage::PersistentMetadataStorage,
};
use diem_storage_interface::{DbReader, DbReaderWriter};
use diem_storage_service_client::StorageServiceClient;
use diem_storage_service_server::{
    network::StorageServiceNetworkEvents, storage::StorageReader, StorageServiceServer,
};
use diem_storage_service_types::StorageServiceMessage;
use diem_time_service::TimeService;
use diem_types::{on_chain_config::ON_CHAIN_CONFIG_REGISTRY, waypoint::Waypoint};
use diem_vm::DiemVM;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Creates the event subscription service and two reconfiguration
/// notification listeners (for mempool and consensus, respectively).
pub fn create_event_subscription_service(
    node_config: &NodeConfig,
    db_rw: &DbReaderWriter,
) -> (
    EventSubscriptionService,
    ReconfigNotificationListener,
    Option<ReconfigNotificationListener>,
) {
    // Create the event subscription service
    let mut event_subscription_service = EventSubscriptionService::new(
        ON_CHAIN_CONFIG_REGISTRY,
        Arc::new(RwLock::new(db_rw.clone())),
    );

    // Create a reconfiguration subscription for mempool
    let mempool_reconfig_subscription = event_subscription_service
        .subscribe_to_reconfigurations()
        .expect("Mempool must subscribe to reconfigurations");

    // Create a reconfiguration subscription for consensus (if this is a validator)
    let consensus_reconfig_subscription = if node_config.base.role.is_validator() {
        Some(
            event_subscription_service
                .subscribe_to_reconfigurations()
                .expect("Consensus must subscribe to reconfigurations"),
        )
    } else {
        None
    };

    (
        event_subscription_service,
        mempool_reconfig_subscription,
        consensus_reconfig_subscription,
    )
}

/// Sets up all state sync runtimes and return the notification endpoints
pub fn start_state_sync_and_get_notification_handles(
    node_config: &NodeConfig,
    storage_network_interfaces: ApplicationNetworkInterfaces<StorageServiceMessage>,
    waypoint: Waypoint,
    event_subscription_service: EventSubscriptionService,
    db_rw: DbReaderWriter,
) -> anyhow::Result<(
    StateSyncRuntimes,
    MempoolNotificationListener,
    ConsensusNotifier,
)> {
    // Get the network client and events
    let network_client = storage_network_interfaces.network_client;
    let network_service_events = storage_network_interfaces.network_service_events;

    // Start the state sync storage service
    let peers_and_metadata = network_client.get_peers_and_metadata();
    let storage_service_runtime = setup_state_sync_storage_service(
        node_config.state_sync.storage_service,
        peers_and_metadata,
        network_service_events,
        &db_rw,
    )?;

    // Start the data client
    let (diem_data_client, diem_data_client_runtime) =
        setup_diem_data_client(node_config, network_client, db_rw.reader.clone())?;

    // Start the data streaming service
    let (streaming_service_client, streaming_service_runtime) =
        setup_data_streaming_service(node_config.state_sync, diem_data_client.clone())?;

    // Create the chunk executor and persistent storage
    let chunk_executor = Arc::new(ChunkExecutor::<DiemVM>::new(db_rw.clone()));
    let metadata_storage = PersistentMetadataStorage::new(&node_config.storage.dir());

    // Create notification senders and listeners for mempool and consensus
    let (mempool_notifier, mempool_listener) =
        diem_mempool_notifications::new_mempool_notifier_listener_pair();
    let (consensus_notifier, consensus_listener) =
        diem_consensus_notifications::new_consensus_notifier_listener_pair(
            node_config
                .state_sync
                .state_sync_driver
                .commit_notification_timeout_ms,
        );

    // Create the state sync driver factory
    let state_sync = DriverFactory::create_and_spawn_driver(
        true,
        node_config,
        waypoint,
        db_rw,
        chunk_executor,
        mempool_notifier,
        metadata_storage,
        consensus_listener,
        event_subscription_service,
        diem_data_client,
        streaming_service_client,
        TimeService::real(),
    );

    // Create a new state sync runtime handle
    let state_sync_runtimes = StateSyncRuntimes::new(
        diem_data_client_runtime,
        state_sync,
        storage_service_runtime,
        streaming_service_runtime,
    );

    Ok((state_sync_runtimes, mempool_listener, consensus_notifier))
}

/// Sets up the data streaming service runtime
fn setup_data_streaming_service(
    state_sync_config: StateSyncConfig,
    diem_data_client: DiemDataClient,
) -> anyhow::Result<(StreamingServiceClient, Runtime)> {
    // Create the data streaming service
    let (streaming_service_client, streaming_service_listener) =
        new_streaming_service_client_listener_pair();
    let data_streaming_service = DataStreamingService::new(
        state_sync_config.diem_data_client,
        state_sync_config.data_streaming_service,
        diem_data_client,
        streaming_service_listener,
    );

    // Start the data streaming service
    let streaming_service_runtime = diem_runtimes::spawn_named_runtime("stream-serv".into(), None);
    streaming_service_runtime.spawn(data_streaming_service.start_service());

    Ok((streaming_service_client, streaming_service_runtime))
}

/// Sets up the diem data client runtime
fn setup_diem_data_client(
    node_config: &NodeConfig,
    network_client: NetworkClient<StorageServiceMessage>,
    storage: Arc<dyn DbReader>,
) -> anyhow::Result<(DiemDataClient, Runtime)> {
    // Create the storage service client
    let storage_service_client = StorageServiceClient::new(network_client);

    // Create a new runtime for the data client
    let diem_data_client_runtime = diem_runtimes::spawn_named_runtime("data-client".into(), None);

    // Create the data client and spawn the data poller
    let (diem_data_client, data_summary_poller) = DiemDataClient::new(
        node_config.state_sync.diem_data_client,
        node_config.base.clone(),
        TimeService::real(),
        storage,
        storage_service_client,
        Some(diem_data_client_runtime.handle().clone()),
    );
    diem_data_client_runtime.spawn(data_summary_poller.start_poller());

    Ok((diem_data_client, diem_data_client_runtime))
}

/// Sets up the state sync storage service runtime
fn setup_state_sync_storage_service(
    config: StorageServiceConfig,
    peers_and_metadata: Arc<PeersAndMetadata>,
    network_service_events: NetworkServiceEvents<StorageServiceMessage>,
    db_rw: &DbReaderWriter,
) -> anyhow::Result<Runtime> {
    // Create a new state sync storage service runtime
    let storage_service_runtime = diem_runtimes::spawn_named_runtime("stor-server".into(), None);

    // Spawn the state sync storage service servers on the runtime
    let storage_reader = StorageReader::new(config, Arc::clone(&db_rw.reader));
    let service = StorageServiceServer::new(
        config,
        storage_service_runtime.handle().clone(),
        storage_reader,
        TimeService::real(),
        peers_and_metadata,
        StorageServiceNetworkEvents::new(network_service_events),
    );
    storage_service_runtime.spawn(service.start());

    Ok(storage_service_runtime)
}
