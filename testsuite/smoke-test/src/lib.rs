// Copyright © Diem Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

extern crate core;

#[cfg(test)]
mod diem;
#[cfg(test)]
mod diem_cli;
#[cfg(test)]
mod client;
#[cfg(test)]
mod consensus;
#[cfg(test)]
mod full_nodes;
#[cfg(test)]
mod fullnode;
#[cfg(test)]
mod genesis;
#[cfg(test)]
mod indexer;
#[cfg(test)]
mod inspection_service;
#[cfg(test)]
mod network;
#[cfg(test)]
mod rest_api;
#[cfg(test)]
mod rosetta;
#[cfg(test)]
mod state_sync;
#[cfg(test)]
mod test_smoke_tests;
#[cfg(test)]
mod transaction;
#[cfg(test)]
mod txn_broadcast;
#[cfg(test)]
mod txn_emitter;
#[cfg(test)]
mod upgrade;

//////// 0L ////////
// not in test config, and must be public
// these need to be callable in other test environments of third-party
// testsuites
pub mod smoke_test_environment;
pub mod storage;
pub mod test_utils;
mod workspace_builder;
/////// end 0L ////////
