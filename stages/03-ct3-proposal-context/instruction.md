## Prerequisite Reading
- Anchor Account Constraints
- Solana Account Types (Signer, Program, etc.)
- Account Initialization Patterns

## Learning Objectives
- Understand Anchor account constraints
- Define proper account relationships
- Set up context for proposal creation

## Task
1. Complete the InitializeProposal context structure:

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

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Context compiles | No syntax errors |
| Proper constraints | init, payer, space, seeds, bump |
| Account relationships | Correct account types and mutability |

## Notes
- `init` constraint creates the account
- `payer` specifies who pays for account creation
- `space` allocates sufficient storage
- `seeds` defines PDA derivation
- `bump` stores the bump seed for later use

## Reference Links
- https://www.anchor-lang.com/docs/account-constraints
- https://solana.com/docs/core/pdas
