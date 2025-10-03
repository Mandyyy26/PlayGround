# Solana Transactions and Instructions

## Overview

- A user sends a **transaction** to interact with the Solana network.
- A transaction can contain **one or more instructions**.

An **instruction** can be thought of as a public function that can be called by anyone.

Each instruction requires:

1. **Program ID** – Address of the program to invoke
2. **Accounts** – The accounts the instruction reads from or writes to
3. **Instruction Data** – Any extra data required by the instruction

---

## Instruction Struct

```rust
pub struct Instruction {
    /// Pubkey of the program that executes this instruction.
    pub program_id: Pubkey,
    /// Metadata describing accounts that should be passed to the program.
    pub accounts: Vec<AccountMeta>,
    /// Opaque data passed to the program for its own interpretation.
    pub data: Vec<u8>,
}
```

A Solana transaction is made up of:

1. **Signature**
2. **Message**

### Transaction Struct

```rust
pub struct Transaction {
    #[wasm_bindgen(skip)]
    #[serde(with = "short_vec")]
    pub signatures: Vec<Signature>,

    #[wasm_bindgen(skip)]
    pub message: Message,
}
```

## Components of a Transaction Message

A transaction message in Solana consists of:

1. **Message Header**
2. **Account Addresses**
3. **Recent Blockhash**
4. **Instructions**

---

## Message Struct

```rust
pub struct Message {
    /// The message header, identifying signed and read-only `account_keys`.
    pub header: MessageHeader,

    /// All the account keys used by this transaction.
    #[serde(with = "short_vec")]
    pub account_keys: Vec<Pubkey>,

    /// The id of a recent ledger entry.
    pub recent_blockhash: Hash,

    /// Programs that will be executed in sequence and committed in
    /// one atomic transaction if all succeed.
    #[serde(with = "short_vec")]
    pub instructions: Vec<CompiledInstruction>,
}
```

## Transaction Size

- **Signatures**: Maximum 19, each **64 bytes**
- **Messages**: Metadata (**3 bytes**) + Accounts (**32 bytes each**)

---

## Message Header

The **Message Header** defines how many signatures are required and which accounts are read-only.

### MessageHeader Struct

```rust
pub struct MessageHeader {
    /// The number of signatures required for this message to be considered
    /// valid. The signers of those signatures must match the first
    /// `num_required_signatures` of [`Message::account_keys`].
    pub num_required_signatures: u8,

    /// The last `num_readonly_signed_accounts` of the signed keys are read-only
    /// accounts.
    pub num_readonly_signed_accounts: u8,

    /// The last `num_readonly_unsigned_accounts` of the unsigned keys are
    /// read-only accounts.
    pub num_readonly_unsigned_accounts: u8,
}
```
