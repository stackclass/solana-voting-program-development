## Prerequisite Reading
- Account Mutability in Anchor
- Signer Account Requirements
- Reference Account Access

## Learning Objectives
- Define voting transaction context
- Set up proper account mutability
- Understand signer requirements

## Task
1. Complete the Vote context structure:

```rust
#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub user: Signer<'info>,
}
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Context compiles | No syntax errors |
| Proposal mutability | Proper mut constraint |
| User signer | Correct Signer type |

## Notes
- `#[account(mut)]` makes proposal account mutable
- `Signer<'info>` ensures user signed the transaction
- No PDA needed for voting (uses existing proposal)

## Reference Links
- https://www.anchor-lang.com/docs/account-constraints#mut
- https://www.anchor-lang.com/docs/account-types#signer
