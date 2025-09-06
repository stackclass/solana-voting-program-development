With proposal creation implemented, you now need to define the context for
voting transactions. This stage focuses on creating the account relationships
and constraints that govern how users cast votes on proposals.

## Prerequisite Reading

To understand voting context design, review:

- **Account Mutability in Anchor**: Learn how the `mut` constraint works and
  when to use it. The [Anchor Mut Constraint
  Documentation](https://www.anchor-lang.com/docs/references/account-constraints#accountmut)
  explains mutable account handling.
- **Signer Account Requirements**: Understand why certain transactions require
  signer accounts. Read about [Signer
  Accounts](https://www.anchor-lang.com/docs/references/account-types#signerinfo)
  in Anchor documentation.
- **Reference Account Access**: Learn how accounts reference each other in
  transactions. The [Solana Transaction
  Documentation](https://solana.com/docs/core/transactions) explains account
  relationships.

## Implement the Vote Context Structure

Add the following context structure to your program:

```rust
#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub user: Signer<'info>,
}
```

## Understanding the Voting Context

Let's examine each component and its purpose:

- **`#[derive(Accounts)]`**: This macro indicates that this struct defines
  account relationships for a voting transaction.

- **`proposal: Account<'info, Proposal>`**: The proposal account being voted on.
  The constraints:
  - **`#[account(mut)]`**: Makes the proposal account mutable, allowing vote
    counts to be updated
  - No PDA seeds needed here because we're modifying an existing account, not
    creating a new one

- **`user: Signer<'info>`**: The user casting the vote. This account:
  - Must sign the transaction to prove authorization
  - Doesn't need mutability since we're not modifying the user account
  - Provides authentication for the voting operation

## Why This Structure is Efficient

This minimal context structure is efficient because:
- Only necessary accounts are included
- No PDA derivation is needed for voting (uses existing proposal account)
- The user only needs to sign, not pay for storage
- Minimal account references reduce transaction complexity and cost

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Context compiles | No syntax errors | Ensures proper Rust syntax |
| Proposal mutability | Proper `mut` constraint | Validates that votes can be updated |
| User signer | Correct Signer type | Confirms authentication requirement |

## Notes

- `#[account(mut)]` is essential for modifying the proposal's vote counts
- `Signer<'info>` ensures only authorized users can cast votes
- No PDA needed here since we're working with an existing account
- Keep contexts minimal to reduce transaction costs and complexity
