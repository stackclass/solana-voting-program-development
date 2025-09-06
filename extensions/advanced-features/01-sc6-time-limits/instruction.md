This extension adds time-based constraints to your voting program, allowing
proposals to have deadlines. This is a common requirement for real-world voting
systems where voting periods need to be limited.

## Prerequisite Reading

To understand time handling in Solana, review:

- **Solana Clock Sysvar**: Learn about the Clock system variable that provides
  time information. Read the [Solana Clock Sysvar
  Documentation](https://docs.anza.xyz/runtime/sysvars#clock) to understand how
  time works on-chain.
- **Unix Timestamp Basics**: Review Unix timestamps (seconds since January 1,
  1970). The [Unix Time Documentation](https://en.wikipedia.org/wiki/Unix_time)
  explains this standard time representation.

## Implement Time-Based Voting Limits

### 1. Add Deadline Field to Proposal Account

Modify your Proposal account structure to include a deadline:

```rust
#[account]
pub struct Proposal {
    pub title: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
    pub bump: u8,
    pub deadline: i64, // Unix timestamp (seconds since epoch)
}
```

### 2. Modify initialize_proposal to Set Deadline

Update your initialization function to accept and set a deadline:

```rust
pub fn initialize_proposal(
    ctx: Context<InitializeProposal>,
    title: String,
    options: Vec<String>,
    duration_seconds: i64
) -> Result<()> {
    let clock = Clock::get()?;
    let proposal = &mut ctx.accounts.proposal;

    // Set deadline as current time + duration
    proposal.deadline = clock.unix_timestamp + duration_seconds;

    proposal.title = title;
    proposal.options = options.clone();
    proposal.votes = vec![0; options.len()];
    proposal.bump = *ctx.bumps.get("proposal").unwrap();

    Ok(())
}
```

### 3. Add Time Check to Vote Function

Modify your vote function to check the deadline:

```rust
pub fn vote(ctx: Context<Vote>, option_index: u32) -> Result<()> {
    let clock = Clock::get()?;
    let proposal = &mut ctx.accounts.proposal;

    // Check if voting period has ended
    require!(
        clock.unix_timestamp < proposal.deadline,
        VotingError::VotingPeriodEnded
    );

    require!(
        (option_index as usize) < proposal.votes.len(),
        VotingError::InvalidOptionIndex
    );

    proposal.votes[option_index as usize] += 1;
    Ok(())
}
```

### 4. Add New Error Variant

Add a new error type for time-based validation failures:

```rust
#[error_code]
pub enum VotingError {
    #[msg("Invalid option index")]
    InvalidOptionIndex,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
}
```

## Understanding Time Implementation

Let's examine the key components:

- **`deadline: i64`**: Stores the voting deadline as a Unix timestamp (seconds since epoch)
- **`Clock::get()?`**: Accesses the Solana clock sysvar to get current blockchain time
- **`clock.unix_timestamp`**: The current time in seconds since Unix epoch
- **Time Validation**: The `require!` check prevents voting after the deadline

## Why Time Limits Matter

Time constraints are important for:
1. **Fairness**: Ensures all votes are cast within the same time period
2. **Finality**: Provides a clear end to voting for result determination
3. **Efficiency**: Prevents indefinite voting that could bloat the system
4. **Real-world Compliance**: Many voting systems have legal time requirements

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Before deadline | Allows voting | Validates time-based access control |
| After deadline | Rejects voting | Confirms deadline enforcement |
| Deadline storage | Correctly stores deadline | Ensures proper time persistence |
| Time calculation | Properly calculates future time | Verifies time arithmetic |

## Notes

- Use `Clock::get()` to access current blockchain time (more reliable than client-side time)
- Unix timestamp uses seconds since January 1, 1970 (UTC)
- Ensure proper error handling for time checks to provide clear user feedback
- Consider timezone implications for user-facing applications
