#![forbid(unsafe_code)]

// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use diem_telemetry_service::DiemTelemetryServiceArgs;
use clap::Parser;

#[tokio::main]
async fn main() {
    diem_logger::Logger::new().init();
    DiemTelemetryServiceArgs::parse().run().await;
}
