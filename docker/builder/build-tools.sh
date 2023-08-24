#!/bin/bash
# Copyright (c) Diem
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=${PROFILE:-release}

echo "Building tools and services docker images"
echo "PROFILE: $PROFILE"
echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p diem \
    -p diem-backup-cli \
    -p diem-faucet-service \
    -p diem-forge-cli \
    -p diem-fn-check-client \
    -p diem-node-checker \
    -p diem-openapi-spec-generator \
    -p diem-telemetry-service \
    -p diem-db-bootstrapper \
    -p diem-db-tool \
    -p diem-transaction-emitter \
    -p diem-indexer-grpc-cache-worker \
    -p diem-indexer-grpc-file-store \
    -p diem-indexer-grpc-data-service \
    -p diem-indexer-grpc-parser \
    -p diem-indexer-grpc-post-processor \
    "$@"

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    diem
    diem-faucet-service
    diem-node-checker
    diem-openapi-spec-generator
    diem-telemetry-service
    diem-fn-check-client
    diem-db-tool
    diem-db-bootstrapper
    forge
    diem-transaction-emitter
    diem-indexer-grpc-cache-worker
    diem-indexer-grpc-file-store
    diem-indexer-grpc-data-service
    diem-indexer-grpc-parser
    diem-indexer-grpc-post-processor
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done

# Build the Diem Move framework and place it in dist. It can be found afterwards in the current directory.
echo "Building the Diem Move framework..."
(cd dist && cargo run --locked --profile=$PROFILE --package diem-framework -- release)