---
title: "DiemClient Class"
slug: "typescript-sdk-diem-client-class"
---

The [DiemClient](https://aptos-labs.github.io/ts-sdk-doc/classes/DiemClient.html) class is a component of the Diem TypeScript SDK that enables developers to interact with the blockchain network through the use of REST APIs generated from an OpenAPI document. The [OpenAPI specification](https://spec.openapis.org/oas/v3.0.3) helps build and document RESTful APIs by providing a standard format for describing the structure of an API, including the available endpoints, methods, input and output parameters.

In addition, the `DiemClient` component supports submitting transactions in BCS format, which prepares and signs the raw transactions on the client-side. This method leverages the BCS Library or Transaction Builder for constructing the transaction payloads.

## Usage

To use the `DiemClient` class, you will need to create an instance of `DiemClient` and call the desired API method. The `DiemClient` object will handle the HTTP requests and responses and return the result to your application.

## Configuration

Before using the `DiemClient` class, you will need to configure it with the necessary parameters. These parameters may include the network endpoint URL, custom configuration, and any other required settings. You can configure the `DiemClient` class by passing in the necessary parameters when you initialize the client object.

## Initialization

Here is an example of how to initialize an `DiemClient`:

```ts
import { DiemClient } from "diem";

const client = new DiemClient("https://fullnode.testnet.diemlabs.com");
```

## Making API fetch calls

To make an API call, you will need to call the appropriate method on the `DiemClient` object. The method name and parameters will depend on the specific API you are using. Here is an example:

```ts
const accountResources = await client.getAccountResources("0x123");
```

In this example, we are using the `getAccountResources()` method to retrieve the resources of an account with the address `0x123`.

## Submit transaction to chain

To submit a transaction to the blockchain, you will need to build a transaction payload to be submitted. Here is an example:

```ts
const alice = new DiemAccount();

const payload: Types.EntryFunctionPayload = {
  function: "0x123::todolist::create_task",
  type_arguments: [],
  arguments: ["read diem.dev"],
};

const rawTxn = await client.generateTransaction(alice.address(), payload);
const bcsTxn = DiemClient.generateBCSTransaction(alice, rawTxn);
const transactionRes = await client.submitSignedBCSTransaction(bcsTxn);
```

Transaction payload contents:  
`function` – This must be a fully qualified function name and composed of `module address`, `module name` and `function name` separated by `::`.
`type_arguments` – This is for the case a Move function expects a generic type argument.
`arguments` – The arguments the function expects.

:::tip
You can use the `DiemClient` class directly or the [Provider](./sdk-client-layer.md) class (preferred).
:::
