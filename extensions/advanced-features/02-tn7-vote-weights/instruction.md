## Prerequisite Reading
- SPL Token Program
- Anchor Token Accounts
- Token Transfers in Solana

## Learning Objectives
- Implement token-weighted voting
- Handle token accounts
- Calculate vote weights
- Prevent double voting

## Task
1. Add voter record account:
```rust
#[account]
pub struct VoterRecord {
    pub voter: Pubkey,
    pub proposal: Pubkey,
    pub voted: bool,
    pub bump: u8,
}
```

2. Modify vote function for weighted voting:
```rust
pub fn vote(ctx: Context<WeightedVote>, option_index: u32) -> Result<()> {
    let voter_record = &mut ctx.accounts.voter_record;
    let proposal = &mut ctx.accounts.proposal;
    let token_account = &ctx.accounts.voter_token_account;

    require!(!voter_record.voted, VotingError::AlreadyVoted);
    require!(token_account.amount > 0, VotingError::NoTokens);

    let vote_weight = token_account.amount;
    proposal.votes[option_index as usize] += vote_weight;
    voter_record.voted = true;

    Ok(())
}
```

3. Add weighted vote context:
```rust
#[derive(Accounts)]
pub struct WeightedVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init_if_needed,
        payer = voter,
        space = 8 + 32 + 32 + 1 + 1,
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

4. Add new error variants:
```rust
#[error_code]
pub enum VotingError {
    #[msg("Already voted")]
    AlreadyVoted,
    #[msg("No tokens to vote with")]
    NoTokens,
    // ... existing errors
}
```

## Test Cases
| Test | Expected Result |
|------|-----------------|
| Token balance 100 | Adds 100 votes |
| Double voting attempt | Rejects second vote |
| Token account validation | Verifies correct token |
| Voter record creation | Properly tracks voters |

## Notes
- Requires SPL token account
- Uses token amount as vote weight
- Prevents double voting with voter records
- Needs token program in context

## Reference Links
- https://spl.solana.com/token
- https://www.anchor-lang.com/docs/token
