---
title: "Governance"
slug: "governance"
---
import ThemedImage from '@theme/ThemedImage';
import useBaseUrl from '@docusaurus/useBaseUrl';

# Governance

The Diem on-chain governance is a process by which the Diem community members can create and vote on proposals that minimize the cost of blockchain upgrades. The following describes the scope of these proposals for the Diem on-chain governance:

- Changes to the blockchain parameters, for example, the epoch duration, and the minimum required and maximum allowed validator stake.
- Changes to the core blockchain code. 
- Upgrades to the Diem Framework modules for fixing bugs or for adding or enhancing the Diem blockchain functionality.
- Deploying new framework modules (at the address `0x1` - `0xa`).

## How a proposal becomes ready to be resolved

See below for a summary description of how a proposal comes to exist and when it becomes ready to be resolved:

<ThemedImage
alt="Proposal voting flow"
sources={{
    light: useBaseUrl('/img/docs/voting-resolution-flow.svg'),
    dark: useBaseUrl('/img/docs/voting-resolution-flow-dark.svg'),
  }}
/>

- The  Diem community can suggest an Diem Improvement Proposal (AIP) in the [Diem Foundation AIP GitHub](https://github.com/diem-foundation/AIPs).
- When appropriate, an on-chain proposal can be created for the AIP via the `diem_governance` module. 
- Voters can then vote on this proposal on-chain via the `diem_governance` module. If there is sufficient support for a proposal, then it can be resolved.
- Governance requires a minimal number of votes to be cast by an expiration threshold. However, if sufficient votes, more than 50% of the total supply, are accumulated prior to that threshold, the proposal can be executed **without waiting for the full voting period**.

## Who can propose

- To either propose or vote, you must stake, but you are not required to run a validator node. However, we recommend that you run validator with a stake as part of the validator set to gain rewards from your stake.
- To create a proposal, the proposer's backing stake pool must have the minimum required proposer stake. The proposer's stake must be locked up for at least as long as the proposal's voting period. This is to avoid potential spammy proposals. 
- Proposers can create a proposal by calling [`diem_governance::create_proposal`](https://github.com/aptos-labs/diem-core/blob/27a255ebc662817944435349afc4ec33ea317e64/diem-move/framework/diem-framework/sources/diem_governance.move#L183).

## Who can vote

- To vote, you must stake, though you are not required to run a validator node. Your voting power is derived from the backing stake pool. 
- Voting power is calculated based on the current epoch's active stake of the proposer or voter's backing stake pool. In addition, the stake pool's lockup must be at least as long as the proposal's duration.
- Verify proposals before voting. Ensure each proposal is linked to its source code, and if there is a corresponding AIP, the AIP is in the title and description.

:::tip
Each stake pool can be used to vote on each proposal exactly only one time.
:::

## Who can resolve
- Anyone can resolve an on-chain proposal that has passed voting requirements by using the `diem governance execute-proposal` command from Diem CLI. 

## Diem Improvement Proposals (AIPs)

AIPs are proposals created by the Diem community or the Diem Labs team to improve the operations and development of the Diem chain. 
To submit an AIP, create an issue in [`Diem Foundation's GitHub repository`](https://github.com/diem-foundation/AIPs/issues) using the [template](https://github.com/diem-foundation/AIPs/blob/main/TEMPLATE.md)
To keep up with new AIPs, check the `#aip-announcements` channel on [Diem' discord channel](https://discord.gg/diemnetwork). 
To view and vote on on-chain proposals, go to [`Diem' Governance website`](https://governance.diemfoundation.org/). 

## Technical Implementation of Diem Governance
The majority of the governance logic is in [`diem_governance.move and voting.move`](https://github.com/aptos-labs/diem-core/blob/main/diem-move/framework/diem-framework/sources). 
The `diem_governance` module outlines how users can interact with Diem Governance. It's the external-facing module of the Diem on-chain governance process and contains logic and checks that are specific to Diem Governance.
The `voting` module is the Diem governance standard that can be used by DAOs on the Diem chain to create their own on-chain governance process.

If you are thinking about creating a DAO on Diem, you can refer to `diem_governance`'s usage of the `voting` module as an example. 
In `diem_governance`, we rely on the `voting` module to create, vote on, and resolve a proposal.
- `diem_governance::create_proposal` calls `voting::create_proposal` to create a proposal on-chain, when an off-chain AIP acquires sufficient importance. 
- `diem_governance::vote` calls `voting::vote` to record the vote on a proposal on-chain; 
- `diem_governance::resolve` can be called by anyone. It calls `voting::resolve` to resolve the proposal on-chain. 
