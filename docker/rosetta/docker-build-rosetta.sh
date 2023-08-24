#!/bin/bash
# Copyright © Diem Foundation
# SPDX-License-Identifier: Apache-2.0

# This script is meant to build the rosetta docker image.
# Run it via `docker/rosetta/docker-build-rosetta.sh`
set -ex

export GIT_REPO="${GIT_REPO:-https://github.com/aptos-labs/diem-core.git}"
export GIT_REF="${GIT_REF:-$(git rev-parse HEAD)}"
docker buildx build --file docker/rosetta/rosetta.Dockerfile --build-arg=GIT_REPO=$GIT_REPO --build-arg=GIT_REF=$GIT_REF -t diem-core:rosetta-$GIT_REF -t diem-core:rosetta-latest --load .
