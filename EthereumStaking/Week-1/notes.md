# Week 1 – Ethereum Staking & Consensus Foundations

Week 1 focuses on building a **first-principles understanding of Ethereum staking**, why it exists, and what security guarantees it provides.

The goal is to move from:

> “Ethereum is a decentralized ledger”

to:

> “Ethereum is a cryptoeconomic system that enforces behavior through incentives and penalties.”

---

## What Is Staking?

Staking is a mechanism where participants:

- lock capital (ETH)
- perform protocol-defined duties
- earn rewards for correct behavior
- get penalized (slashed) for malicious behavior

The core idea:

> **Economic risk replaces trust.**

Instead of promising honesty, validators risk losing money.

---

## Why Staking Exists

Staking solves the problem of **honest participation** in a decentralized system.

If validators:

- act honestly → they earn rewards
- act maliciously → they lose stake

This makes attacks economically irrational.

---

## Role of Ethereum Validators

Ethereum validators are responsible for securing:

1. **Block proposal**
2. **Block ordering**
3. **State execution**
4. **Finality**

Validators:

- propose blocks
- attest to blocks proposed by others
- vote on correct chain history

---

## How Consensus Works (High Level)

Ethereum uses Proof of Stake with:

- block proposals
- attestations
- finality through voting

Simplified flow:

1. A validator proposes a block
2. Other validators attest to it
3. Once enough attestations are collected, the block becomes finalized
4. Finalized blocks cannot be reverted without slashing

---

## What Is Slashing?

Slashing is a **penalty mechanism** that:

- destroys a portion of validator stake
- forcibly exits the validator from the network

Slashing is designed to punish **malicious behavior**, not mistakes.

---

## Common Slashing Conditions

Examples of slashable behavior:

### 1. Double Proposal

- Proposing more than one block for the same slot

### 2. Double Voting

- Attesting to two conflicting blocks

### 3. Surround Voting

- Attesting in a way that violates finality rules

These actions:

- prevent consensus
- threaten network safety
- are objectively provable

---

## Where Staking Rewards Come From

Validator rewards are derived from:

1. **Transaction fees**

   - Small fees paid by users

2. **MEV (Maximal Extractable Value)**
   - Value from transaction ordering
   - Priority fees paid to validators

Rewards incentivize:

- uptime
- correct participation
- protocol-aligned behavior

---

## Risks in Ethereum Staking

Even with staking, risks still exist:

### 1. 51% Attack

If an attacker controls a majority of stake:

- they can censor transactions
- disrupt finality

However:

- the cost is extremely high
- attacker risks massive slashing

---

### 2. Mass Validator Exit

If many validators exit simultaneously:

- network liveness may degrade
- but funds remain safe

Ethereum prioritizes **safety over liveness**.

---

## Staking vs Stable Assets

Staked ETH derivatives (e.g., stETH):

- are market-traded assets
- not stablecoins
- can depeg due to market conditions

This is normal and expected.

---

## Key Mental Models:

- Ethereum does not rely on trust
- Validators are economically incentivized actors
- Slashing enforces safety, not correctness
- Finality makes history expensive to rewrite
- Security comes from **risk**, not promises

---

## Week 1 Takeaway

> Ethereum staking is not about rewards.  
> It is about enforcing correct behavior through economic penalties.
