// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

pub use crate::{
    diem_framework_sdk_builder::*, diem_token_objects_sdk_builder as diem_token_objects_stdlib,
    diem_token_sdk_builder as diem_token_stdlib,
};
use diem_types::{account_address::AccountAddress, transaction::TransactionPayload};

pub fn diem_coin_transfer(to: AccountAddress, amount: u64) -> TransactionPayload {
    coin_transfer(
        diem_types::utility_coin::DIEM_COIN_TYPE.clone(),
        to,
        amount,
    )
}
