In this stage, you'll refine your account initialization by implementing precise
space calculation and proper PDA (Program Derived Address) handling. Accurate
space calculation is crucial for efficient on-chain storage and cost management.

## Prerequisite Reading

To understand space calculation and PDAs, review:

- **Program Derived Addresses (PDAs)**: Deepen your understanding of how PDAs
  work and why they're important for security. Read the [Solana PDA
  Documentation](https://solana.com/docs/core/pda) for comprehensive details.
- **Anchor Space Calculation**: Learn how Anchor calculates account space
  requirements. The [Anchor Space
  Documentation](https://www.anchor-lang.com/docs/references/space) explains the
  formula for different data types.
- **Transaction Fees**: Understand how Solana's transaction fee system works and
  why proper space allocation affects costs. Review [Solana Fees
  Documentation](https://solana.com/docs/core/fees) to learn about transaction
  pricing.

## Implement Precise Space Calculation

Update your `InitializeProposal` context with precise space calculation:

```rust
#[account(
    init,
    payer = user,
    space = 8 + 4 + title.len() + 4 + (options.iter().map(|opt| 4 + opt.len()).sum::<usize>()) + 4 + (options.len() * 4) + 1,
    seeds = [b"proposal", user.key().as_ref()],
    bump
)]
pub proposal: Account<'info, Proposal>,
```

## Understanding the Space Calculation

Let's break down each component of the space calculation:

- **`8` bytes**: Anchor discriminator - unique identifier added by Anchor to all
  accounts
- **`4 + title.len()`**: String storage - 4 bytes for length prefix + actual
  string content
- **`4 + (options.iter().map(|opt| 4 + opt.len()).sum::<usize>())`**: Vector of
  strings - 4 bytes for vector length + each string (4 bytes length + content)
- **`4 + (options.len() * 4)`**: Vector of u32 votes - 4 bytes for vector length
  + each u32 (4 bytes)
- **`1` byte**: u8 bump seed storage

This precise calculation ensures you only pay for the storage you actually need,
minimizing rent costs.

## Understanding PDA Security

The PDA initialization provides several security benefits:

- **Deterministic Addresses**: PDAs are derived from seeds, ensuring the same
  input always produces the same address
- **Program Control**: Only the deriving program can sign for PDA operations
- **No Private Key**: PDAs don't have private keys, enhancing security
- **`bump`**: The bump seed ensures we find a valid on-curve address

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Space calculation | Proper account size allocation | Ensures efficient storage usage |
| PDA seeds | Correct seed structure | Validates secure address derivation |
| Bump storage | Bump seed properly handled | Confirms security feature implementation |

## Notes

- Precise space calculation reduces storage costs and optimizes on-chain efficiency
- PDA derivation ensures secure, deterministic account addresses
- The bump seed mechanism guarantees valid address generation
- Always calculate space requirements accurately to avoid overpaying for storage
