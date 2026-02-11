# Day 3

## Breaking a Restaking Protocol & Analyzing Real AVSs

Day 3 focuses on:

- Designing a _bad_ restaking protocol intentionally
- Understanding why high yield often hides structural risk
- Connecting theory to real-world AVSs (e.g., Lido + Restaking)
- Identifying objective vs subjective slashing in production systems

---

# Part 1: Designing a Bad Restaking AVS (Intentionally)

## Hypothetical AVS: Yield-Optimized Data Availability Network

### Claims

- Provides decentralized data availability
- Secured by restaked ETH
- Offers 20–35% APY

### Slashing Rule (Flawed)

> Validators are slashed if data is later proven unavailable.

---

## Why This Design Is Dangerous

### 1. Data Availability Is Not Objectively Verifiable

- Depends on network conditions
- May vary across observers
- Cannot deterministically prove non-availability on-chain
- Absence of data ≠ cryptographic proof of fault

This makes slashing subjective.

---

### 2. Slashing Depends on Future Events

If slashing depends on:

- later observations
- post-mortem investigations
- governance review

Then validators face **retroactive liability**.

This violates a core safety rule:

> Slashing must depend only on validator actions, not outcomes.

---

### 3. Governance Becomes a Slashing Authority

If a committee decides:

- whether data was unavailable
- whether slashing should occur

Then:

- slashing power becomes political
- validators face unpredictable downside
- security degrades long-term

Governance feels safe early but weakens credibility over time.

---

### 4. Correlated Slashing Risk

If:

- many validators run the same client
- same failure condition triggers
- same slashing logic applies

Then:

- mass slashing event possible
- restaked capital wiped simultaneously
- systemic contagion spreads

---

# Cost of Corruption Analysis (Hypothetical)

Example numbers:

- Total restaked: $500M
- Slashing severity: 20%
- Cost to attacker: $100M
- Extractable value: $300M+

Result:

> Cost of corruption < profit  
> → Protocol is economically insecure.

TVL alone does not guarantee safety.

---

# Yield Insight

High yield often compensates for:

- undefined slashing liability
- governance discretion
- correlated tail risk
- systemic contagion

Yield is not free income.  
It is payment for absorbing structural risk.

---

# Part 2: Real-World Connection — Lido + Restaking

## What Lido Alone Secures

Lido staking:

- Secures Ethereum consensus
- Slashing tied to objective validator misbehavior
- Deterministic on-chain enforcement

This is objective security.

---

## What Happens When stETH Is Restaked

When stETH is used in restaking systems:

- It becomes collateral for additional AVSs
- It inherits new slashing conditions
- Those conditions may depend on external systems

This introduces:

- multi-layer slashing exposure
- correlated risk
- possible subjective enforcement

---

## Where Objective Security Ends

| Layer              | Guarantee Type             |
| ------------------ | -------------------------- |
| Ethereum consensus | Objective                  |
| Lido staking       | Objective (Ethereum-based) |
| External AVS       | Depends on AVS design      |
| Combined system    | Mixed / Compounded risk    |

If an AVS defines slashing based on:

- outcome failures
- external truth
- governance decisions

Then:

> stETH holders absorb subjective risk.

---

# Core Principles Reinforced

1. Slashing must be objective and local.
2. Slashing must not depend on future information.
3. Governance should not be a primary enforcement mechanism.
4. Signaling risk is safer than punishing uncertainty.
5. High yield implies higher structural risk.

---

# Final Mental Model:

Protocols do not secure truth.  
They secure specific invariants.

Slashing enforces invariants.  
Signaling communicates uncertainty.

If slashing depends on outcomes instead of actions,
the system is structurally unsafe.
