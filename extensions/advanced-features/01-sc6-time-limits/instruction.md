## Prerequisite Reading
- Solana Clock Sysvar
- Anchor Time Handling
- Unix Timestamp Basics

## Learning Objectives
- Add time constraints to voting
- Use Solana's clock sysvar
- Implement deadline enforcement
- Handle time-based errors

## Task
1. Add deadline field to Proposal account:
```rust
#[account]
pub struct Proposal {
    pub title: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
    pub bump: u8,
    pub deadline: i64, // Unix timestamp
}
```

2. Modify initialize_proposal to set deadline:
```rust
pub fn initialize_proposal(
    ctx: Context<InitializeProposal>,
    title: String,
    options: Vec<String>,
    duration_seconds: i64
) -> Result<()> {
    let clock = Clock::get()?;
    let proposal = &mut ctx.accounts.proposal;
    proposal.deadline = clock.unix_timestamp + duration_seconds;
    // ... rest of initialization
}
```

3. Add time check to vote function:
```rust
pub fn vote(ctx: Context<Vote>, option_index: u32) -> Result<()> {
    let clock = Clock::get()?;
    let proposal = &mut ctx.accounts.proposal;

    require!(
        clock.unix_timestamp < proposal.deadline,
        VotingError::VotingPeriodEnded
    );

    // ... rest of voting logic
}
```

4. Add new error variant:
```rust
#[error_code]
pub enum VotingError {
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    // ... existing errors
}
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Before deadline | Allows voting |
| After deadline | Rejects voting |
| Deadline storage | Correctly stores deadline |
| Time calculation | Properly calculates future time |

## Notes
- Use `Clock::get()` to access current time
- Unix timestamp is seconds since epoch
- Ensure proper error handling for time checks

## Reference Links
- https://docs.solana.com/developing/runtime-facilities/sysvars#clock
- https://www.anchor-lang.com/docs/sysvars
