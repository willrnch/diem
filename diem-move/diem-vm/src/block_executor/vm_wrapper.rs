// Copyright © Diem Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    adapter_common::{PreprocessedTransaction, VMAdapter},
    diem_vm::DiemVM,
    block_executor::DiemTransactionOutput,
};
use diem_block_executor::task::{ExecutionStatus, ExecutorTask};
use diem_logger::{enabled, Level};
use diem_mvhashmap::types::TxnIndex;
use diem_state_view::StateView;
use diem_vm_logging::{log_schema::AdapterLogSchema, prelude::*};
use move_core_types::{
    ident_str,
    language_storage::{ModuleId, CORE_CODE_ADDRESS},
    vm_status::VMStatus,
};

pub(crate) struct DiemExecutorTask<'a, S> {
    vm: DiemVM,
    base_view: &'a S,
}

impl<'a, S: 'a + StateView + Sync> ExecutorTask for DiemExecutorTask<'a, S> {
    type Argument = &'a S;
    type Error = VMStatus;
    type Output = DiemTransactionOutput;
    type Txn = PreprocessedTransaction;

    fn init(argument: &'a S) -> Self {
        let vm = DiemVM::new(argument);

        // Loading `0x1::account` and its transitive dependency into the code cache.
        //
        // This should give us a warm VM to avoid the overhead of VM cold start.
        // Result of this load could be omitted as this is a best effort approach and won't hurt if that fails.
        //
        // Loading up `0x1::account` should be sufficient as this is the most common module
        // used for prologue, epilogue and transfer functionality.

        let _ = vm.load_module(
            &ModuleId::new(CORE_CODE_ADDRESS, ident_str!("account").to_owned()),
            &vm.as_move_resolver(argument),
        );

        Self {
            vm,
            base_view: argument,
        }
    }

    // This function is called by the BlockExecutor for each transaction is intends
    // to execute (via the ExecutorTask trait). It can be as a part of sequential
    // execution, or speculatively as a part of a parallel execution.
    fn execute_transaction(
        &self,
        view: &impl StateView,
        txn: &PreprocessedTransaction,
        txn_idx: TxnIndex,
        materialize_deltas: bool,
    ) -> ExecutionStatus<DiemTransactionOutput, VMStatus> {
        let log_context = AdapterLogSchema::new(self.base_view.id(), txn_idx as usize);

        match self
            .vm
            .execute_single_transaction(txn, &self.vm.as_move_resolver(view), &log_context)
        {
            Ok((vm_status, mut vm_output, sender)) => {
                if materialize_deltas {
                    // TODO: Integrate delta application failure.
                    vm_output = vm_output
                        .try_materialize(view)
                        .expect("Delta materialization failed");
                }

                if vm_output.status().is_discarded() {
                    match sender {
                        Some(s) => speculative_trace!(
                            &log_context,
                            format!(
                                "Transaction discarded, sender: {}, error: {:?}",
                                s, vm_status
                            ),
                        ),
                        None => {
                            speculative_trace!(
                                &log_context,
                                format!("Transaction malformed, error: {:?}", vm_status),
                            )
                        },
                    };
                }
                if DiemVM::should_restart_execution(&vm_output) {
                    speculative_info!(
                        &log_context,
                        "Reconfiguration occurred: restart required".into()
                    );
                    ExecutionStatus::SkipRest(DiemTransactionOutput::new(vm_output))
                } else {
                    ExecutionStatus::Success(DiemTransactionOutput::new(vm_output))
                }
            },
            Err(err) => ExecutionStatus::Abort(err),
        }
    }
}
