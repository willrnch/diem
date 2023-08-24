// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use diem_indexer_grpc_cache_worker::IndexerGrpcCacheWorkerConfig;
use diem_indexer_grpc_server_framework::ServerArgs;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = ServerArgs::parse();
    args.run::<IndexerGrpcCacheWorkerConfig>().await
}
