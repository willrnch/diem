/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { DiemErrorCode } from './DiemErrorCode';

/**
 * This is the generic struct we use for all API errors, it contains a string
 * message and an Diem API specific error code.
 */
export type DiemError = {
    /**
     * A message describing the error
     */
    message: string;
    error_code: DiemErrorCode;
    /**
     * A code providing VM error details when submitting transactions to the VM
     */
    vm_error_code?: number;
};

