// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

//:!:>section_1
export const NODE_URL = process.env.DIEM_NODE_URL || "https://fullnode.devnet.diemlabs.com";
export const FAUCET_URL = process.env.DIEM_FAUCET_URL || "https://faucet.devnet.diemlabs.com";
//<:!:section_1

export const diemCoinStore = "0x1::coin::CoinStore<0x1::diem_coin::DiemCoin>";
export const fungibleStore = "0x1::fungible_asset::FungibleStore";
