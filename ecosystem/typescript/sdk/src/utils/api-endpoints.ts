export const NetworkToIndexerAPI: Record<string, string> = {
  mainnet: "https://indexer.mainnet.diemlabs.com/v1/graphql",
  testnet: "https://indexer-testnet.staging.gcp.diemdev.com/v1/graphql",
  devnet: "https://indexer-devnet.staging.gcp.diemdev.com/v1/graphql",
};

export const NetworkToNodeAPI: Record<string, string> = {
  mainnet: "https://fullnode.mainnet.diemlabs.com/v1",
  testnet: "https://fullnode.testnet.diemlabs.com/v1",
  devnet: "https://fullnode.devnet.diemlabs.com/v1",
};

export const NodeAPIToNetwork: Record<string, string> = {
  "https://fullnode.mainnet.diemlabs.com/v1": "mainnet",
  "https://fullnode.testnet.diemlabs.com/v1": "testnet",
  "https://fullnode.devnet.diemlabs.com/v1": "devnet",
};

export enum Network {
  MAINNET = "mainnet",
  TESTNET = "testnet",
  DEVNET = "devnet",
}

export interface CustomEndpoints {
  fullnodeUrl: string;
  indexerUrl?: string;
}
