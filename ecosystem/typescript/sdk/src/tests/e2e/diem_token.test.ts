import { DiemAccount } from "../../account";
import { UserTransaction } from "../../generated";
import { DiemToken } from "../../plugins";
import { Provider } from "../../providers";
import { PROVIDER_LOCAL_NETWORK_CONFIG, getFaucetClient, longTestTimeout } from "../unit/test_helper.test";

const provider = new Provider(PROVIDER_LOCAL_NETWORK_CONFIG);
const faucetClient = getFaucetClient();
const diemToken = new DiemToken(provider);

const alice = new DiemAccount();
const bob = new DiemAccount();

const collectionName = "AliceCollection";
const tokenName = "Alice Token";
let tokenAddress = "";

describe("token objects", () => {
  beforeAll(async () => {
    // Fund Alice's Account
    await faucetClient.fundAccount(alice.address(), 100000000);
    await faucetClient.fundAccount(bob.address(), 100000000);
  }, longTestTimeout);

  test(
    "create collection",
    async () => {
      await provider.waitForTransaction(
        await diemToken.createCollection(alice, "Alice's simple collection", collectionName, "https://diem.dev", 5, {
          royaltyNumerator: 10,
          royaltyDenominator: 10,
        }),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "mint",
    async () => {
      const txn = await provider.waitForTransactionWithResult(
        await diemToken.mint(
          alice,
          collectionName,
          "Alice's simple token",
          tokenName,
          "https://diem.dev/img/nyan.jpeg",
          ["key"],
          ["bool"],
          ["true"],
        ),
        { checkSuccess: true },
      );
      tokenAddress = (txn as UserTransaction).events[0].data.token;
    },
    longTestTimeout,
  );

  test(
    "mint soul bound",
    async () => {
      await provider.waitForTransaction(
        await diemToken.mintSoulBound(
          alice,
          collectionName,
          "Alice's simple soul bound token",
          "Alice's soul bound token",
          "https://diem.dev/img/nyan.jpeg",
          bob,
          ["key"],
          ["bool"],
          ["true"],
        ),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "freeze transfer",
    async () => {
      await provider.waitForTransaction(await diemToken.freezeTokenTransafer(alice, tokenAddress), {
        checkSuccess: true,
      });
    },
    longTestTimeout,
  );

  test(
    "unfreeze token transfer",
    async () => {
      await provider.waitForTransaction(await diemToken.unfreezeTokenTransafer(alice, tokenAddress), {
        checkSuccess: true,
      });
    },
    longTestTimeout,
  );

  test(
    "set token description",
    async () => {
      await provider.waitForTransaction(
        await diemToken.setTokenDescription(alice, tokenAddress, "my updated token description"),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "set token name",
    async () => {
      await provider.waitForTransaction(await diemToken.setTokenName(alice, tokenAddress, "my updated token name"), {
        checkSuccess: true,
      });
    },
    longTestTimeout,
  );

  test(
    "set token uri",
    async () => {
      await provider.waitForTransaction(
        await diemToken.setTokenName(alice, tokenAddress, "https://diem.dev/img/hero.jpg"),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "add token property",
    async () => {
      await provider.waitForTransaction(
        await diemToken.addTokenProperty(alice, tokenAddress, "newKey", "BOOLEAN", "true"),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "add typed property",
    async () => {
      await provider.waitForTransaction(
        await diemToken.addTypedProperty(alice, tokenAddress, "newTypedKey", "VECTOR", "[hello,world]"),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "update typed property",
    async () => {
      await provider.waitForTransaction(
        await diemToken.updateTypedProperty(alice, tokenAddress, "newTypedKey", "U8", "2"),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "update token property",
    async () => {
      await provider.waitForTransaction(
        await diemToken.updateTokenProperty(alice, tokenAddress, "newKey", "U8", "5"),
        { checkSuccess: true },
      );
    },
    longTestTimeout,
  );

  test(
    "remove token property",
    async () => {
      await provider.waitForTransaction(await diemToken.removeTokenProperty(alice, tokenAddress, "newKey"), {
        checkSuccess: true,
      });
    },
    longTestTimeout,
  );

  test(
    "transfer token ownership",
    async () => {
      await provider.waitForTransaction(await diemToken.transferTokenOwnership(alice, tokenAddress, bob.address()), {
        checkSuccess: true,
      });
    },
    longTestTimeout,
  );

  test(
    "transfer non fungible token",
    async () => {
      const getTokenDataSpy = jest.spyOn(provider, "getTokenData");
      const getTokenDataSpyResponse = { current_token_datas_v2: new Array() };
      getTokenDataSpyResponse.current_token_datas_v2.push({ is_fungible_v2: undefined });
      getTokenDataSpy.mockResolvedValue(getTokenDataSpyResponse);

      await provider.waitForTransaction(
        await diemToken.transfer({ owner: bob, tokenAddress, recipient: alice.address() }),
        {
          checkSuccess: true,
        },
      );
      getTokenDataSpy.mockRestore();
    },
    longTestTimeout,
  );

  test(
    "transfer non fungible token when isFungibleToken param set to false",
    async () => {
      await provider.waitForTransaction(
        await diemToken.transfer({ owner: alice, tokenAddress, recipient: bob.address() }, false),
        {
          checkSuccess: true,
        },
      );
    },
    longTestTimeout,
  );

  test(
    "getTokenData indexer query is not being called when isFungibleToken param is set",
    async () => {
      const getTokenDataSpy = jest.spyOn(provider, "getTokenData");
      await diemToken.transfer({ owner: bob, tokenAddress, recipient: alice.address() }, false);
      expect(getTokenDataSpy).not.toBeCalled();
      getTokenDataSpy.mockRestore();
    },
    longTestTimeout,
  );

  test(
    "burn token",
    async () => {
      await provider.waitForTransaction(await diemToken.burnToken(alice, tokenAddress), { checkSuccess: true });
    },
    longTestTimeout,
  );
});
