In native Rust programs we must manually :
Decode instructions from bytes
Serialise/deserialize accounts
Check ownerships and permissions
Handle PDAs and seeds carefully.

ANCHOR abstracts all that.

Its a framework that gives us:
Declarative syntax
Auto serialization
Account validation
Client SDKs
CPI helpers

Bsically Anchor hides all the low level stuff.

Every Anchor program has 3 core parts:

1. libs.rs ---> the entry point defining all our instruction.
2. #[derive(Acccounts)] structs ---> define which accounts are needed and enforce access control.
3. State structs ---> Define how our on-chain data is stored.

Anchor's Core Macros:

1.  #[program] ---> defien our programs's entrypoint , cointains all instructions (functions).
2.  [derive(Accounts)] ---> Declares and validates which accounts are needed for a given instruction.
3.  #[account] ---> Marks a struct as on-chain account data (serialized and stored on Solana)
4.  #[error_code] ---> Defines you program-specific error messages
