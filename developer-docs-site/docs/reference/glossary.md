---
title: "Diem Glossary"
slug: "glossary"
---

# Diem Glossary

## A

### Accumulator Root Hash

- An **accumulator root hash** is the root hash of a [Merkle accumulator.](https://eprint.iacr.org/2009/625.pdf)

### Account

- An **account** in the Diem blockchain is a container for an arbitrary number of [Move modules](#move-module) and [Move resources](#move-resources). This essentially means that the state of each [account](../concepts/accounts.md) is comprised of both code and data.
- The account is identified by [account address](#account-address).

See [Accounts](../concepts/accounts.md) for more information.

### Account Address

- An **account address** is the address of an Diem account.
- Account address refers to a specific destination on the Diem network. The address dictates the destination and source of a specific amount of assets exchanged by two parties on the blockchain. 
- An Diem address is a 64-character hex string, and sometimes it can be shortened by stripping leading 0s and prefixing `0x`. This makes a hex-encoded 32 byte Diem account address.

See [Accounts](../concepts/accounts.md) for more information.

### API

- An **Application Programming Interface (API)(** is a set of protocols and tools that allow users to interact with Diem blockchain nodes and client networks via external applications. Diem offers a REST API for this purpose. See the [Diem API reference](https://diem.dev/nodes/diem-api-spec#/) documentation and [Use the Diem API](../integration/diem-apis.md) for more details.

### APT

**Diem token (APT)** is the Diem blockchain native token used for paying network and transaction fees.

### Diem

**Diem** is a Layer 1 blockchain for everyone. It uses the Move programming language and launched its mainnet on 2022-10-17 to redefine the web3 user experience. The Diem blockchain is dedicated to creating better user experiences through increased speed, security, scalability, reliability and usability with low transaction costs.  The word “Diem" means "The People" in the Ohlone language. See the [Diem White Paper](../diem-white-paper/index.md) for more details.

### DiemBFT

- **DiemBFT** is the Diem protocol's BFT consensus algorithm.
- DiemBFT is based on Jolteon.

### Diem Blockchain

- The **Diem blockchain** is a ledger of immutable transactions agreed upon by the validators on the Diem network (the network of validators).

### Diem Name Service (ANS)

- The **Diem Name Service (ANS)** is a decentralized naming address service for the Diem blockchain. An Diem name is a human-readable *.apt* domain name that is used in place of a public key, for example *love.apt*. 
- This service also allows users to register subdomain names in addition to the registered domain. Find out more at: [Diemnames.com](https://www.diemnames.com/)

### Diem-core

**Diem-core** is the open source technology on which the Diem Payment Network runs. Diem-core contains software for

* the Diem blockchain itself, which generates and stores the immutable ledger of confirmed transactions and
* the validation process, which implements the consensus algorithm to validate transactions and add them to the Diem blockchain immutable ledger.

### Diem Ecosystem

- **Diem ecosystem** refers to various components of the Diem blockchain network and their interactions.  The Diem ecosystem includes the community, community-driven projects, and events. See [Contribute to the Diem Ecosystem](../community/index.md) for all possible ways to join Diem.

### Diem Explorer

- The **[Diem Explorer](https://explorer.diemlabs.com/)** is an interface that helps users examine details of the Diem blockchain, including account information, validators, and transactions.
- The Diem Explorer help users validate their work in Diem wallets and other tools in the blockchain. Find more details at [Use the Diem Explorer](../guides/explore-diem.md).

### Diem Framework
The **Diem Framework** defines the public API for blockchain updates and the structure of on-chain data. It defines the business logic and access control for the three key pillars of Diem functionality: payments, treasury, and on-chain governance. It is implemented as a set of modules written in the Move programming language and stored on-chain as Move bytecode.

### Diem Node
An **Diem node** is a peer entity of the Diem network that tracks the state of the Diem blockchain. There are two types of Diem nodes, [validators](#validator) and [fullnodes](#fullnode)).

### Diem Protocol

- **Diem protocol** is the specification of how transactions are submitted, ordered, executed, and recorded within the Diem network.

### DiemAccount

- A **`DiemAccount`** is a Move resource that holds all the administrative data associated with an account, such as sequence number, balance, and authentication key.
- A **`DiemAccount`** is the only resource that every account is guaranteed to contain.

### DiemAccount module

- **The DiemAccount module** is a Move module that contains the code for manipulating the administrative data held in a particular `DiemAccount.T` resource.
- Code for checking or incrementing sequence numbers, withdrawing or depositing currency, and extracting gas deposits is included in the DiemAccount module.

### Diem devnet

- See [devnet](#devnet).

## B

### Byzantine (Validator)

- A **validator** that does not follow the specification of the consensus protocol, and wishes to compromise the correct execution of the protocol.
- BFT algorithms traditionally support up to one-third of the algorithm's voting power being held by Byzantine validators.

### Byzantine Fault Tolerance (BFT)

- **Byzantine Fault Tolerance** (BFT) is the ability of a distributed system to provide safety and liveness guarantees in the presence of faulty, or “[Byzantine](#byzantine-validator),” validators below a certain threshold.
- The Diem blockchain uses DiemBFT, a consensus protocol based on [Jolteon](#Jolteon).
- BFT algorithms typically operate with a number of entities, collectively holding N votes (which are called “validators” in the Diem network’s application of the system).
- N is chosen to withstand some number of validators holding f votes, which might be malicious.
- In this configuration, N is typically set to 3f+1. Validators holding up to f votes will be allowed to be faulty &mdash; offline, malicious, slow, etc. As long as 2f+1 votes are held by [honest](#honest-validator) validators, they will be able to reach consensus on consistent decisions.
- This implies that BFT consensus protocols can function correctly, even if up to one-third of the voting power is held by validators that are compromised or fail.

## C

### CLI

- **Command line interface** refers to the Diem CLI used for developing on the Diem blockchain, operating nodes, and debugging issues. Find out more at [Use the Diem CLI](../tools/diem-cli-tool/use-diem-cli.md). 

### Client

- **Client** is software that receives information from the blockchain and manages transactions. Clients interact with the blockchain through the Diem nodes.

### Code labs

- **Code labs and tutorials** depict various workflows - such as the use of the Diem CLI in minting non-fungible tokens (NFTs) - in order for users to understand how the process works and employ related functions in their code. If users have the necessary funds in their accounts, they can follow the same code lab and tutorial steps used in devnet, testnet and mainnet networks.

### Consensus

- **Consensus** is a component of a validator.
- The consensus component is responsible for coordination and agreement amongst all validators on the block of transactions to be executed, their order, and the execution results.
- The Diem blockchain is formed with these agreed-upon transactions and their corresponding execution results.
- The consensus component is accountable for achieving security, trust, and agreement among all validators on the Diem blockchain.

### Consensus Protocol

- A **consensus protocol** is collectively executed by n validators to accept or reject a transaction and to agree on the ordering of transactions and [execution results](#execution-result).
- See [BFT](#byzantine-fault-tolerance-bft).

## D

### dApps

- **Decentralized applications (dApps)** are programs or digital applications that run on the Diem blockchain autonomously. Smart contracts are commonly used to achieve this function.

### devnet

- The **Diem devnet** is a publicly deployed instance of the Diem network that runs using a set of validator test nodes.
- The devnet is a demonstration of the Diem network that is built for experimenting with new ideas
- The devnet simulates a digital payment system and the coins on the devnet have _no real world value_.
- The devnet is the network by which developers are given the opportunity to test given protocols. It is similar to testnet as it operates independently of the mainnet yet is reset weekly.

## E

### Ed25519

- **Ed25519** is our supported digital signature scheme.
- More specifically, the Diem network uses the PureEdDSA scheme over the Ed25519 curve, as defined in RFC 8032.

### Event

- An **event** is the user-facing representation of the effects of executing a transaction.
- A transaction may be designed to emit any number of events as a list. For example, a `Coin<DiemCoin>` transfer emits a `WithdrawEvent` for the sender account and a `DepositEvent` for the recipient account.
- In the Diem protocol, events provide evidence that the successful execution of a transaction resulted in a specific effect. The `DepositEvent` (in the above example) allows the recipient to confirm that a payment was received into their account.
- Events are persisted on the blockchain and are used to answer queries by [clients](#client).

### Execution

- **Execution** in the Diem blockchain is an Diem node component that manages the block of transactions. The execution component stores successful transactions.

### Expiration Time

A transaction ceases to be valid after its **expiration time**. If it is assumed that:

- Time_C is the current time that is agreed upon between validators (Time_C is not the local time of the client);
- Time_E is the expiration time of a transaction T_N; and
- Time_C > Time_E and transaction T_N has not been included in the blockchain,

then there is a guarantee that T_N will never be included in the blockchain.

## F

### Faucet

- **Faucet** is a service that mints APT on devnet and testnet. APT on these networks has no real world value, it is only for development purposes.
- You can use the faucet in a few different ways:
  - With the [Diem CLI](../tools/diem-cli-tool/use-diem-cli.md#fund-an-account-with-the-faucet).
  - Through a wallet, such as Petra, Martian, or Pontem. You can find a full list [here](https://github.com/diem-foundation/ecosystem-projects#wallets).
  - Using an SDK, for example by using the `FaucetClient` in the TypeScript SDK.
  - With a direct HTTP request. Learn how to do this [here](guides/system-integrators-guide.md#calling-the-faucet-other-languages).

### Fullnodes

- **Fullnodes** are clients that ensure data are stored up-to-date on the network. They replicate blockchain state and transactions from other fullnodes and validator nodes.

### Fungible Asset

- A **fungible asset** is an asset, such as a currency, share, in-game resource, etc., that is interchangeable with another identical asset without any loss in its value. For example, APT is a fungible asset because you can exchange one APT for another.
- Follow the [Digital Asset Standards](../standards/index.md#digital-asset-standards) to create fungible assets on the Diem blockchain. 
- Next generation of the Coin standard that addresses shortcomings of `diem_framework::coin` such as lack of guaranteed enforcement of freeze and burn and advanced functionalities such as programmable transfers, e.g., approve in ERC-20.

### Fungible Token

- For TokenV1 (diem_token::token), a **fungible token** is a token that is interchangeable with other identical tokens (i.e., tokens that share the same `TokenId`). This means the tokens have the same `creator address`, `collection name`, `token name`, and `property version`.
- For TokenV2 (diem_token_objects::token), a **fungible token** is a fungible asset with metadata object includes a TokenV2 resource.

### Fungible Unit

- A **fungible unit** is an individual unit of a fungible asset. These units are identical and interchangeable without any loss in value. For example, each Octa (the smallest unit of APT) is a fungible unit.

## G

### Gas

- **Gas** is a way to pay for computation and storage on a blockchain network. All transactions on the Diem network cost a certain amount of gas.
- The gas required for a transaction depends on the size of the transaction, the computational cost of executing the transaction, and the amount of additional global state created by the transaction (e.g., if new accounts are created).
- The purpose of gas is regulating demand for the limited computational and storage resources of the validators, including preventing denial of service (DoS) attacks.

### Gas Price

- Each transaction specifies the **gas price** the sender is willing to pay. Gas price is specified in currency/gas units.
- The price of gas required for a transaction depends on the current demand for usage of the network.
- The gas cost is fixed at a point in time. Gas costs are denominated in gas units.

## H

### Honest (Validator)

- **Honesty** means a validator that faithfully executes the consensus protocol and is not Byzantine.

### Jolteon

- **Jolteon** is a recent proposal for a [BFT](#byzantine-fault-tolerance-bft) consensus protocol.
- DiemBFT, the Diem network's consensus algorithm, is based on Jolteon.
- It simplifies the reasoning about safety, and it addresses some performance limitations of previous consensus protocols. In particular, it reduces latency by 33% compared to HotStuff.

## I

### Indexer

- **[Indexer](../integration/indexing.md)** is the component of Diem that retrieves, processes, and efficiently stores raw data in the database to provide speedy access to the Diem blockchain state.

## L

### Leader

- A **leader** is a validator that proposes a block of transactions for the consensus protocol.
- In leader-based protocols, nodes must agree on a leader to make progress.
- Leaders are selected by a function that takes the current [round number](https://fb.quip.com/LkbMAEBIVNbh#ffYACAO6CzD) as input.

## M

### Mainnet

- **Mainnet** refers to a working, fully-operational blockchain. A mainnet network has been fully deployed and performs the functionality of transferring digital currency from a sender to a recipient.

### Maximum Gas Amount

- The **Maximum Gas Amount** of a transaction is the maximum amount of gas the sender is ready to pay for the transaction.
- The gas charged is equal to the gas price multiplied by units of gas required to process this transaction. If the result is less than the max gas amount, the transaction has been successfully executed.
- If the transaction runs out of gas while it is being executed or the account runs out of balance during execution, then the sender will be charged for gas used and the transaction will fail.

### Mempool

- **Mempool** is one of the components of the validator. It holds an in-memory buffer of transactions that have been submitted but not yet agreed upon and executed. Mempool receives transactions from [JSON-RPC Service](#json-rpc-service).
- Transactions in the mempool of a validator are added from the JSON-RPC Service of the current node and from the mempool of other Diem nodes.
- When the current validator is the leader, its consensus component pulls the transactions from its mempool and proposes the order of the transactions that form a block. The validator quorum then votes on the proposal.

### Merkle Trees

- **Merkle tree** is a type of authenticated data structure that allows for efficient verification of data integrity and updates.
- The Diem network treats the entire blockchain as a single data structure that records the history of transactions and states over time.
- The [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree) implementation simplifies the work of apps accessing the blockchain. It allows apps to:
  - Read any data from any point in time.
  - Verify the integrity of the data using a unified framework.

### Merkle Accumulator

- The **[Merkle Accumulator](https://www.usenix.org/legacy/event/sec09/tech/full_papers/crosby.pdf)** is an _append-only_ Merkle tree that the Diem blockchain uses to store the ledger.
- Merkle accumulators can provide proofs that a transaction was included in the chain (“proof of inclusion”).
- They are also called "history trees" in literature.

### Module

- A **module** in the Move programming language may either be a program or library that can create, transfer, or store assets.

### Move

- **Move** is a new programming language that implements all the transactions on the Diem blockchain.
- It has two different kinds of code &mdash; [transaction scripts](#transaction-script) and [Move modules](#move-module).
- Move is a safe and secure programming language for web3 that emphasizes access control and scarcity. It is the programming language used to build the Diem blockchain. You can read more about it in [Move on Diem](../move/move-on-diem.md).

### Move Bytecode

- Move programs are compiled into **Move bytecode**.
- Move bytecode is used to express transaction scripts and Move modules.

### Move Module

- A **Move module** defines the rules for updating the global state of the Diem blockchain.
- In the Diem protocol, a Move module is a **smart contract**.
- Each user-submitted transaction includes a transaction script. The transaction script invokes procedures of one or more Move modules to update the global state of the blockchain according to the rules.

### Move Resources

- **Move resources** contain data that can be accessed according to the **procedures** declared in a Move **module.**
- Move resources can never be copied, reused, or lost. This protects Move programmers from accidentally or intentionally losing track of a resource.

### Move Virtual Machine (MVM)

- The **Move virtual machine** executes transaction scripts written in [Move bytecode](#move-bytecode) to produce an [execution result](#execution-result). This result is used to update the blockchain **state**.
- The virtual machine is part of a [validator](#validator).
- The Move virtual machine (MoveVM) processes each validator node that translates transactions along with the current blockchain ledger state to produce a changeset as input or storage delta as output.

## N

### Node

- A **node** is a peer entity of the Diem network that tracks the state of the Diem blockchain.
- An Diem node consists of logical components. [Mempool](#mempool), [consensus](#consensus), and the [virtual machine](#virtual-machine) are examples of node components.

### Nonce

- **Nonce** is a number only used once, a random or semi-random number that is generated for a specific use for authentication protocols and cryptographic hash functions.

## O

### Open-Source Community

- **Open-source community** is a term used for a group of developers who work on open-source software. If you're reading this glossary, then you are part of the Diem project's developer community.

## P

### Proof

- A **proof** is a way to verify the accuracy of data in the blockchain.
- Every operation in the Diem blockchain can be verified cryptographically that it is indeed correct and that data has not been omitted.
- For example, if a user queries the information within a particular executed transaction, they will be provided with a cryptographic proof that the data returned to them is correct.

### PoS

**Proof-of-Stake (PoS)** is a security mechanism that serves in confirming the uniqueness and legitimacy of blockchain transactions. The PoS consensus mechanism is leveraged by the Diem blockchain powered by a network of validators, which in turn update the system and process transactions.

## R

### Resource Account

- A **resource account** is used to manage resources independent of an account managed by a user. For example, a developer may use a resource account to manage an account for module publishing, say managing a contract.

- The contract itself does not require a signer post initialization. A resource account gives you the means for the module to provide a signer to other modules and sign transactions on behalf of the module.

See [Resource accounts](../move/move-on-diem/resource-accounts.md) for instructions on use.

### REST Service

- The **REST Service** component is the external interface of a Diem node. Any incoming client request, such as submitted transactions or queries, must first go through the REST Service. A client needs to go through the REST Service component to access storage or any other component in the system. This filters requests and protects the system.
- Whenever a client submits a new transaction, the REST Service passes it to [mempool](#mempool).

### Round

- A **round** consists of achieving consensus on a block of transactions and their execution results.

### Round Number

- A **round number** is a shared counter used to select leaders during an [epoch](#epoch) of the consensus protocol.

## S

### SDKs

- Diem **software development kits (SDKs)** are sets of tools that enable a developer to quickly create a custom app on the Diem platform. Find out more at [Use the Diem SDKs](../sdks/index.md).

### Sequence Number

- The **sequence number** for an account indicates the number of transactions that have been submitted and committed on chain from that account. It is incremented every time a transaction sent from that account is executed or aborted and stored in the blockchain.
- A transaction is executed only if it matches the current sequence number for the sender account. This helps sequence multiple transactions from the same sender and prevents replay attacks.
- If the current sequence number of an account A is X, then a transaction T on account A will only be executed if T's sequence number is X.
- These transactions will be held in mempool until they are the next sequence number for that account (or until they expire).
- When the transaction is applied, the sequence number of the account will become X+1. The account has a strictly increasing sequence number.

### Sender

- _Alternate name_: Sender address.
- **Sender** is the address of the originator account for a transaction. A transaction must be signed by the originator.

### Smart Contract

- **Smart contract** refers to a computer program that automatically and directly carries out the contract's terms.
- See [Move Module](#move-module) for related details.

### State

- A **state** in the Diem protocol is a snapshot of the distributed database.
- A transaction modifies the database and produces a new and updated state.

### State Root Hash

- **State root hash** is a [Merkle hash](https://en.wikipedia.org/wiki/Merkle_tree) over all keys and values the state of the Diem blockchain at a given version.

## T

### Table

- A [**table**](https://github.com/aptos-labs/diem-core/blob/main/diem-move/framework/diem-stdlib/doc/table.md) implements the Table type and in Diem is used to store information as key-value data within an account at large scale.

See [`table.move`](https://github.com/aptos-labs/diem-core/blob/main/diem-move/framework/diem-stdlib/sources/table.move) for the associated Diem source file.

### Testnet

- **Testnet** describes the Diem network that is not fully functional yet more stable than devnet; it is an alternative network to mainnet to be used for testing.

### Tokens

- **Tokens** are digital units of value issued on a blockchain. They can be redeemed for assets or value held. Tokens can be of the types: Fungible Token (FT), Non-Fungible Token (NFT), and Semi-Fungible Token (SFT).

### Transaction

- A raw **transaction** contains the following fields:
  - [Sender (account address)](#account-address)
  - [Transaction script](#transaction-script)
  - [Gas price](#gas-price)
  - [Maximum gas amount](#maximum-gas-amount)
  - [Sequence number](#sequence-number)
  - [Expiration time](#expiration-time)
- A signed transaction is a raw transaction with the digital signature.
- An executed transaction changes the state of the Diem blockchain.

### Transaction (or Move) Script

- Each transaction submitted by a user includes a **transaction script**.
- These transactions, also know as Move scripts, represent the operations a client submits to a validator.
- The operation could be a request to move coins from user A to user B, or it could involve interactions with published [Move modules](#move-module) (smart contracts).
- The transaction script is an arbitrary program that interacts with resources published in the global storage of the Diem blockchain by calling the procedures of a module. It encodes the logic for a transaction.
- A single transaction script can send funds to multiple recipients and invoke procedures from several different modules.
- A transaction script **is not** stored in the global state and cannot be invoked by other transaction scripts. It is a single-use program.

To see example uses of transaction scripts, follow [Move scripts](../move/move-on-diem/move-scripts.md) and the [Your First Multisig](../tutorials/first-multisig.md) tutorial.

## V

### Validator

- _Alternate name_: Validators.
- A **validator** is an entity of the Diem ecosystem that validates on the Diem blockchain. It receives requests from clients and runs consensus, execution, and storage.
- A validator maintains the history of all the transactions on the blockchain.
- Internally, a validator needs to keep the current state, to execute transactions, and to calculate the next state.
- Diem validators are in charge of verifying transactions.

### Validator Nodes

- **Validator nodes** are a unique class of fullnodes that take part in consensus, specifically a Byzantine Fault Tolerance (BFT) consensus protocol in Diem. Validators agree upon transactions to be added to the Diem blockchain as well as the order in which they are added. 

### Version

- A **version** is also called “height” in blockchain literature.
- The Diem blockchain doesn't have an explicit notion of a block &mdash; it only uses blocks for batching and executing transactions.
- A transaction at height 0 is the first transaction (genesis transaction), and a transaction at height 100 is the 101st transaction in the transaction store.

## W

### Well-Formed Transaction

An Diem transaction is **well formed** if each of the following conditions are true for the transaction:

- The transaction has a valid signature.
- An account exists at the sender address.
- It includes a public key, and the hash of the public key matches the sender account's authentication key.
- The sequence number of the transaction matches the sender account's sequence number.
- The sender account's balance is greater than the [maximum gas amount](#maximum-gas-amount).
- The expiration time of the transaction has not passed.
