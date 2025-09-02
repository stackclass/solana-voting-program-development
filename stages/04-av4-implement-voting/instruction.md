## Prerequisite Reading
- Solana Account Mutability
- Anchor Account Constraints
- Basic Access Control Patterns

## Learning Objectives
- Implement voting instructions
- Update on-chain state safely
- Add basic access control
- Handle index bounds checking

## Task
1. Add vote function:
```rust
pub fn vote(ctx: Context<Vote>, option_index: u32) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    // Basic bounds checking
    require!(
        (option_index as usize) < proposal.votes.len(),
        VotingError::InvalidOptionIndex
    );

    proposal.votes[option_index as usize] += 1;
    Ok(())
}
```

2. Define vote context:
```rust
#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub user: Signer<'info>,
}
```

3. Add error enum:
```rust
#[error_code]
pub enum VotingError {
    #[msg("Invalid option index")]
    InvalidOptionIndex,
}
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Vote function compiles | No syntax errors |
| Bounds checking | Prevents invalid indices |
| Vote counting | Correctly increments votes |
| Multiple votes | Handles multiple voters |

## Notes
- Always validate array indices
- Use require! for condition checking
- Define proper error codes

## Reference Links
- https://www.anchor-lang.com/docs/errors
- https://docs.rs/anchor-lang/latest/anchor_lang/macro.require.html
