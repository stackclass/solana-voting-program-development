## Prerequisite Reading
- Array Bounds Checking
- State Mutation Safety
- Anchor require! Macro

## Learning Objectives
- Implement secure voting logic
- Add bounds validation
- Handle state updates safely

## Task
1. Complete the vote function:

```rust
pub fn vote(ctx: Context<Vote>, option_index: u32) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    require!(
        (option_index as usize) < proposal.votes.len(),
        VotingError::InvalidOptionIndex
    );

    proposal.votes[option_index as usize] += 1;
    Ok(())
}
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Function compiles | No syntax errors |
| Bounds checking | Prevents invalid indices |
| Vote counting | Correctly increments votes |
| Error handling | Proper error type usage |

## Notes
- Use `require!` for condition checking
- Validate array indices before access
- Cast u32 to usize for vector indexing
- Increment vote count safely

## Reference Links
- https://www.anchor-lang.com/docs/errors#require
- https://doc.rust-lang.org/std/primitive.usize.html
