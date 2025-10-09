On Solana every accont has an address. Normally addresses come from a keypair(pub+priv)

But sometimes we want an account that no one can sign for direrctly, and instead only a program can own and control it. THATS WHERE PDAs COME in.

PDA is an address that looks like a public key but it has no private key.
It is derived by the programs.

they are created deterministically with the function:

```
pda = findProgramAddress(seeds[],program_id)
```

If the result falls on the curve(where private key could exist), Solana bumps a number unitl it finds an address that doesnt correspond to any private key. (starting from 255)

User wallets = freedom (user controls).
Program PDAs = rules (program logic controls).

PDAs cant sign transaction with a private key, Instead programs use _invoke_signed_ with the same seeds to prove ownership.

PDAs helps to :
Securely store program state (vaults, escrows order books).
prevent users from bypassing program logic.
Deterministic address generation -> anyone can derive the address, no private key needed.
