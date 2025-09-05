## Prerequisite Reading
- Anchor Framework: Account Attributes
- Solana Account Data Storage
- Rust Vector Types

## Learning Objectives
- Understand Solana account data structure requirements
- Design appropriate field types for voting data
- Implement the Proposal account structure
- Use proper Anchor attributes

## Task
1. Open `programs/voting-program/src/lib.rs`
2. Complete the Proposal account structure:

```rust
#[account]
pub struct Proposal {
    pub title: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
    pub bump: u8,
}
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| #[account] attribute present | Struct is properly annotated |
| Title field | String type for proposal title |
| Options field | Vector of strings for choices |
| Votes field | Vector of u32 for vote counts |
| Bump field | u8 type for PDA bump seed |
| anchor build | Compiles successfully |

## Notes
- `#[account]` attribute is required for Anchor accounts
- `String` type stores variable-length text data
- `Vec<String>` stores multiple option choices
- `Vec<u32>` stores vote counts for each option
- `u8` stores the PDA bump seed for later reference

## Reference Links
- https://www.anchor-lang.com/docs/accounts
- https://docs.rs/anchor-lang/latest/anchor_lang/attr.account.html
