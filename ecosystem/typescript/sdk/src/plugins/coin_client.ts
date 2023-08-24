// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0
import { DiemAccount, getAddressFromAccountOrAddress } from "../account/diem_account";
import { DiemClient, OptionalTransactionArgs } from "../providers/diem_client";
import { MaybeHexString, DIEM_COIN, NetworkToIndexerAPI, NodeAPIToNetwork } from "../utils";
import { TransactionBuilderRemoteABI } from "../transaction_builder";
import { FungibleAssetClient } from "./fungible_asset_client";
import { Provider } from "../providers";
import { AccountAddress } from "../diem_types";

/**
 * Class for working with the coin module, such as transferring coins and
 * checking balances.
 */
export class CoinClient {
  diemClient: DiemClient;

  /**
   * Creates new CoinClient instance
   * @param diemClient DiemClient instance
   */
  constructor(diemClient: DiemClient) {
    this.diemClient = diemClient;
  }

  /**
   * Generate, sign, and submit a transaction to the Diem blockchain API to
   * transfer coins from one account to another. By default it transfers
   * 0x1::diem_coin::DiemCoin, but you can specify a different coin type
   * with the `coinType` argument.
   *
   * You may set `createReceiverIfMissing` to true if you want to create the
   * receiver account if it does not exist on chain yet. If you do not set
   * this to true, the transaction will fail if the receiver account does not
   * exist on-chain.
   *
   * The TS SDK supports fungible assets operations. If you want to use CoinClient
   * with this feature, set the `coinType` to be the fungible asset metadata address.
   * This option uses the `FungibleAssetClient` class and queries the
   * fungible asset primary store.
   *
   * @param from Account sending the coins
   * @param to Account to receive the coins
   * @param amount Number of coins to transfer
   * @param extraArgs Extra args for building the transaction or configuring how
   * the client should submit and wait for the transaction
   * @returns The hash of the transaction submitted to the API
   */
  // :!:>transfer
  async transfer(
    from: DiemAccount,
    to: DiemAccount | MaybeHexString,
    amount: number | bigint,
    extraArgs?: OptionalTransactionArgs & {
      // The coin type to use, defaults to 0x1::diem_coin::DiemCoin.
      // If you want to transfer a fungible asset, set this param to be the
      // fungible asset address
      coinType?: string | MaybeHexString;
      // If set, create the `receiver` account if it doesn't exist on-chain.
      // This is done by calling `0x1::diem_account::transfer` instead, which
      // will create the account on-chain first if it doesn't exist before
      // transferring the coins to it.
      // If this is the first time an account has received the specified coinType,
      // and this is set to false, the transaction would fail.
      createReceiverIfMissing?: boolean;
    },
  ): Promise<string> {
    if (extraArgs?.coinType && AccountAddress.isValid(extraArgs.coinType)) {
      /* eslint-disable no-console */
      console.warn("to transfer a fungible asset, use `FungibleAssetClient()` class for better support");
      const provider = new Provider({
        fullnodeUrl: this.diemClient.nodeUrl,
        indexerUrl: NetworkToIndexerAPI[NodeAPIToNetwork[this.diemClient.nodeUrl]] ?? this.diemClient.nodeUrl,
      });
      const fungibleAsset = new FungibleAssetClient(provider);
      const txnHash = await fungibleAsset.transfer(
        from,
        extraArgs?.coinType,
        getAddressFromAccountOrAddress(to),
        amount,
      );
      return txnHash;
    }

    // If none is explicitly given, use 0x1::diem_coin::DiemCoin as the coin type.
    const coinTypeToTransfer = extraArgs?.coinType ?? DIEM_COIN;

    // If we should create the receiver account if it doesn't exist on-chain,
    // use the `0x1::diem_account::transfer` function.
    const func = extraArgs?.createReceiverIfMissing ? "0x1::diem_account::transfer_coins" : "0x1::coin::transfer";

    // Get the receiver address from the DiemAccount or MaybeHexString.
    const toAddress = getAddressFromAccountOrAddress(to);

    const builder = new TransactionBuilderRemoteABI(this.diemClient, { sender: from.address(), ...extraArgs });
    const rawTxn = await builder.build(func, [coinTypeToTransfer as string], [toAddress, amount]);

    const bcsTxn = DiemClient.generateBCSTransaction(from, rawTxn);
    const pendingTransaction = await this.diemClient.submitSignedBCSTransaction(bcsTxn);
    return pendingTransaction.hash;
  } // <:!:transfer

  /**
   * Get the balance of the account. By default it checks the balance of
   * 0x1::diem_coin::DiemCoin, but you can specify a different coin type.
   *
   * to use a different type, set the `coinType` to be the fungible asset type.
   *
   * The TS SDK supports fungible assets operations. If you want to use CoinClient
   * with this feature, set the `coinType` to be the fungible asset metadata address.
   * This option uses the FungibleAssetClient class and queries the
   * fungible asset primary store.
   *
   * @param account Account that you want to get the balance of.
   * @param extraArgs Extra args for checking the balance.
   * @returns Promise that resolves to the balance as a bigint.
   */
  // :!:>checkBalance
  async checkBalance(
    account: DiemAccount | MaybeHexString,
    extraArgs?: {
      // The coin type to use, defaults to 0x1::diem_coin::DiemCoin.
      // If you want to check the balance of a fungible asset, set this param to be the
      // fungible asset address
      coinType?: string;
    },
  ): Promise<bigint> {
    if (extraArgs?.coinType && AccountAddress.isValid(extraArgs.coinType)) {
      /* eslint-disable no-console */
      console.warn("to check balance of a fungible asset, use `FungibleAssetClient()` class for better support");
      const provider = new Provider({
        fullnodeUrl: this.diemClient.nodeUrl,
        indexerUrl: NetworkToIndexerAPI[NodeAPIToNetwork[this.diemClient.nodeUrl]] ?? this.diemClient.nodeUrl,
      });
      const fungibleAsset = new FungibleAssetClient(provider);
      const balance = await fungibleAsset.getPrimaryBalance(
        getAddressFromAccountOrAddress(account),
        extraArgs?.coinType,
      );
      return balance;
    }

    const coinType = extraArgs?.coinType ?? DIEM_COIN;
    const typeTag = `0x1::coin::CoinStore<${coinType}>`;
    const address = getAddressFromAccountOrAddress(account);
    const accountResource = await this.diemClient.getAccountResource(address, typeTag);
    return BigInt((accountResource.data as any).coin.value);
  } // <:!:checkBalance
}
