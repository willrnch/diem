# Copyright © Diem Foundation
# SPDX-License-Identifier: Apache-2.0

import asyncio

from diem_sdk.async_client import RestClient

from .common import NODE_URL


async def main():
    rest_client = RestClient(NODE_URL)
    total_apt = await rest_client.aggregator_value(
        "0x1", "0x1::coin::CoinInfo<0x1::diem_coin::DiemCoin>", ["supply"]
    )
    print(f"Total circulating APT: {total_apt}")
    await rest_client.close()


if __name__ == "__main__":
    asyncio.run(main())
