With your account structure defined, you now need to create the context that
governs how proposals are initialized. This stage focuses on defining the
relationships and constraints between accounts during proposal creation.

## Prerequisite Reading

To understand account contexts in Anchor, review:

- **Anchor Account Constraints**: Learn about the various constraints that
  control account behavior. The [Anchor Account Constraints
  Documentation](https://www.anchor-lang.com/docs/references/account-constraints)
  explains `init`, `mut`, `seeds`, and other important constraints.
- **Solana Account Types**: Understand different account roles including Signer,
  Program, and System accounts. Read about [Solana Account
  Types](https://solana.com/docs/core/transactions#account-keys) to understand
  their purposes.
- **Account Initialization Patterns**: Learn how accounts are created and
  initialized on Solana. The [Program Derived Addresses
  Documentation](https://solana.com/docs/core/pda) explains secure account
  creation patterns.

## Implement the InitializeProposal Context

Add the following context structure to your program:

```rust
#[derive(Accounts)]
pub struct InitializeProposal<'info> {
    #[account(
        init,
        payer = user,
        space = 9000,
        seeds = [b"proposal", user.key().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

## Understanding the Context Structure

Let's examine each component and its purpose:

- **`#[derive(Accounts)]`**: This macro tells Anchor that this struct defines
  account relationships for a transaction.

- **`proposal: Account<'info, Proposal>`**: The main account being created. The
  constraints define how it's initialized:
  - **`init`**: Creates a new account
  - **`payer = user`**: Specifies which account pays for the account creation (rent)
  - **`space = 9000`**: Allocates initial storage space (we'll refine this later)
  - **`seeds = [b"proposal", user.key().as_ref()]`**: Defines how the PDA is derived
  - **`bump`**: Automatically finds and stores the valid bump seed

- **`user: Signer<'info>`**: The user creating the proposal. The
  `#[account(mut)]` makes it mutable since it will pay for account creation.

- **`system_program: Program<'info, System>`**: Required for account creation
  operations. The System Program handles fundamental account operations.

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Context compiles | No syntax errors | Ensures proper Rust syntax |
| Proper constraints | init, payer, space, seeds, bump | Validates all required constraints are present |
| Account relationships | Correct account types and mutability | Confirms proper account setup |

## Notes

- The `init` constraint is essential for creating new accounts
- `payer` specifies who bears the cost of account storage (rent)
- `space` is temporarily set to 9000 bytes; we'll calculate this precisely in the next stage
- `seeds` define how the PDA is derived from the user's public key
- `bump` ensures we use a valid bump seed for the PDA
- The System Program is always required for account creation operations
