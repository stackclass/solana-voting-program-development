This is the core stage where you implement the actual voting logic. You'll add
bounds validation to prevent invalid votes and safely update the proposal state.
This combines the context structure, error handling, and data manipulation
concepts from previous stages.

## Prerequisite Reading

To implement secure voting logic, review:

- **Array Bounds Checking**: Understand why bounds validation is crucial for
  security. Read about [Rust Vector Bounds
  Checking](https://doc.rust-lang.org/std/primitive.usize.html) and why it's
  important.
- **State Mutation Safety**: Learn patterns for safe state updates in smart
  contracts. The [Sealevel Attacks
  documentation](https://www.anchor-lang.com/docs/references/security-exploits)
  provides guidance on common security vulnerabilities and safe programming
  practices.
- **Anchor require! Macro**: Understand how to use the `require!` macro for
  condition checking. Review [Anchor Require Macro
  Documentation](https://www.anchor-lang.com/docs/errors#require) for examples.

## Implement the Vote Function

Add the following function to your program:

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

## Understanding the Voting Logic

Let's examine each component of the implementation:

- **`let proposal = &mut ctx.accounts.proposal;`**: Gets a mutable reference to
  the proposal account for updating vote counts.

- **`require!(`**: The Anchor macro for condition checking. If the condition
  fails, it returns the specified error.

- **`(option_index as usize) < proposal.votes.len()`**: The validation
  condition. It checks that the provided option index is within the valid range
  of options.

- **`VotingError::InvalidOptionIndex`**: The error to return if validation
  fails. This uses the custom error code you defined in the previous stage.

- **`proposal.votes[option_index as usize] += 1;`**: The actual vote update.
  Only reaches this line if validation passes.

- **`Ok(())`**: Returns success after the vote is recorded.

## Why Bounds Checking is Crucial

Bounds validation is essential for several reasons:

1. **Security**: Prevents out-of-bounds access that could corrupt memory or
   cause panics
2. **Data Integrity**: Ensures votes are only cast for valid options
3. **User Experience**: Provides clear feedback when users make invalid
   selections
4. **Cost Efficiency**: Prevents wasted computation on invalid operations

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Function compiles | No syntax errors | Ensures proper Rust syntax |
| Bounds checking | Prevents invalid indices | Validates security feature |
| Vote counting | Correctly increments votes | Confirms core functionality |
| Error handling | Proper error type usage | Ensures graceful failure handling |

## Notes

- Always use `require!` for condition checking in smart contracts
- Validate array indices before access to prevent panics and security issues
- Cast numeric types appropriately (u32 to usize for vector indexing)
- Increment vote counts safely after validation
