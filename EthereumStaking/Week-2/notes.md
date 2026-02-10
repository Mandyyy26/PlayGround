# Week 2 – Economic Security & Restaking Foundations

This week focuses on understanding **economic security**, **objective vs subjective guarantees**, and the **design philosophy behind restaking systems** like EigenLayer.

The goal is to move beyond smart contract–level security and reason about **protocol-level incentives, slashing, and failure modes**.

---

## Week 2 – Day 1

### What Ethereum Secures (and What It Intentionally Does Not)

### What Ethereum Validators Secure

Ethereum validators provide security guarantees for:

1. **Block ordering**
2. **State execution**
3. **Finality**

These guarantees are:

- deterministic
- objectively verifiable
- enforced by cryptography and consensus

### What Ethereum Does NOT Secure

Ethereum intentionally does _not_ secure:

- Oracles (off-chain data)
- Bridges (cross-chain state)
- Off-chain computation (e.g., ZK proofs, external services)

**Reason**:

- These introduce non-determinism
- Increase node load
- Create subjective slashing conditions

Ethereum chooses to remain:

> extremely secure at **on-chain computation**,  
> intentionally agnostic about **off-chain truth**

---

## Objective vs Subjective Security

### Objective

- Verifiable on-chain
- Same conclusion for all honest observers
- Safe for slashing

Examples:

- Double-signing
- Invalid state transition
- Invalid signature

### Subjective

- Depends on off-chain data or governance
- Ambiguous or externally controlled
- Dangerous for slashing

Examples:

- Oracle prices
- Human committee decisions
- External system failures

**Key rule**:

> Slashing must be objective, local, and predictable.

---

## Week 2 – Day 2

### Economic Security & Cost of Corruption

### Economic Security

Economic security asks:

> How expensive is it to break the system compared to the value gained?

Example:

- $1B cost to steal $100M → economically secure
- $80M cost to steal $2B → insecure

---

### Cost of Corruption (CoC)

**CoC** = total loss an attacker suffers when attempting to break the protocol.

It is a better metric than:

- TVL
- Market cap
- Number of validators

Because it measures:

> actual attacker downside

---

### Assets vs Liabilities

**Assets (for attacker)**:

- Extracted value
- Bribes
- MEV
- External gains

**Liabilities (for attacker)**:

- Slashed stake
- Forced exits
- Reputation loss

Security exists only if:

> liabilities > attacker profit

---

### Who Absorbs Losses When a Protocol Fails?

Possible loss bearers:

1. Validators (strong security)
2. Users (weak security)
3. Token holders (weak security)

Protocols where validators absorb loss provide the strongest guarantees.

---

## Why Early Protocols Offer High Yield

Early-stage protocols often offer high APY because:

- Security is weak initially
- They must attract validators
- Yield compensates for high risk

Yield is **payment for risk**, not free money.

---

## Week 2 – Day 3

### Restaking Fundamentals

### What Is Restaking?

Restaking allows:

- already-staked ETH
- to secure additional protocols (AVSs)
- in exchange for extra yield

This increases:

- capital efficiency
- systemic risk

---

### Staking vs Restaking

**Staking**:

- One asset → one protocol → one set of guarantees

**Restaking**:

- One asset → multiple protocols → multiple guarantees

If any system fails:

- the same ETH can be slashed

---

### Correlated Slashing

Occurs when:

- multiple validators fail simultaneously
- often due to shared software, governance, or off-chain dependencies

Correlated slashing is more dangerous than isolated slashing because:

- it looks like coordinated attack
- can wipe out large portions of stake

---

### Objective vs Subjective Slashing in Restaking

**Objective slashing**:

- double-signing
- equivocation
- invalid state transitions

**Subjective slashing**:

- oracle failures
- governance decisions
- off-chain disputes

Only **objective slashing** is considered safe in restaking systems.

---

### Why Restaking Yields Are Higher

Restakers are paid because:

- they provide security to additional protocols
- they take on extra tail risk
- slashing risk compounds across systems

Yield reflects **security responsibility**.

---

### Who Is Most at Risk in Restaking?

- Restakers (capital at risk)
- Often lowest in information hierarchy
- Vulnerable to governance abuse and subjective slashing

---

## Week 2 – Day 4

### EigenLayer Design Insights

### What EigenLayer Secures

EigenLayer can:

- secure AVSs with **objective rules**
- enforce non-equivocation
- pool economic security

Example:

- EigenDA

---

### What EigenLayer Does NOT Secure

EigenLayer does NOT secure:

- smart contract bugs
- AVS software bugs
- correctness of off-chain computation

Any AVS bug can still lead to slashing.

---

### Governance & Veto Committee

Governance is used to:

- resolve accidental slashing
- prevent mass slashing of honest validators

However:

- governance introduces subjectivity
- must be minimized and bounded

---

### Why EigenLayer Yields Exist

EigenLayer yields are justified by:

- pooled security
- capital efficiency
- increased systemic risk

---

## Week 2 – Day 5

### Designing Safer Restaking Systems

### Limiting Capital Concentration

Allowing validators to restake into:

- many high-value AVSs
  increases:
- centralization
- attack incentives

Better design:

- caps exposure
- limits correlated risk

---

### Avoiding Off-Chain Slashing Conditions

Off-chain dependencies:

- increase attack surface
- enable manipulation
- break objective guarantees

Subjective slashing is the fastest way to kill validator participation.

---

### Governance-Heavy Systems

Governance-heavy systems:

- reduce accidental slashing
- attract early capital
- but weaken cryptoeconomic guarantees

Governance should be:

- minimal
- defensive
- last-resort only

---

### My Personal Restaking Rule

A strong restaking rule:

> Never restake into systems with subjective slashing conditions.

---

## Core Takeaways :

- Blockchains secure **invariants**, not truth
- Slashing must be objective and local
- Cost of corruption > TVL
- Restaking compounds both yield and risk
- Signaling risk is safer than punishing uncertainty
