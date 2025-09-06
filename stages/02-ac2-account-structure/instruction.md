Now that your development environment is ready, you'll design the core data
structure for your voting program. In Solana, all data is stored in accounts, so
designing an efficient account structure is crucial.

## Prerequisite Reading

To understand this stage, review these key concepts:

- **Anchor Account Attributes**: Learn how Anchor simplifies account management
  with the `#[account]` attribute. Read the [Account Constraints
  Documentation](https://www.anchor-lang.com/docs/references/account-constraints)
  to understand how accounts are defined and managed.
- **Solana Account Data Storage**: Understand how data is stored and managed in
  Solana accounts. The [Solana Account Model
  Documentation](https://solana.com/docs/core/accounts) explains the
  fundamentals.
- **Rust Vector Types**: Since we'll be storing multiple options and votes,
  review [Rust Vector
  Documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html) to
  understand how dynamic arrays work in Rust.

## Implement the Proposal Account Structure

### 1. Open the Program File

Navigate to `programs/voting-program/src/lib.rs` - this is where you'll define
your program's structure.

### 2. Complete the Proposal Account Structure

Add the following structure to your program:

```rust
#[account]
pub struct Proposal {
    pub title: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
    pub bump: u8,
}
```

## Understanding the Structure

Let's examine each field and why it's necessary:

- **`#[account]` attribute**: This Anchor macro tells the framework that this
  struct represents an on-chain account. It automatically handles
  serialization/deserialization and adds a discriminator.

- **`title: String`**: Stores the proposal title as variable-length text. This
  allows users to create proposals with descriptive titles of varying lengths.

- **`options: Vec<String>`**: A vector (dynamic array) of strings that stores
  the available voting choices. This design allows proposals to have any number
  of options.

- **`votes: Vec<u32>`**: A vector of unsigned 32-bit integers that tracks vote
  counts for each option. The index of each vote count corresponds to the index
  in the options vector.

- **`bump: u8`**: Stores the PDA (Program Derived Address) bump seed. This is
  used for security and reference when interacting with the account later.

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| `#[account]` attribute present | Struct is properly annotated | Ensures Anchor recognizes this as an account |
| Title field | String type for proposal title | Validates proper text storage |
| Options field | Vector of strings for choices | Confirms dynamic option storage |
| Votes field | Vector of u32 for vote counts | Ensures proper vote tracking |
| Bump field | u8 type for PDA bump seed | Validates security feature |
| `anchor build` | Compiles successfully | Confirms syntax and structure are correct |

## Notes

- The `#[account]` attribute is required for Anchor to properly handle account
  serialization and deserialization
- `String` type provides flexible text storage but requires careful space
  management
- `Vec<String>` allows dynamic option lists but increases storage complexity
- `Vec<u32>` efficiently stores vote counts with fixed-size elements
- The `bump` field will be used in later stages for secure account access
