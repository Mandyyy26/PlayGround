Smart Contracts === Programs
Functons === Instructions

programs are statelss
Upgrade authority can update programs. Once this authoriyt removed this, program becomes immutable.

Programs are written in rust:

1. Anchor: Framework for solana program development.
2. Native Rust: writing programs in rust without leveraging any frameworks.

Berkeley Packet Filter (BPF)

Solana usees low level virtual machine(LLVM) to compile programs into (ELF) exectable and linkable format files.

Loader Programs

every program itself is owned by another program which is its loader.
Currently 5 loader program exists:

native(owns other 4 loaders) NativeLoader1111111111111111111111111111111
v1 BPFLoader1111111111111111111111111111111111
v2 BPFLoader2111111111111111111111111111111111
v3 BPFLoaderUpgradeab1e11111111111111111111111
v4 LoaderV411111111111111111111111111111111111

these loaders are necessary to create and manage custom programs:
Deploy a new program or buffer
Close a program or buffer
Redeploy / upgrade an existing program
Transfer the authority over a program
Finalize a program

Core Programs

System Program 11111111111111111111111111111111  
Create new accounts, allocate account data, assign accounts to owning programs, transfer lamports from System Program owned accounts, and pay transaction fees.

Vote Program Vote111111111111111111111111111111111111111
Create and manage accounts that track validator voting state and rewards.

Stake Program Stake11111111111111111111111111111111111111
Create and manage accounts representing stake and rewards for delegations to validators.

Config Program Config1111111111111111111111111111111111111
Add configuration data to the chain, followed by the list of public keys that are allowed to modify it. Unlike the other programs, the Config program does not define any individual instructions. It has just one implicit instruction: "store". Its instruction data is a set of keys that gate access to the account and the data to store inside of it.

Compute Budget Program ComputeBudget111111111111111111111111111111
Set compute unit limits and prices for transactions, allowing users to control compute resources and prioritization fees.
