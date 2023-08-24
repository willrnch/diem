---
title: "Deployments"
slug: "deployments"
hide_table_of_contents: true
---

# Diem Deployments

|Description                                 |Mainnet | Testnet | Devnet |
|--------------------------------------------|---|---|---|
|**REST API**             | https://fullnode.mainnet.diemlabs.com/v1 | https://fullnode.testnet.diemlabs.com/v1 | https://fullnode.devnet.diemlabs.com/v1 |
|**REST API Spec**        | <a href="https://fullnode.mainnet.diemlabs.com/v1/spec#/">Link</a> | <a href="https://fullnode.testnet.diemlabs.com/v1/spec#/">Link</a> | <a href="https://fullnode.devnet.diemlabs.com/v1/spec#/">Link</a> |
|**Indexer API**          | https://indexer.mainnet.diemlabs.com/v1/graphql | https://indexer-testnet.staging.gcp.diemdev.com/v1/graphql | https://indexer-devnet.staging.gcp.diemdev.com/v1/graphql |
|**Indexer API Spec**     | <a href="https://cloud.hasura.io/public/graphiql?endpoint=https://indexer.mainnet.diemlabs.com/v1/graphql">Link</a> | <a href="https://cloud.hasura.io/public/graphiql?endpoint=https://indexer-testnet.staging.gcp.diemdev.com/v1/graphql">Link</a> | <a href="https://cloud.hasura.io/public/graphiql?endpoint=https://indexer-devnet.staging.gcp.diemdev.com/v1/graphql">Link</a> |
|**Faucet**               | No Faucet | https://faucet.testnet.diemlabs.com/ | https://faucet.devnet.diemlabs.com/ |
|**Genesis and Waypoint** | https://github.com/aptos-labs/diem-networks/tree/main/mainnet | https://github.com/aptos-labs/diem-networks/tree/main/testnet| https://github.com/aptos-labs/diem-networks/tree/main/devnet |
|**Chain ID**             | 1 | 2 | [On Diem Explorer **select Devnet from top right**](https://explorer.diemlabs.com/?network=Devnet).|
|**Epoch duration**       | 7200 seconds |7200 seconds | 7200 seconds |
|**Network providers**    | Fully decentralized. | Managed by Diem Labs on behalf of Diem Foundation. | Managed by Diem Labs on behalf of Diem Foundation. |
|**Release cadence**      | Monthly | Monthly | Weekly |
|**Wipe cadence**         | Never. | Never. | On update. |
|**Purpose**              | The main Diem network. | Long-lived test network. | Bleeding edge and exploratory. |
|**Network status**       | Always live. | Always live. | Almost always live, with brief interruptions during updates. |
