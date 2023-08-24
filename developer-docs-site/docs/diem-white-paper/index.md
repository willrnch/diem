---
title: "Diem White Paper"
---

# The Diem Blockchain
## A Safe, Scalable, and Upgradeable Web3 Infrastructure

Abstract

The rise of blockchains as a new Internet infrastructure has led to developers deploying tens of
thousands of decentralized applications at rapidly growing rates. Unfortunately, blockchain usage
is not yet ubiquitous due to frequent outages, high costs, low throughput limits, and numerous
security concerns. To enable mass adoption in the web3 era, blockchain infrastructure needs
to follow the path of cloud infrastructure as a trusted, scalable, cost-efficient, and continually
improving platform for building widely-used applications.

We present the Diem blockchain, designed with scalability, safety, reliability, and upgradeability
as key principles, to address these challenges. The Diem blockchain has been developed over the
past three years by over 350+ developers across the globe. It offers new and novel innovations
in consensus, smart contract design, system security, performance, and decentralization. The
combination of these technologies will provide a fundamental building block to bring web3 to the
masses:

- First, the Diem blockchain natively integrates and internally uses the Move language for fast
and secure transaction execution. The Move prover, a formal verifier for smart contracts
written in the Move language, provides additional safeguards for contract invariants and
behavior. This focus on security allows developers to better protect their software from
malicious entities.
- Second, the Diem data model enables flexible key management and hybrid custodial options.
This, alongside transaction transparency prior to signing and practical light client protocols,
provides a safer and more trustworthy user experience.
- Third, to achieve high throughput and low latency, the Diem blockchain leverages a pipelined
and modular approach for the key stages of transaction processing. Specifically, transaction
dissemination, block metadata ordering, parallel transaction execution, batch storage, and
ledger certification all operate concurrently. This approach fully leverages all available physical resources, improves hardware efficiency, and enables highly parallel execution.
- Fourth, unlike other parallel execution engines that break transaction atomicity by requiring
upfront knowledge of the data to be read and written, the Diem blockchain does not put
such limitations on developers. It can efficiently support atomicity with arbitrarily complex
transactions, enabling higher throughput and lower latency for real-world applications and
simplifying development.
- Fifth, the Diem modular architecture design supports client flexibility and optimizes for
frequent and instant upgrades. Moreover, to rapidly deploy new technology innovations
and support new web3 use cases, the Diem blockchain provides embedded on-chain change
management protocols.
- Finally, the Diem blockchain is experimenting with future initiatives to scale beyond individual validator performance: its modular design and parallel execution engine support internal
sharding of a validator and homogeneous state sharding provides the potential for horizontal
throughput scalability without adding additional complexity for node operators.

## Full PDF versions

:::tip Full PDF versions

- **English**: Get the [full PDF of the Diem White Paper](/papers/Diem-Whitepaper.pdf).
- **Korean**: Get the [Korean version full PDF of the Diem White Paper](/papers/whitepaper-korean.pdf).
:::
