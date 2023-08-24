#!/bin/bash
# Copyright (c) Diem
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=${PROFILE:-release}
FEATURES=${FEATURES:-""}

echo "Building diem-node"
echo "PROFILE: $PROFILE"
echo "FEATURES: $FEATURES"
echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build and overwrite the diem-node binary with features if specified
if [ -n "$FEATURES" ]; then
    echo "Building diem-node with features ${FEATURES}"
    cargo build --profile=$PROFILE --features=$FEATURES -p diem-node "$@"
else 
    # Build diem-node separately
    cargo build --locked --profile=$PROFILE \
        -p diem-node \
        "$@"
fi

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    diem-node
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done
