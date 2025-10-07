1. Creating and Initialising accounts

```
#[account(init, payer = user, space = 8+8)]
pub my_account: Account<'info, MyAccount>,
```

this line alone tells Anchor to:
Allocate account space on chain,
Use user to pay rent,
Set ownership to your program's ID

we didnt need to call invke_signed or system_instruction::create_account , Anchor does it all.

2. PDAs in Anchor

Instead of manually calling Pubkey::find_program_address(), we declare it like:

```
#[account(
    init,
    seeds = [b"my_seed", user.key().as_ref()],
    bump,
    payer = user,
    space = 8+8
)]
pub my_account = Account<'info, MyAccount>,
```

Anchor:
Derives the PDA using those seeds + your program ID,
Validates it during execution,
Stores the bump automatically in your struct(If we include it)
