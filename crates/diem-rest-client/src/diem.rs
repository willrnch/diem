// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use diem_api_types::U64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiemCoin {
    pub value: U64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub coin: DiemCoin,
}

impl Balance {
    pub fn get(&self) -> u64 {
        *self.coin.value.inner()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiemVersion {
    pub major: U64,
}
