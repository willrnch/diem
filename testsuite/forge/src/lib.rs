// Copyright © Diem Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Forge is a framework for writing and running end-to-end tests in Diem

pub use anyhow::Result;

mod interface;
pub use interface::*;

mod runner;
pub use runner::*;

mod backend;
pub use diem_transaction_emitter_lib::*;
pub use diem_transaction_generator_lib::*;
pub use backend::*;

mod report;
pub use report::*;

mod github;
pub use github::*;

mod slack;
pub use slack::*;

pub mod success_criteria;

pub mod test_utils;
