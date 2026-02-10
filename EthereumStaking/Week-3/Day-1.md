# Week 3 – Protocol Design & Restaking Risk Reasoning

Week 3 focuses on **thinking like a protocol designer**:

- defining what a protocol actually secures
- drawing hard boundaries around responsibility
- designing slashing conditions that are safe
- understanding how restaking changes failure modes

The emphasis is on **mechanism design**, not code.

---

### Designing a Protocol from First Principles

### Core Shift in Thinking

Before:

> Protocols secure truth.

Now:

> Protocols secure **specific invariants**, not real-world truth.

This distinction is critical for safe cryptoeconomic design.

---

## Example Protocol: RWA Ownership Registry (Conceptual)

A protocol designed to track ownership history of real-world assets (e.g., land).

### What the Protocol Can Secure

- History cannot be rewritten
- Ownership changes must be properly signed
- State transitions follow protocol rules

### What the Protocol CANNOT Secure

- Whether the signer is the “real” owner in the physical world
- Whether off-chain legal documents are valid
- Whether fraud happened outside the chain

This limitation is intentional.

---

## Validator Responsibilities

Validators are responsible for:

- voting on state transitions
- updating state when selected
- preserving consistency of ownership history

They are **not** responsible for real-world truth.

---

## Correct Behavior

- Update state correctly when selected
- Vote consistently with protocol rules
- Never attempt to rewrite ownership history

---

## Misbehavior

- Invalid state transition
- Double voting
- Refusing to participate in consensus
- Attempting to modify historical state

---

## Objective Slashing Condition

**Invalid state transition**

Why this is safe:

- provable on-chain
- deterministic
- entirely within protocol scope

---

## Key Insight:

> A protocol should only slash for actions it can objectively verify  
> and only for invariants it explicitly claims to secure.

Trying to secure more leads to subjective slashing and system failure.

---
