## Prerequisite Reading
- Anchor Framework: Accounts and Programs
- Solana Account Model Documentation
- Rust Structs and Enums

## Learning Objectives
- Understand Solana's account model
- Design Proposal account structure
- Define program instructions
- Set up basic program skeleton

## Task
1. Open `programs/voting-program/src/lib.rs`
2. Define Proposal account structure:
```rust
#[account]
pub struct Proposal {
    pub title: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
    pub bump: u8,
}
```

3. Define program instructions:
```rust
#[program]
pub mod voting_program {
    use super::*;

    pub fn initialize_proposal(
        ctx: Context<InitializeProposal>,
        title: String,
        options: Vec<String>
    ) -> Result<()> {
        // Implementation coming later
        Ok(())
    }
}
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| File contains #[account] struct | Proposal struct defined |
| File contains #[program] mod | voting_program module defined |
| Struct has required fields | title, options, votes, bump |
| anchor build | Compiles successfully |

## Notes
- Account struct must have #[account] attribute
- Program module must have #[program] attribute
- Use appropriate field types for Solana constraints

## Reference Links
- https://www.anchor-lang.com/docs/accounts
- https://solana.com/docs/core/accounts
