## Prerequisite Reading
- Anchor Context and Accounts
- Solana PDAs (Program Derived Addresses)
- Error Handling in Anchor

## Learning Objectives
- Implement Solana program instructions
- Initialize on-chain accounts
- Handle input parameters safely
- Use PDAs for account addressing

## Task
1. Complete the initialize_proposal function:
```rust
pub fn initialize_proposal(
    ctx: Context<InitializeProposal>,
    title: String,
    options: Vec<String>
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    proposal.title = title;
    proposal.options = options.clone();
    proposal.votes = vec![0; options.len()];
    proposal.bump = *ctx.bumps.get("proposal").unwrap();
    Ok(())
}
```

2. Define context structure:
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
| Function compiles | No syntax errors |
| Account initialization | Proper PDA seeds |
| Data assignment | Title and options stored correctly |
| Votes initialization | All options start with 0 votes |

## Notes
- Ensure proper space allocation for account
- Use appropriate seeds for PDA
- Handle vector cloning properly

## Reference Links
- https://www.anchor-lang.com/docs/space
- https://solana.com/docs/core/pdas
