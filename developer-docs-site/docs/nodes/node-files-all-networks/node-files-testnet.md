---
title: "Node Files For Testnet"
slug: "node-files-testnet"
---

# Node Files For Testnet

When you are deploying an Diem node in the **testnet**, you will need to download the files listed on this page. 

- **Mainnet:** If you are deploying in the mainnet, download the files from the [Node Files For Mainnet](./node-files.md) page.
- **Devnet:** If you are deploying in the testnet, download the files from the [Node Files For Devnet](./node-files-devnet.md) page.

---

These files can be downloaded from separate `aptos-labs` repos on GitHub. The `wget` commands provided below will work on macOS and Linux. Open a terminal and paste the `wget` command to download the file. 

:::tip Files for the validator node
Unless specified, all these files are required for validator node. A file with `fullnode` in its filename is required for either a validator fullnode or a public fullnode.
:::

## docker-compose.yaml

- **Git repo:** `diem-core`
- **Git branch:** `testnet` on https://github.com/aptos-labs/diem-core
- **Command to download:**
    ```bash
    wget -O docker-compose.yaml https://raw.githubusercontent.com/aptos-labs/diem-core/testnet/docker/compose/diem-node/docker-compose.yaml
    ```

## validator.yaml

- **Git repo:** `diem-core`
- **Git branch:** `testnet` on https://github.com/aptos-labs/diem-core
- **Command to download:**
  ```bash
  wget -O validator.yaml https://raw.githubusercontent.com/aptos-labs/diem-core/testnet/docker/compose/diem-node/validator.yaml
  ```

## genesis.blob 

- **Git repo:** `diem-networks`
- **Git branch:** `main` on https://github.com/aptos-labs/diem-networks
- **Command to download:**
  ```bash
  wget -O genesis.blob https://raw.githubusercontent.com/aptos-labs/diem-networks/main/testnet/genesis.blob
  ```

## waypoint.txt

- **Git repo:** `diem-networks`
- **Git branch:** `main` on https://github.com/aptos-labs/diem-networks
- **Command to download:**
  ```bash
  wget -O waypoint.txt https://raw.githubusercontent.com/aptos-labs/diem-networks/main/testnet/waypoint.txt
  ```

## docker-compose-src.yaml

- **Git repo:** `diem-core`
- **Git branch:** `testnet` on https://github.com/aptos-labs/diem-core
- **Command to download:**
  ```bash
  wget -O docker-compose-src.yaml https://raw.githubusercontent.com/aptos-labs/diem-core/testnet/docker/compose/diem-node/docker-compose-src.yaml
  ```

## haproxy.cfg

- **Git repo:** `diem-core`
- **Git branch:** `testnet` on https://github.com/aptos-labs/diem-core
- **Command to download:**
  ```bash
  wget -O haproxy.cfg https://raw.githubusercontent.com/aptos-labs/diem-core/testnet/docker/compose/diem-node/haproxy.cfg
  ```

## blocked.ips 

- **Git repo:** `diem-core`
- **Git branch:** `testnet` on https://github.com/aptos-labs/diem-core
- **Command to download:**
  ```bash
  wget -O blocked.ips https://raw.githubusercontent.com/aptos-labs/diem-core/testnet/docker/compose/diem-node/blocked.ips
  ```

## docker-compose-fullnode.yaml (fullnode only)

:::tip Fullnode 
Fullnode means either a validator fullnode or a public fullnode.
:::

- **Git repo:** `diem-core`
- **Git branch:** `testnet` on https://github.com/aptos-labs/diem-core
- **Command to download:**
  ```bash
  wget -O docker-compose-fullnode.yaml https://raw.githubusercontent.com/aptos-labs/diem-core/testnet/docker/compose/diem-node/docker-compose-fullnode.yaml
  ```

## fullnode.yaml (fullnode only)

:::tip Fullnode 
Fullnode means either a validator fullnode or a public fullnode.
:::

- **Git repo:** `diem-core`
- **Git branch:** `testnet` on https://github.com/aptos-labs/diem-core
- **Command to download:**
  ```bash
  wget -O fullnode.yaml https://raw.githubusercontent.com/aptos-labs/diem-core/testnet/docker/compose/diem-node/fullnode.yaml
  ```

## haproxy-fullnode.cfg (fullnode only)

- **Git repo:** `diem-core`
- **Git branch:** `testnet` on https://github.com/aptos-labs/diem-core
- **Command to download:**
  ```bash
  wget -O haproxy-fullnode.cfg https://raw.githubusercontent.com/aptos-labs/diem-core/testnet/docker/compose/diem-node/haproxy-fullnode.cfg
  ```
