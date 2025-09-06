This extension implements token-weighted voting, where users' voting power is
proportional to their token holdings. This is a common pattern in DAOs
(Decentralized Autonomous Organizations) and governance systems.

## Prerequisite Reading

To understand token-based voting, review:

- **SPL Token Program**: Learn about Solana's standard token program. The [SPL
  Token Documentation](https://www.solana-program.com/docs/token) explains token
  creation, transfers, and account management.
- **Token Integration with Anchor**: Understand how Anchor handles SPL tokens.
  The [Token Integration with Anchor
  documentation](https://www.anchor-lang.com/docs/tokens) explains working with
  token accounts, mints, and token programs in Anchor.
- **SPL Token Basics**: Learn about token account structure and validation.
  Review [SPL Token Basics](https://www.anchor-lang.com/docs/tokens/basics) to
  understand mint, owner, and amount fields.

## Implement Token-Weighted Voting

### 1. Add Voter Record Account

Create a new account structure to track voting:

```rust
#[account]
pub struct VoterRecord {
    pub voter: Pubkey,      // The voter's public key
    pub proposal: Pubkey,   // The proposal being voted on
    pub voted: bool,        // Whether this voter has already voted
    pub bump: u8,           // PDA bump seed
}
```

### 2. Modify Vote Function for Weighted Voting

Update your vote function to handle token-weighted voting:

```rust
pub fn vote(ctx: Context<WeightedVote>, option_index: u32) -> Result<()> {
    let voter_record = &mut ctx.accounts.voter_record;
    let proposal = &mut ctx.accounts.proposal;
    let token_account = &ctx.accounts.voter_token_account;

    // Prevent double voting
    require!(!voter_record.voted, VotingError::AlreadyVoted);

    // Ensure voter has tokens
    require!(token_account.amount > 0, VotingError::NoTokens);

    // Calculate vote weight based on token balance
    let vote_weight = token_account.amount;
    proposal.votes[option_index as usize] += vote_weight;

    // Mark this voter as having voted
    voter_record.voted = true;

    Ok(())
}
```

### 3. Add Weighted Vote Context

Create a new context structure for weighted voting:

```rust
#[derive(Accounts)]
pub struct WeightedVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init_if_needed,
        payer = voter,
        space = 8 + 32 + 32 + 1 + 1, // Discriminator + voter + proposal + voted + bump
        seeds = [b"voter_record", voter.key().as_ref(), proposal.key().as_ref()],
        bump
    )]
    pub voter_record: Account<'info, VoterRecord>,

    #[account(
        constraint = voter_token_account.mint == proposal_vote_token.mint,
        constraint = voter_token_account.owner == voter.key()
    )]
    pub voter_token_account: Account<'info, TokenAccount>,

    pub proposal_vote_token: Account<'info, Mint>,

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}
```

### 4. Add New Error Variants

Add error types for voting validation:

```rust
#[error_code]
pub enum VotingError {
    #[msg("Invalid option index")]
    InvalidOptionIndex,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("Already voted")]
    AlreadyVoted,
    #[msg("No tokens to vote with")]
    NoTokens,
}
```

## Understanding Weighted Voting

Let's examine the key components:

- **VoterRecord**: Tracks which voters have voted on which proposals to prevent
  double voting
- **Token-based Weighting**: Vote power equals token balance (1 token = 1 vote)
- **Account Constraints**: Validates that the token account belongs to the voter
  and uses the correct token
- **init_if_needed**: Creates voter records on-demand when users first vote

## Why Weighted Voting Matters

Token-weighted voting is important for:
1. **Fair Representation**: Users with more stake have more voting power
2. **Sybil Resistance**: Prevents spam voting through economic cost
3. **DAO Governance**: Standard pattern for decentralized decision-making
4. **Economic Alignment**: Voting power aligns with financial interest

## Test Cases

| Test | Expected Result | Purpose |
|------|-----------------|---------|
| Token balance 100 | Adds 100 votes | Validates weighted voting calculation |
| Double voting attempt | Rejects second vote | Confirms anti-double-voting protection |
| Token account validation | Verifies correct token | Ensures proper token validation |
| Voter record creation | Properly tracks voters | Validates voter tracking system |

## Notes

- Requires the SPL token program for token account management
- Uses token amount directly as vote weight (1 token = 1 vote)
- Prevents double voting using persistent voter records
- Validates token account ownership and mint compatibility
- Consider adding minimum token requirements for voting
