# Copyright © Diem Foundation
# SPDX-License-Identifier: Apache-2.0

import os

# :!:>section_1
NODE_URL = os.getenv("DIEM_NODE_URL", "https://fullnode.devnet.diemlabs.com/v1")
FAUCET_URL = os.getenv(
    "DIEM_FAUCET_URL",
    "https://faucet.devnet.diemlabs.com",
)  # <:!:section_1
