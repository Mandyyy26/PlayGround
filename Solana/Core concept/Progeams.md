# Solana Programs & Smart Contracts

## Smart Contracts

- **Smart Contracts === Programs**
- **Functions === Instructions**

- Programs are **stateless**.
- **Upgrade authority** can update programs. Once this authority is removed, the program becomes **immutable**.

### Writing Programs

Programs are written in **Rust**:

1. **Anchor**: Framework for Solana program development.
2. **Native Rust**: Writing programs in Rust without leveraging any frameworks.

---

## Berkeley Packet Filter (BPF)

- Solana uses **Low Level Virtual Machine (LLVM)** to compile programs into **ELF (Executable and Linkable Format)** files.

---

## Loader Programs

Every program is owned by another program called its **loader**.  
Currently, there are **5 loader programs**:

| Loader                            | Address                                       |
| --------------------------------- | --------------------------------------------- |
| **Native** (owns other 4 loaders) | `NativeLoader1111111111111111111111111111111` |
| **v1**                            | `BPFLoader1111111111111111111111111111111111` |
| **v2**                            | `BPFLoader2111111111111111111111111111111111` |
| **v3**                            | `BPFLoaderUpgradeab1e11111111111111111111111` |
| **v4**                            | `LoaderV411111111111111111111111111111111111` |

**Responsibilities of loaders:**

- Deploy a new program or buffer
- Close a program or buffer
- Redeploy / upgrade an existing program
- Transfer the authority over a program
- Finalize a program

---

## Core Programs

| Program                    | Address                                       | Description                                                                                                                                                                                                                                       |
| -------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **System Program**         | `11111111111111111111111111111111`            | Create new accounts, allocate account data, assign accounts to owning programs, transfer lamports from System Program–owned accounts, and pay transaction fees.                                                                                   |
| **Vote Program**           | `Vote111111111111111111111111111111111111111` | Create and manage accounts that track validator voting state and rewards.                                                                                                                                                                         |
| **Stake Program**          | `Stake11111111111111111111111111111111111111` | Create and manage accounts representing stake and rewards for delegations to validators.                                                                                                                                                          |
| **Config Program**         | `Config1111111111111111111111111111111111111` | Add configuration data to the chain, followed by the list of public keys that are allowed to modify it. Unlike the other programs, the Config program does not define individual instructions. It has just one implicit instruction: **"store"**. |
| **Compute Budget Program** | `ComputeBudget111111111111111111111111111111` | Set compute unit limits and prices for transactions, allowing users to control compute resources and prioritization fees.                                                                                                                         |

---
