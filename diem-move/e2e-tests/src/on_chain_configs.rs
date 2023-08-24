// Copyright © Diem Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{account::Account, executor::FakeExecutor};
use diem_cached_packages::diem_stdlib;
use diem_types::{account_config::CORE_CODE_ADDRESS, on_chain_config::Version};
use diem_vm::DiemVM;

pub fn set_diem_version(executor: &mut FakeExecutor, version: Version) {
    let account = Account::new_genesis_account(CORE_CODE_ADDRESS);
    let txn = account
        .transaction()
        .payload(diem_stdlib::version_set_version(version.major))
        .sequence_number(0)
        .sign();
    executor.new_block();
    executor.execute_and_apply(txn);

    let new_vm = DiemVM::new(executor.get_state_view());
    assert_eq!(new_vm.internals().version().unwrap(), version);
}
