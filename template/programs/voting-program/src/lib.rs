use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod voting_program {
    use super::*;

    pub fn initialize_proposal(
        ctx: Context<InitializeProposal>,
        title: String,
        options: Vec<String>,
    ) -> Result<()> {
        // Students will implement this function
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, option_index: u32) -> Result<()> {
        // Students will implement this function
        Ok(())
    }
}

#[account]
pub struct Proposal {
    // Students will complete this struct
}

#[derive(Accounts)]
pub struct InitializeProposal<'info> {
    // Students will complete this struct
}

#[derive(Accounts)]
pub struct Vote<'info> {
    // Students will complete this struct
}

#[error_code]
pub enum VotingError {
    // Students will define error codes
}
