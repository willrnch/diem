/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { DiemError } from './DiemError';

/**
 * Information telling which batch submission transactions failed
 */
export type TransactionsBatchSingleSubmissionFailure = {
    error: DiemError;
    /**
     * The index of which transaction failed, same as submission order
     */
    transaction_index: number;
};

