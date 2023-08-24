// Copyright © Diem Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use diem_transactional_test_harness::run_diem_test;

datatest_stable::harness!(run_diem_test, "tests", r".*\.(mvir|move)$");
