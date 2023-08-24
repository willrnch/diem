// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use diem_crypto::hash::{DefaultHasher, HashValue};
use diem_types::write_set::TransactionWrite;

pub(crate) fn module_hash<V: TransactionWrite>(module: &V) -> HashValue {
    module
        .extract_raw_bytes()
        .map(|bytes| {
            let mut hasher = DefaultHasher::new(b"Module");
            hasher.update(&bytes);
            hasher.finish()
        })
        .expect("Module can't be deleted")
}
